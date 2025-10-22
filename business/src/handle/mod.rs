// Handle 包 - 业务处理器模块

pub mod common;
pub mod wallet;
pub mod test;
pub mod user_handle;

// 重新导出常用的 handler
pub use common::{index, health_check};
pub use wallet::{get_wallet_balance};
pub use test::{test_error, test_delete, language_demo};
pub use user_handle::{register, login, change_password, reset_password, get_user_detail};

