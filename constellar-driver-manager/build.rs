fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/driver_manager/driver_manager.proto")?;
    Ok(())
}
