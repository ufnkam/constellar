fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src/grpc")
        .compile(&["src/grpc/driver.proto"], &["."])?;
    Ok(())
}
