// 管理服务处理器
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use football_common::ApiResponse;
use football_orm::repositories::{UserRepository, BaseRepository};

/// 健康检查
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("OK"))
}

/// 服务信息
#[derive(Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: String,
}

pub async fn index() -> impl Responder {
    let info = ServiceInfo {
        name: "Football Manage API".to_string(),
        version: "0.1.0".to_string(),
        status: "running".to_string(),
    };
    HttpResponse::Ok().json(ApiResponse::success(info))
}

/// 用户管理 - 获取所有用户
pub async fn get_users() -> impl Responder {
    let repo = UserRepository::new();
    match repo.find_all().await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// 用户管理 - 根据ID获取用户
pub async fn get_user(user_id: web::Path<i64>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.find_by_id(*user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error("用户不存在".to_string())),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// 用户管理 - 根据用户名获取用户
pub async fn get_user_by_username(username: web::Path<String>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.find_by_username(&username).await {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error("用户不存在".to_string())),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

