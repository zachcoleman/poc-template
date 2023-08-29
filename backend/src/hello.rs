use tonic::{Request, Response, Status};

mod server{
    tonic::include_proto!("hello");
}

pub use server::hello_service_server::{HelloService, HelloServiceServer};
pub use server::{HelloRequest, HelloResponse};

#[derive(Debug, Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService{
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = server::HelloResponse{
            message: format!("Hello {}!", request.into_inner().name).into(),
        };
        Ok(Response::new(reply))
    }
}