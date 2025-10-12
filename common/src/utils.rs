use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// 获取当前UTC时间
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// 生成唯一ID（基于时间戳和计数器）
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}{:04}", timestamp, counter % 10000)
}

/// 生成简单的 UUID（使用时间戳）
pub fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
        println!("ID1: {}, ID2: {}", id1, id2);
    }

    #[test]
    fn test_now_utc() {
        let now = now_utc();
        assert!(now.timestamp() > 0);
    }

    #[test]
    fn test_generate_uuid() {
        let uuid1 = generate_uuid();
        let uuid2 = generate_uuid();
        assert_ne!(uuid1, uuid2);
    }
}

