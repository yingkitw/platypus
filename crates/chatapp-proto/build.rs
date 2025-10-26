fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(
        &[
            "proto/element.proto",
            "proto/forward_msg.proto",
            "proto/back_msg.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
