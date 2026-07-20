//! `/api/tenant/communication/*`

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::http::{ExtractServices, ServiceHttpError};
use crate::http::middleware::TenantScopeState;
use crate::repositories::communication::{
    Announcement, Message, NewAnnouncement, NewNotification, Notification,
};

pub fn routes() -> Router<TenantScopeState> {
    Router::new()
        .route("/announcements",              get(active).post(broadcast))
        .route("/announcements/class/{cid}",   get(for_class))
        .route("/announcements/{id}",          axum::routing::delete(delete_ann))
        .route("/messages",                    post(send))
        .route("/messages/inbox/{uid}",        get(inbox))
        .route("/messages/unread/{uid}",       get(unread))
        .route("/messages/{id}/read",          post(mark_read_msg))
        .route("/notifications",               post(notify))
        .route("/notifications/user/{uid}",    get(for_user))
        .route("/notifications/{id}/read",     post(mark_read))
        .route("/notifications/user/{uid}/read-all", post(read_all))
}

#[derive(Deserialize)] struct Limit { #[serde(default = "d50")] limit: i64 }
fn d50() -> i64 { 50 }

async fn active(ExtractServices(a): ExtractServices, Query(q): Query<Limit>)
    -> Result<Json<Vec<Announcement>>, ServiceHttpError>
{ Ok(Json(a.repos.announcements.active(q.limit).await.map_err(re)?)) }

async fn broadcast(ExtractServices(a): ExtractServices, Json(b): Json<NewAnnouncement>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let (ann, pushed) = a.communication.broadcast(b).await?;
    Ok(Json(serde_json::json!({ "announcement": ann, "notifications_pushed": pushed })))
}

async fn for_class(ExtractServices(a): ExtractServices, Path(cid): Path<i64>)
    -> Result<Json<Vec<Announcement>>, ServiceHttpError>
{ Ok(Json(a.repos.announcements.for_class(cid).await.map_err(re)?)) }

async fn delete_ann(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.announcements.delete(id).await.map_err(re)?; Ok(axum::http::StatusCode::NO_CONTENT) }

#[derive(Deserialize)]
struct Send { from_user_id: Option<i64>, to_user_id: Option<i64>, subject: Option<String>, body: String }

async fn send(ExtractServices(a): ExtractServices, Json(b): Json<Send>)
    -> Result<Json<Message>, ServiceHttpError>
{
    Ok(Json(a.repos.messages.send(b.from_user_id, b.to_user_id, b.subject.as_deref(), &b.body)
        .await.map_err(re)?))
}

async fn inbox(ExtractServices(a): ExtractServices, Path(uid): Path<i64>, Query(q): Query<Limit>)
    -> Result<Json<Vec<Message>>, ServiceHttpError>
{ Ok(Json(a.repos.messages.inbox(uid, q.limit).await.map_err(re)?)) }

async fn unread(ExtractServices(a): ExtractServices, Path(uid): Path<i64>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = a.repos.messages.unread_count(uid).await.map_err(re)?;
    Ok(Json(serde_json::json!({ "unread": n })))
}

async fn mark_read_msg(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.messages.mark_read(id).await.map_err(re)?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn notify(ExtractServices(a): ExtractServices, Json(b): Json<NewNotification>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = a.communication.notify_user(b).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)] struct UnreadOnly { #[serde(default)] unread_only: bool, #[serde(default = "d50")] limit: i64 }

async fn for_user(ExtractServices(a): ExtractServices, Path(uid): Path<i64>, Query(q): Query<UnreadOnly>)
    -> Result<Json<Vec<Notification>>, ServiceHttpError>
{ Ok(Json(a.repos.notifications.for_user(uid, q.unread_only, q.limit).await.map_err(re)?)) }

async fn mark_read(ExtractServices(a): ExtractServices, Path(id): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.notifications.mark_read(id).await.map_err(re)?; Ok(axum::http::StatusCode::NO_CONTENT) }

async fn read_all(ExtractServices(a): ExtractServices, Path(uid): Path<i64>)
    -> Result<axum::http::StatusCode, ServiceHttpError>
{ a.repos.notifications.mark_all_read(uid).await.map_err(re)?; Ok(axum::http::StatusCode::NO_CONTENT) }

fn re(e: crate::error::RepoError) -> ServiceHttpError { ServiceHttpError(e.into()) }
