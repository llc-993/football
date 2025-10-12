// 通用处理器
use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use football_common::ApiResponse;
use crate::conf::keys;

/// 健康检查
pub async fn health_check() -> impl Responder {
    // 极简！无需任何参数，自动翻译
    HttpResponse::Ok().json(ApiResponse::success_i18n("OK", keys::SUCCESS))
}

/// 服务信息
#[derive(Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: String,
}

/// 首页 - 服务信息
pub async fn index() -> impl Responder {
    let info = ServiceInfo {
        name: "Football Business API".to_string(),
        version: "0.1.0".to_string(),
        status: "running".to_string(),
    };
    // 极简！无需任何参数
    HttpResponse::Ok().json(ApiResponse::success_i18n(info, keys::SUCCESS))
}

