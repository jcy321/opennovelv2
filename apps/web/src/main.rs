//! OpenNovel Web Server
//!
//! 主Web服务器入口点

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod state;
mod book_store;
mod agent_collaboration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "opennovel_web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载环境变量
    dotenvy::dotenv().ok();

    // 创建应用状态
    let state = state::AppState::new();

    // 构建路由
    let app = Router::new()
        .merge(routes::build_routes())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🚀 OpenNovel Web Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}