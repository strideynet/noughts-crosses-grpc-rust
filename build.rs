fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/game.proto")?;
    built::write_built_file().expect("Failed to acquire build-time information");
    Ok(())
}