use tonic::{Request, Response, Status};

mod server{
    tonic::include_proto!("goodbye");
}

pub use server::goodbye_service_server::{GoodbyeService, GoodbyeServiceServer};
pub use server::{GoodbyeRequest, GoodbyeResponse};

#[derive(Debug)]
pub struct MyGoodbyeService {
    pub db_pool: sqlx::SqlitePool,
}

impl MyGoodbyeService{
    pub fn new(db_pool: sqlx::SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl GoodbyeService for MyGoodbyeService{
    async fn goodbye(
        &self,
        request: Request<GoodbyeRequest>,
    ) -> Result<Response<GoodbyeResponse>, Status> {
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
            r#"DELETE FROM log where message = ?"#, 
        )
        .bind(&msg)
        .execute(&mut *conn)
        .await;

        match query{
            Ok(_) => { 
                log::info!("Message deleted"); 
            },
            Err(_) => {
                log::error!("Error deleting message");
                return Err(Status::internal("Error deleting message"));
            }
        }

        // respond with a message
        let reply = server::GoodbyeResponse{
            success: true,
        };
        Ok(Response::new(reply))
    }
}