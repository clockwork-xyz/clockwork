use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/event.proto"], &["proto/"])?;
    Ok(())
}
