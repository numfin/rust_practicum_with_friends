use practicum::server::create_server;
use practicum::todo::TodoList;

#[tokio::main]
async fn main() {
    create_server().await;
}
