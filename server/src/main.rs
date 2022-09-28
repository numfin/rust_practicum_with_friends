use std::io::Result;

use server::logs;

#[tokio::main]
async fn main() -> Result<()> {
    logs::init_logs();
    server::create_server().await?;
    Ok(())
}
