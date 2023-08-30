fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
         .build_server(false)
         .out_dir("../client/src")
         .compile(
             &[
                "../proto/hello.proto",
                "../proto/goodbye.proto",
            ],
             &["../proto/", "../proto/google/"],
         )?;
    
    tonic_build::configure()
         .build_server(true)
         .build_transport(false)
         .compile(
             &[
                "../proto/hello.proto",
                "../proto/goodbye.proto",
            ],
             &["../proto/", "../proto/google/"],
         )?;

    Ok(())
 }