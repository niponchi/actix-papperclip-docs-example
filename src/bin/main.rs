use std::{env, io::Result};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let address = env::var("address").unwrap_or("127.0.0.1:3000".to_string());

    println!("running at: http://{}", address);

    paperclip_docs::server::serve(address).await
}
