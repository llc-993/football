// 钱包相关处理器
use actix_web::{HttpResponse, Responder, web};

use football_common::ApiResponse;
use crate::conf::keys;


/// 查询钱包余额
pub async fn get_wallet_balance(user_id: web::Path<i64>) -> impl Responder {

    HttpResponse::Ok().json(ApiResponse::success_i18n("", keys::SUCCESS))
}

