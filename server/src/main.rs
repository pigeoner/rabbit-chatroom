use server::server::serve;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    
    serve().await;
}
