pub mod hello;
pub mod goodbye;

#[cfg(test)]
mod test_hello{
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

#[cfg(test)]
mod test_goodbye{
    use super::*;

    #[tokio::test]
    async fn goodbye_req() -> Result<(), Box<dyn std::error::Error>>{
        let mut client = goodbye::goodbye_service_client::GoodbyeServiceClient::connect("http://[::1]:50051").await?;
        let request = tonic::Request::new(goodbye::GoodbyeRequest {
            name: "Test".into(),
        });
        let response = client.goodbye(request).await?;
        println!("RESPONSE={:?}", response);
        Ok(())
    }
}