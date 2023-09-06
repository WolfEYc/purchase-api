fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().protoc_arg("--experimental_allow_proto3_optional")
        .compile(&[
                "proto/helloworld.proto",
                "proto/accounts.proto"
                ], 
                &[
                "proto/helloworld",
                "proto/accounts"
                ])?;
    Ok(())
}
