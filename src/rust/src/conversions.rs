use apache_avro::types::Value as AvroValue;
use apache_avro::Schema;
use extendr_api::prelude::*;

/// Convert an R object (Robj) to an Avro Value, guided by the Avro schema.
pub fn robj_to_avro(robj: &Robj, schema: &Schema) -> std::result::Result<AvroValue, String> {
    match schema {
        Schema::Null => Ok(AvroValue::Null),
        Schema::Boolean => {
            let val = robj.as_logical()
                .ok_or("Expected logical value for Boolean schema")?;
            Ok(AvroValue::Boolean(val.is_true()))
        }
        Schema::Int => {
            let val = robj.as_integer()
                .ok_or("Expected integer value for Int schema")?;
            Ok(AvroValue::Int(val))
        }
        Schema::Long => {
            let val = robj.as_real()
                .ok_or("Expected numeric value for Long schema")?;
            Ok(AvroValue::Long(val as i64))
        }
        Schema::Float => {
            let val = robj.as_real()
                .ok_or("Expected numeric value for Float schema")?;
            Ok(AvroValue::Float(val as f32))
        }
        Schema::Double => {
            let val = robj.as_real()
                .ok_or("Expected numeric value for Double schema")?;
            Ok(AvroValue::Double(val))
        }
        Schema::String | Schema::Bytes => {
            let val = robj.as_str()
                .ok_or("Expected string value for String schema")?;
            if matches!(schema, Schema::Bytes) {
                Ok(AvroValue::Bytes(val.as_bytes().to_vec()))
            } else {
                Ok(AvroValue::String(val.to_string()))
            }
        }
        Schema::Array(inner) => {
            let list = robj.as_list()
                .ok_or("Expected list for Array schema")?;
            let items: std::result::Result<Vec<AvroValue>, String> = list.iter()
                .map(|(_, item)| robj_to_avro(&item, &inner.items))
                .collect();
            Ok(AvroValue::Array(items?))
        }
        Schema::Map(inner) => {
            let list = robj.as_list()
                .ok_or("Expected named list for Map schema")?;
            let mut map = std::collections::HashMap::new();
            for (key, val) in list.iter() {
                map.insert(key.to_string(), robj_to_avro(&val, &inner.types)?);
            }
            Ok(AvroValue::Map(map))
        }
        Schema::Union(union_schema) => {
            let variants = union_schema.variants();
            if robj.is_null() {
                for (i, variant) in variants.iter().enumerate() {
                    if matches!(variant, Schema::Null) {
                        return Ok(AvroValue::Union(i as u32, Box::new(AvroValue::Null)));
                    }
                }
                return Err("Null not allowed in this union".to_string());
            }
            for (i, variant) in variants.iter().enumerate() {
                if matches!(variant, Schema::Null) {
                    continue;
                }
                if let Ok(val) = robj_to_avro(robj, variant) {
                    return Ok(AvroValue::Union(i as u32, Box::new(val)));
                }
            }
            Err("Could not match R value to any union variant".to_string())
        }
        Schema::Record(record_schema) => {
            let list = robj.as_list()
                .ok_or("Expected named list for Record schema")?;
            let mut fields = Vec::new();
            for field in &record_schema.fields {
                let val = list.iter()
                    .find(|(k, _)| k == &field.name)
                    .map(|(_, v)| v)
                    .ok_or_else(|| format!("Missing field '{}' in R list", field.name))?;
                let avro_val = robj_to_avro(&val, &field.schema)?;
                fields.push((field.name.clone(), avro_val));
            }
            Ok(AvroValue::Record(fields))
        }
        Schema::Enum(enum_schema) => {
            let val = robj.as_str()
                .ok_or("Expected string value for Enum schema")?;
            let idx = enum_schema.symbols.iter()
                .position(|s| s == val)
                .ok_or_else(|| format!("'{}' is not a valid enum symbol", val))?;
            Ok(AvroValue::Enum(idx as u32, val.to_string()))
        }
        Schema::Fixed(fixed_schema) => {
            let val = robj.as_str()
                .ok_or("Expected string (or raw) for Fixed schema")?;
            let bytes = val.as_bytes().to_vec();
            if bytes.len() != fixed_schema.size {
                return Err(format!(
                    "Fixed size mismatch: expected {}, got {}",
                    fixed_schema.size, bytes.len()
                ));
            }
            Ok(AvroValue::Fixed(fixed_schema.size, bytes))
        }
        _ => Err(format!("Unsupported Avro schema type: {:?}", schema)),
    }
}

/// Convert an Avro Value to an R object (Robj).
pub fn avro_to_robj(value: &AvroValue) -> std::result::Result<Robj, String> {
    match value {
        AvroValue::Null => Ok(().into_robj()),
        AvroValue::Boolean(b) => Ok(b.into_robj()),
        AvroValue::Int(i) => Ok(i.into_robj()),
        AvroValue::Long(l) => Ok((*l as f64).into_robj()),
        AvroValue::Float(f) => Ok((*f as f64).into_robj()),
        AvroValue::Double(d) => Ok(d.into_robj()),
        AvroValue::String(s) => Ok(s.into_robj()),
        AvroValue::Bytes(b) => {
            Ok(b.as_slice().into_robj())
        }
        AvroValue::Array(arr) => {
            let items: std::result::Result<Vec<Robj>, String> = arr.iter()
                .map(avro_to_robj)
                .collect();
            let items = items?;
            let list = List::from_values(items);
            Ok(list.into_robj())
        }
        AvroValue::Map(map) => {
            let names: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
            let values: std::result::Result<Vec<Robj>, String> = map.values()
                .map(avro_to_robj)
                .collect();
            let values = values?;
            let list = List::from_names_and_values(names, values)
                .map_err(|e| format!("Failed to create named list: {:?}", e))?;
            Ok(list.into_robj())
        }
        AvroValue::Union(_idx, inner) => avro_to_robj(inner),
        AvroValue::Record(fields) => {
            let names: Vec<&str> = fields.iter().map(|(k, _)| k.as_str()).collect();
            let values: std::result::Result<Vec<Robj>, String> = fields.iter()
                .map(|(_, v)| avro_to_robj(v))
                .collect();
            let values = values?;
            let list = List::from_names_and_values(names, values)
                .map_err(|e| format!("Failed to create named list: {:?}", e))?;
            Ok(list.into_robj())
        }
        AvroValue::Enum(_idx, symbol) => Ok(symbol.into_robj()),
        AvroValue::Fixed(_size, bytes) => {
            Ok(bytes.as_slice().into_robj())
        }
        _ => Err(format!("Unsupported Avro value type: {:?}", value)),
    }
}
