
use crate::application::boot;


pub mod application;
pub mod pincode;
pub mod cipher;


pub mod vault {
    tonic::include_proto!("vault");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = boot().await;
    
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");
    
    Ok(())
}
