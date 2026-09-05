/// 账户状态：待激活
pub const USER_STATUS_PENDING: u8 = 0;
/// 账户状态：正常
pub const USER_STATUS_ACTIVE: u8 = 1;
/// 账户状态：已封禁
pub const USER_STATUS_BANNED: u8 = 2;

/// 管理员标记：是（user.is_admin 字段）
pub const USER_IS_ADMIN: u8 = 1;

/// 验证码有效期（秒）
pub const CAPTCHA_EXPIRY: u64 = 300;

/// 密码重置链接有效期（秒）
pub const PASSWORD_RESET_EXPIRY: u64 = 3600;
/// 错误码 → i18n 键（业务错误码映射，各项目自持）
pub fn error_i18n_key(err: &str) -> Option<&'static str> {
    Some(match err {
        "captcha" => "captcha_invalid",
        "incorrect" => "sign_in_incorrect",
        "not_activation" => "sign_in_not_activation",
        "banned" => "sign_in_banned",
        "security" => "sign_in_security_problem",
        "password_weak" => "register_password_weak",
        "password_mismatch" => "register_password_mismatch",
        "exist" => "register_exist",
        _ => return None,
    })
}
