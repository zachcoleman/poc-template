use tonic::{Request, Response, Status};

mod server{
    tonic::include_proto!("hello");
}

pub use server::hello_service_server::{HelloService, HelloServiceServer};
pub use server::{HelloRequest, HelloResponse};

#[derive(Debug)]
pub struct MyHelloService {
    pub db_pool: sqlx::SqlitePool,
}

impl MyHelloService {
    pub fn new(db_pool: sqlx::SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl HelloService for MyHelloService{
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        // try to get a connection from the pool
        let try_conn = self.db_pool.acquire().await;
        if let Err(_) = try_conn{
            log::error!("Database connection failed");
            return Err(Status::internal("Database connection failed"));
        }
        let mut conn = try_conn.unwrap();

        // get message content from request
        let msg = request.into_inner().name;

        // try and insert message into database
        let query = sqlx::query(
            r#"INSERT INTO log (message) values (?)"#, 
        )
        .bind(&msg)
        .execute(&mut *conn)
        .await;
        match query{
            Ok(_) => { },
            Err(_) => {
                log::error!("Error inserting message");
                return Err(Status::internal("Error inserting message"));
            }
        }

        // respond with a message
        let reply = server::HelloResponse{
            message: format!("Hello {}!", &msg).into(),
        };
        Ok(Response::new(reply))
    }
}