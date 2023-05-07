fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/health.proto")?;
    tonic_build::compile_protos("proto/noughts_crosses.proto")?;
    tonic_build::compile_protos("proto/user.proto")?;
    built::write_built_file().expect("Failed to acquire build-time information");
    Ok(())
}
