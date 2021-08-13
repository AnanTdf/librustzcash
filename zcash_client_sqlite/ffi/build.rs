fn main() {
    uniffi_build::generate_scaffolding("./src/zcash_client_sqlite.udl").unwrap();
}
