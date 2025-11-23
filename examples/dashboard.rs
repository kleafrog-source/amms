use axum::{
    routing::get,
    Router,
    response::Html,
    http::StatusCode,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    // Updated for Axum 0.7
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>MMSS Dashboard</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 20px;
                    background-color: #f5f5f5;
                }
                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                    background: white;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                h1 {
                    color: #333;
                    text-align: center;
                }
                .status {
                    padding: 10px;
                    margin: 10px 0;
                    border-radius: 4px;
                    background: #e8f5e9;
                    color: #2e7d32;
                    text-align: center;
                }
            </style>
        </head>
        <body>
            <div class='container'>
                <h1>MMSS Dashboard</h1>
                <div class='status'>Server is running</div>
                <div style="text-align: center;">Visit /health for API status</div>
            </div>
        </body>
        </html>
    "#)
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "{\"status\":\"ok\"}")
}
