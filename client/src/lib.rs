pub mod hello;

#[cfg(test)]
mod test{
    use super::*;

    #[tokio::test]
    async fn hello_req() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = hello::hello_service_client::HelloServiceClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(hello::HelloRequest {
            name: "Test".into(),
        });
        let response = client.hello(request).await?;
        println!("RESPONSE={:?}", response);
        Ok(())
    }
}