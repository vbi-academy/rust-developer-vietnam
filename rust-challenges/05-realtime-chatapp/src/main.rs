pub mod features;
pub mod enums;

mod router;
use router::create_router;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    
    // build our application with a route
    let app = create_router();

    println!("🚀 Starting server at http://localhost:3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}