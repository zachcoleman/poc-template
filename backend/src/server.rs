use tonic::{transport::Server};

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = hello::MyHelloService::default();

    Server::builder()
        .add_service(hello::HelloServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}