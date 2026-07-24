use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;

use crate::http::{ServiceHttpError, TenantScope};
use crate::repositories::communication::{
    Announcement, Message, NewAnnouncement, NewNotification, Notification,
};
use crate::services::perm;

#[derive(Deserialize)] pub struct Limit { #[serde(default = "d50")] limit: i64 }
fn d50() -> i64 { 50 }

pub async fn active(scope: TenantScope, Query(q): Query<Limit>)
    -> Result<Json<Vec<Announcement>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::COMMUNICATION_VIEW, perm::COMMUNICATION_BROADCAST])?;
    Ok(Json(scope.services.repos.announcements.active(q.limit).await?))
}

pub async fn broadcast(scope: TenantScope, Json(b): Json<NewAnnouncement>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let (ann, pushed) = scope.services.communication.broadcast(&scope.ctx, b).await?;
    Ok(Json(serde_json::json!({ "announcement": ann, "notifications_pushed": pushed })))
}

pub async fn for_class(scope: TenantScope, Path((_t, cid)): Path<(String, i64)>)
    -> Result<Json<Vec<Announcement>>, ServiceHttpError>
{
    scope.ctx.require_any(&[perm::COMMUNICATION_VIEW, perm::COMMUNICATION_BROADCAST])?;
    Ok(Json(scope.services.repos.announcements.for_class(cid).await?))
}

pub async fn delete_ann(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{
    scope.ctx.require(perm::COMMUNICATION_BROADCAST)?;
    scope.services.repos.announcements.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct Send { from_user_id: Option<i64>, to_user_id: Option<i64>, subject: Option<String>, body: String }

pub async fn send(scope: TenantScope, Json(b): Json<Send>)
    -> Result<Json<Message>, ServiceHttpError>
{
    Ok(Json(scope.services.repos.messages
        .send(b.from_user_id, b.to_user_id, b.subject.as_deref(), &b.body).await?))
}

pub async fn inbox(scope: TenantScope, Path((_t, uid)): Path<(String, i64)>, Query(q): Query<Limit>)
    -> Result<Json<Vec<Message>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.messages.inbox(uid, q.limit).await?)) }

pub async fn unread(scope: TenantScope, Path((_t, uid)): Path<(String, i64)>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let n = scope.services.repos.messages.unread_count(uid).await?;
    Ok(Json(serde_json::json!({ "unread": n })))
}

pub async fn mark_read_msg(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.repos.messages.mark_read(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn notify(scope: TenantScope, Json(b): Json<NewNotification>)
    -> Result<Json<serde_json::Value>, ServiceHttpError>
{
    let id = scope.services.communication.notify_user(b).await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)] pub struct UnreadOnly { #[serde(default)] unread_only: bool, #[serde(default = "d50")] limit: i64 }

pub async fn for_user(scope: TenantScope, Path((_t, uid)): Path<(String, i64)>, Query(q): Query<UnreadOnly>)
    -> Result<Json<Vec<Notification>>, ServiceHttpError>
{ Ok(Json(scope.services.repos.notifications.for_user(uid, q.unread_only, q.limit).await?)) }

pub async fn mark_read(scope: TenantScope, Path((_t, id)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.repos.notifications.mark_read(id).await?; Ok(StatusCode::NO_CONTENT) }

pub async fn read_all(scope: TenantScope, Path((_t, uid)): Path<(String, i64)>)
    -> Result<StatusCode, ServiceHttpError>
{ scope.services.repos.notifications.mark_all_read(uid).await?; Ok(StatusCode::NO_CONTENT) }
