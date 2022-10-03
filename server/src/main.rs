use server::logs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logs::init_logs();
    server::create_server().await?;
    Ok(())
}
