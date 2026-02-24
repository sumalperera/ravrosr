use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().expect("Failed to create Tokio runtime")
});
