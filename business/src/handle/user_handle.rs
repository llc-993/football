// 用户相关业务处理器
use actix_web::{web, HttpResponse, Responder};
use football_common::{AppError, AppResult, ApiResponse};
use football_orm::user::{users::User, user_level_config::UserLevelConfig};
use rbs::{to_value, value};
use sa_token_core::StpUtil;
use crate::types::*;
use crate::conf::i18n::keys;
use sa_token_macro::{sa_ignore, sa_check_login};
use football_orm::db::get_db;

/// 用户注册
#[sa_ignore]
pub async fn register(req: web::Json<RegisterRequest>) -> impl Responder {


    // 参数校验
    if req.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request_i18n(keys::PASSWORD_REQUIRED));
    }
    
    if req.email.trim().is_empty() && req.phone.as_ref().map_or(true, |p| p.trim().is_empty()) {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request_i18n(keys::EMAIL_OR_PHONE_REQUIRED));
    }

    let rb = get_db();

    // 检查用户名是否已存在
    let existing_user: Option<User> = match User::select_by_username(rb, &req.username).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    };
    if existing_user.is_some() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request_i18n(keys::USER_ALREADY_EXISTS));
    }

    // 检查邮箱是否已存在（如果提供了邮箱）
    if !req.email.trim().is_empty() {
        let existing_email: Option<User> = match User::select_by_email(rb, &req.email).await {
            Ok(email) => email,
            Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
        };
        if existing_email.is_some() {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request_i18n(keys::EMAIL_ALREADY_EXISTS));
        }
    }

    // 密码加密
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    };

    // 创建新用户
    let mut new_user = User::new(req.username.clone(), req.email.clone(), password_hash);
    new_user.phone = req.phone.clone();

    // 插入数据库
    match User::insert(rb, &new_user).await {
        Ok(_) => {},
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }

    HttpResponse::Ok().json(ApiResponse::success())
}

/// 用户登录
#[sa_ignore]
pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    let rb = get_db();

    // 查找用户
    let user: Option<User> = match User::select_by_username(rb, &req.username).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    };

    let user = match user {
        Some(user) => user,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request_i18n(keys::USERNAME_OR_PASSWORD_ERROR)),
    };

    // 验证密码
    let is_valid_password = match verify_password(&req.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };
    if !is_valid_password {
        return HttpResponse::BadRequest().json(ApiResponse::bad_request_i18n(keys::USERNAME_OR_PASSWORD_ERROR));
    }

    // 检查用户状态
    if user.status != 1 {
        return HttpResponse::BadRequest().json(ApiResponse::bad_request_i18n(keys::ACCOUNT_FROZEN_OR_DISABLED));
    }

    // 生成token，这里需要集成sa-token或JWT
    let result = match StpUtil::login(user.id.unwrap()).await {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error_i18n(keys::TOKEN_GENERATION_FAILED)),
    };

    // 更新最后登录时间
    match User::update_last_login_time(user).await {
        Ok(_) => {},
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error_i18n(keys::UPDATE_LOGIN_TIME_FAILED)),
    }

    HttpResponse::Ok().json(ApiResponse::success_i18n(result.as_str(), keys::USER_LOGIN_SUCCESS))
}

/// 修改密码
#[sa_check_login]
pub async fn change_password(req: web::Json<ChangePasswordRequest>,) -> impl Responder {
    let rb = get_db();
    let user_id = match StpUtil::get_login_id_as_long() {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(ApiResponse::<()>::unauthorized_i18n(keys::UNAUTHORIZED)),
    };
    // 查找用户
    let mut user = match User::select_by_id(rb, user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::not_found_i18n(keys::USER_NOT_FOUND)),
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };

    // 验证旧密码
    let is_valid_old_password = match verify_password(&req.old_password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };
    if !is_valid_old_password {
        return HttpResponse::BadRequest().json(ApiResponse::bad_request_i18n(keys::OLD_PASSWORD_ERROR));
    }

    // 更新密码
    let new_password_hash = match hash_password(&req.new_password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };
    user.password_hash = new_password_hash;

    match User::update_by_map(rb, &user, value!{"id":&user.id}).await {
        Ok(_) => {},
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    }

    HttpResponse::Ok().json(ApiResponse::ok_i18n(keys::USER_PASSWORD_CHANGED))
}

/// 找回密码
pub async fn reset_password(req: web::Json<ResetPasswordRequest>) -> impl Responder {
    let rb = get_db();

    // 查找用户
    let mut user = match User::select_by_email(rb, &req.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()>::not_found_i18n(keys::EMAIL_NOT_REGISTERED)),
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };

    // TODO: 验证验证码（需要实现验证码发送和验证逻辑）
    let is_valid_code = match verify_code(&req.email, &req.verification_code).await {
        Ok(valid) => valid,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };
    if !is_valid_code {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request_i18n(keys::VERIFICATION_CODE_ERROR));
    }

    // 更新密码
    let new_password_hash = match hash_password(&req.new_password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };
    user.password_hash = new_password_hash;
    let mut update_map = std::collections::HashMap::new();
    update_map.insert("id", user.id.unwrap());
    match User::update_by_map(rb, &user, to_value!(&update_map)).await {
        Ok(_) => {},
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    }

    HttpResponse::Ok().json(ApiResponse::<()>::ok_i18n(keys::USER_PASSWORD_RESET))
}

/// 获取用户详细信息
#[sa_check_login]
pub async fn get_user_detail() -> impl Responder {
    let rb = get_db();
    let user_id = match StpUtil::get_login_id_as_long() {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(ApiResponse::unauthorized_i18n(keys::UNAUTHORIZED)),
    };
    // 查找用户
    let user = match User::select_by_id(rb, user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::not_found_i18n(keys::USER_NOT_FOUND)),
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };

    // 查找用户等级配置
    let level_config: Option<UserLevelConfig> = match UserLevelConfig::select_by_level_value(rb, user.user_level).await {
        Ok(config) => config,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::error(e.to_string())),
    };

    // 转换为响应对象，过滤敏感字段
    let response = user_to_detail_response(user, level_config);

    HttpResponse::Ok().json(ApiResponse::success_i18n(response, keys::USER_DETAIL_SUCCESS))
}

/// 密码加密（实际应用中应使用bcrypt）
fn hash_password(password: &str) -> AppResult<String> {
    // TODO: 使用bcrypt等加密算法
    // 这里仅作示例，实际应使用：
    // use bcrypt::{hash, DEFAULT_COST};
    // Ok(hash(password, DEFAULT_COST)?)

    Ok(format!("hashed_{}", password))
}

/// 密码验证
fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    // TODO: 使用bcrypt验证
    // use bcrypt::verify;
    // Ok(verify(password, hash)?)

    Ok(hash == format!("hashed_{}", password))
}


/// 验证验证码
async fn verify_code(_email: &str, _code: &str) -> AppResult<bool> {
    // TODO: 实现验证码验证逻辑
    // 可以使用Redis存储验证码，设置过期时间
    Ok(true)
}

