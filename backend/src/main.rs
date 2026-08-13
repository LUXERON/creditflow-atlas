use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::{
    env,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Scenario {
    role: String,
    loan_volume: i64,
    monthly_fee: f64,
    completion_lift: i64,
    partner_contribution: i64,
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status":"healthy","service":"creditflow-atlas"}))
}

async fn create_scenario(
    State(state): State<AppState>,
    Json(s): Json<Scenario>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if s.loan_volume < 0 || s.monthly_fee < 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "values must be non-negative".into(),
        ));
    }
    let db = state.db.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database unavailable".into(),
        )
    })?;
    db.execute("INSERT INTO scenarios(role,loan_volume,monthly_fee,completion_lift,partner_contribution) VALUES(?1,?2,?3,?4,?5)",params![s.role,s.loan_volume,s.monthly_fee,s.completion_lift,s.partner_contribution]).map_err(internal)?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"saved":true,"id":db.last_insert_rowid()})),
    ))
}

fn internal(e: rusqlite::Error) -> (StatusCode, String) {
    tracing::error!(error=%e,"database error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "database operation failed".into(),
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "creditflow_atlas=info,tower_http=info".into()),
        )
        .init();
    let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "data/creditflow.db".into());
    if let Some(parent) = PathBuf::from(&db_path).parent() {
        std::fs::create_dir_all(parent).expect("create database directory");
    }
    let db = Connection::open(db_path).expect("open sqlite database");
    db.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE IF NOT EXISTS scenarios(id INTEGER PRIMARY KEY, role TEXT NOT NULL, loan_volume INTEGER NOT NULL, monthly_fee REAL NOT NULL, completion_lift INTEGER NOT NULL, partner_contribution INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);").expect("migrate database");
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };
    let static_dir = env::var("STATIC_DIR").unwrap_or_else(|_| "../frontend/dist".into());
    let index = PathBuf::from(&static_dir).join("index.html");
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/scenarios", post(create_scenario))
        .fallback_service(ServeDir::new(&static_dir).not_found_service(ServeFile::new(index)))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr,"CreditFlow Atlas listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind server");
    axum::serve(listener, app).await.expect("serve app");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scenario_serializes() {
        let s = Scenario {
            role: "bank".into(),
            loan_volume: 1000,
            monthly_fee: 2.0,
            completion_lift: 10,
            partner_contribution: 5,
        };
        assert!(serde_json::to_string(&s).unwrap().contains("bank"));
    }
}
