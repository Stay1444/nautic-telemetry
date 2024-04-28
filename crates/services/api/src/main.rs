use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:5000").await?;

    let app = Router::new().into_make_service();

    axum::serve(listener, app).await?;

    Ok(())
}
