use std::env;

use tonic::{transport::Server};
use sqlx::sqlite::SqlitePool;

mod hello;
mod goodbye;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load dotenv file
    match dotenv::dotenv(){
        Ok(_) => log::info!("Dotenv file loaded"),
        Err(_) => log::error!("Dotenv file not found"),
    } 

    // get database connection pool
    let conn = SqlitePool::connect(
        &env::var("DATABASE_URL").unwrap()
    )
    .await;
    if let Err(_) = conn{
        log::error!("Database connection failed");
        return Err("Database connection failed".into());
    }
    let db_pool = conn.unwrap();

    // Define the gRPC services 
    let hello_service= hello::MyHelloService::new(db_pool.clone());
    let goodbye_service = goodbye::MyGoodbyeService::new(db_pool.clone());

    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(hello::HelloServiceServer::new(hello_service))
        .add_service(goodbye::GoodbyeServiceServer::new(goodbye_service))
        .serve(addr)
        .await?;

    Ok(())
}