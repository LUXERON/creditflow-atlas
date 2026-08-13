use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
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

#[derive(Debug, Deserialize, Serialize)]
struct WorkItem {
    id: Option<i64>,
    role: String,
    kind: String,
    title: String,
    status: String,
    priority: String,
    owner: String,
    counterparty: String,
    amount: f64,
    #[serde(default)]
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct StatusUpdate {
    status: String,
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

async fn list_work_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<WorkItem>>, (StatusCode, String)> {
    let db = state.db.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database unavailable".into(),
        )
    })?;
    let mut stmt=db.prepare("SELECT id,role,kind,title,status,priority,owner,counterparty,amount,created_at FROM work_items ORDER BY id DESC").map_err(internal)?;
    let rows = stmt
        .query_map([], |r| {
            Ok(WorkItem {
                id: Some(r.get(0)?),
                role: r.get(1)?,
                kind: r.get(2)?,
                title: r.get(3)?,
                status: r.get(4)?,
                priority: r.get(5)?,
                owner: r.get(6)?,
                counterparty: r.get(7)?,
                amount: r.get(8)?,
                created_at: r.get(9)?,
            })
        })
        .map_err(internal)?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(internal)?);
    }
    Ok(Json(items))
}

async fn create_work_item(
    State(state): State<AppState>,
    Json(item): Json<WorkItem>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if item.title.trim().is_empty()
        || !["low", "medium", "high", "critical"].contains(&item.priority.as_str())
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "valid title and priority required".into(),
        ));
    }
    let db = state.db.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database unavailable".into(),
        )
    })?;
    db.execute("INSERT INTO work_items(role,kind,title,status,priority,owner,counterparty,amount) VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",params![item.role,item.kind,item.title,item.status,item.priority,item.owner,item.counterparty,item.amount]).map_err(internal)?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"created":true,"id":db.last_insert_rowid()})),
    ))
}

async fn update_work_item(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(change): Json<StatusUpdate>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !["open", "in_review", "approved", "complete"].contains(&change.status.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "invalid workflow status".into()));
    }
    let db = state.db.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database unavailable".into(),
        )
    })?;
    let changed = db
        .execute(
            "UPDATE work_items SET status=?1 WHERE id=?2",
            params![change.status, id],
        )
        .map_err(internal)?;
    if changed == 0 {
        return Err((StatusCode::NOT_FOUND, "work item not found".into()));
    }
    Ok(Json(serde_json::json!({"updated":true,"id":id})))
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
    db.execute_batch("PRAGMA journal_mode=WAL;
      CREATE TABLE IF NOT EXISTS scenarios(id INTEGER PRIMARY KEY, role TEXT NOT NULL, loan_volume INTEGER NOT NULL, monthly_fee REAL NOT NULL, completion_lift INTEGER NOT NULL, partner_contribution INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);
      CREATE TABLE IF NOT EXISTS work_items(id INTEGER PRIMARY KEY, role TEXT NOT NULL, kind TEXT NOT NULL, title TEXT NOT NULL, status TEXT NOT NULL, priority TEXT NOT NULL, owner TEXT NOT NULL, counterparty TEXT NOT NULL, amount REAL NOT NULL DEFAULT 0, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);
      INSERT INTO work_items(role,kind,title,status,priority,owner,counterparty,amount)
      SELECT 'bank','Product approval','Approve pilot milestone and disclosure pack','in_review','high','Bank executive','Risk committee',250000 WHERE NOT EXISTS(SELECT 1 FROM work_items);
      INSERT INTO work_items(role,kind,title,status,priority,owner,counterparty,amount)
      SELECT 'airline','Inventory release','Reserve 2.4m miles for completion cohorts','open','medium','Airline loyalty','Platform settlement',42000 WHERE (SELECT COUNT(*) FROM work_items)=1;
      INSERT INTO work_items(role,kind,title,status,priority,owner,counterparty,amount)
      SELECT 'risk','Fairness review','Review early-exit parity before cohort expansion','open','critical','Risk & compliance','Bank product',0 WHERE (SELECT COUNT(*) FROM work_items)=2;").expect("migrate database");
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };
    let static_dir = env::var("STATIC_DIR").unwrap_or_else(|_| "../frontend/dist".into());
    let index = PathBuf::from(&static_dir).join("index.html");
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/scenarios", post(create_scenario))
        .route(
            "/api/work-items",
            get(list_work_items).post(create_work_item),
        )
        .route("/api/work-items/{id}", patch(update_work_item))
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
