// 业务处理器
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

pub async fn index() -> impl Responder {
    let info = ServiceInfo {
        name: "Football Business API".to_string(),
        version: "0.1.0".to_string(),
        status: "running".to_string(),
    };
    // 极简！无需任何参数
    HttpResponse::Ok().json(ApiResponse::success_i18n(info, keys::SUCCESS))
}

/// 测试多语言错误响应
pub async fn test_error() -> impl Responder {
    // 极简！一行搞定
    HttpResponse::NotFound().json(ApiResponse::not_found_i18n(keys::USER_NOT_FOUND))
}

/// 测试多语言成功响应（无数据）
pub async fn test_delete() -> impl Responder {
    // 无数据成功响应
    HttpResponse::Ok().json(ApiResponse::<()>::ok_i18n(keys::USER_DELETED))
}

/// 演示：根据语言返回不同消息
#[derive(Serialize)]
pub struct LanguageDemo {
    pub detected_language: String,
    pub message: String,
}

pub async fn language_demo() -> impl Responder {
    use crate::conf::i18n;
    
    let lang = i18n::get_current_language();
    let demo = LanguageDemo {
        detected_language: format!("{:?}", lang),
        message: i18n::translate(keys::SUCCESS),
    };
    
    HttpResponse::Ok().json(ApiResponse::success_i18n(demo, keys::SUCCESS))
}

