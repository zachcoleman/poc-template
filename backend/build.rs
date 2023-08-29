fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
         .build_server(false)
         .out_dir("../client/src")
         .compile(
             &["../proto/hello.proto"],
             &["../proto/", "../proto/googleapis/"],
         )?;
    
    tonic_build::configure()
         .build_server(true)
        //  .out_dir("./src")
         .build_transport(false)
         .compile(
             &["../proto/hello.proto"],
             &["../proto/", "../proto/googleapis/"],
         )?;

    Ok(())
 }