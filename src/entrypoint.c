// We need to forward routine registration from C to Rust
// to avoid the linker removing the bytes of the Rust routine
// registration table when merging object files into a shared library.

void R_init_ravrosr_extendr(void *dll);

void R_init_ravrosr(void *dll) {
    R_init_ravrosr_extendr(dll);
}
