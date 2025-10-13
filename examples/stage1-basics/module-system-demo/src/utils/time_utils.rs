//! 时间处理工具

use super::UtilError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(feature = "serde_support")]
use chrono::{DateTime, Utc};

/// 格式化持续时间为人类可读的字符串
/// 
/// # Examples
/// 
/// ```
/// use std::time::Duration;
/// use module_system_demo::utils::format_duration;
/// 
/// let duration = Duration::from_secs(3661); // 1小时1分1秒
/// assert_eq!(format_duration(duration), "1h 1m 1s");
/// ```
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds == 0 {
        return format!("{}ms", duration.as_millis());
    }
    
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    let mut parts = Vec::new();
    
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 {
        parts.push(format!("{}s", seconds));
    }
    
    if parts.is_empty() {
        format!("{}ms", duration.as_millis())
    } else {
        parts.join(" ")
    }
}

/// 获取当前时间戳（Unix时间戳，秒）
/// 
/// # Examples
/// 
/// ```
/// use module_system_demo::utils::current_timestamp;
/// 
/// let timestamp = current_timestamp();
/// assert!(timestamp > 0);
/// ```
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

/// 获取当前时间戳（毫秒）
pub fn current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis()
}

/// 从Unix时间戳创建SystemTime
pub fn timestamp_to_system_time(timestamp: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(timestamp)
}

/// 计算两个时间点之间的持续时间
pub fn time_diff(start: SystemTime, end: SystemTime) -> Duration {
    end.duration_since(start).unwrap_or(Duration::from_secs(0))
}

/// 检查时间戳是否在指定范围内
pub fn is_timestamp_in_range(timestamp: u64, start: u64, end: u64) -> bool {
    timestamp >= start && timestamp <= end
}

/// 将持续时间转换为不同的单位
#[derive(Debug, Clone, Copy)]
pub struct DurationBreakdown {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub milliseconds: u64,
}

impl DurationBreakdown {
    pub fn from_duration(duration: Duration) -> Self {
        let total_seconds = duration.as_secs();
        let milliseconds = duration.as_millis() as u64 % 1000;
        
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        
        Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
    
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        if self.days > 0 {
            parts.push(format!("{}d", self.days));
        }
        if self.hours > 0 {
            parts.push(format!("{}h", self.hours));
        }
        if self.minutes > 0 {
            parts.push(format!("{}m", self.minutes));
        }
        if self.seconds > 0 {
            parts.push(format!("{}s", self.seconds));
        }
        if self.milliseconds > 0 && parts.is_empty() {
            parts.push(format!("{}ms", self.milliseconds));
        }
        
        if parts.is_empty() {
            "0ms".to_string()
        } else {
            parts.join(" ")
        }
    }
}

/// 简单的性能计时器
#[derive(Debug)]
pub struct Timer {
    start_time: SystemTime,
    name: String,
}

impl Timer {
    /// 创建并启动计时器
    pub fn new(name: &str) -> Self {
        Self {
            start_time: SystemTime::now(),
            name: name.to_string(),
        }
    }
    
    /// 获取已经过的时间
    pub fn elapsed(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.start_time)
            .unwrap_or(Duration::from_secs(0))
    }
    
    /// 重置计时器
    pub fn reset(&mut self) {
        self.start_time = SystemTime::now();
    }
    
    /// 停止计时器并返回持续时间
    pub fn stop(self) -> Duration {
        self.elapsed()
    }
    
    /// 打印经过的时间（如果启用了日志功能）
    pub fn log_elapsed(&self) {
        #[cfg(feature = "logging")]
        log::info!("{}: {}", self.name, format_duration(self.elapsed()));
    }
}

// 当启用chrono支持时的额外功能
#[cfg(feature = "serde_support")]
pub mod chrono_utils {
    use super::*;
    
    /// 解析ISO 8601日期字符串
    /// 
    /// # Examples
    /// 
    /// ```
    /// use module_system_demo::utils::parse_iso_date;
    /// 
    /// let result = parse_iso_date("2023-12-25T10:30:00Z");
    /// assert!(result.is_ok());
    /// ```
    pub fn parse_iso_date(date_str: &str) -> Result<DateTime<Utc>, UtilError> {
        date_str.parse::<DateTime<Utc>>()
            .map_err(|e| UtilError::TimeError(format!("无法解析日期 '{}': {}", date_str, e)))
    }
    
    /// 格式化DateTime为ISO 8601字符串
    pub fn format_iso_date(datetime: DateTime<Utc>) -> String {
        datetime.to_rfc3339()
    }
    
    /// 获取当前UTC时间
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// 从Unix时间戳创建DateTime
    pub fn timestamp_to_datetime(timestamp: i64) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(timestamp, 0)
    }
    
    /// 计算两个日期之间的天数差
    pub fn days_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
        (end.date_naive() - start.date_naive()).num_days()
    }
}

// 重新导出chrono功能（如果可用）
#[cfg(feature = "serde_support")]
pub use chrono_utils::*;

#[cfg(not(feature = "serde_support"))]
pub fn parse_iso_date(_date_str: &str) -> Result<String, UtilError> {
    Err(UtilError::TimeError("需要启用 serde_support 功能来解析ISO日期".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0ms");
        assert_eq!(format_duration(Duration::from_secs(1)), "1s");
        assert_eq!(format_duration(Duration::from_secs(61)), "1m 1s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }

    #[test]
    fn test_current_timestamp() {
        let timestamp = current_timestamp();
        assert!(timestamp > 0);
        
        // 时间戳应该是合理的（大于2020年）
        assert!(timestamp > 1577836800); // 2020-01-01
    }

    #[test]
    fn test_timestamp_conversion() {
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let system_time = timestamp_to_system_time(timestamp);
        let converted_back = system_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert_eq!(converted_back, timestamp);
    }

    #[test]
    fn test_time_diff() {
        let start = SystemTime::now();
        thread::sleep(Duration::from_millis(10));
        let end = SystemTime::now();
        
        let diff = time_diff(start, end);
        assert!(diff.as_millis() >= 10);
    }

    #[test]
    fn test_is_timestamp_in_range() {
        assert!(is_timestamp_in_range(100, 50, 150));
        assert!(!is_timestamp_in_range(200, 50, 150));
        assert!(!is_timestamp_in_range(25, 50, 150));
    }

    #[test]
    fn test_duration_breakdown() {
        let duration = Duration::from_secs(90061); // 1天1小时1分1秒
        let breakdown = DurationBreakdown::from_duration(duration);
        
        assert_eq!(breakdown.days, 1);
        assert_eq!(breakdown.hours, 1);
        assert_eq!(breakdown.minutes, 1);
        assert_eq!(breakdown.seconds, 1);
    }

    #[test]
    fn test_timer() {
        let mut timer = Timer::new("test");
        thread::sleep(Duration::from_millis(10));
        
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);
        
        timer.reset();
        let new_elapsed = timer.elapsed();
        assert!(new_elapsed < elapsed);
    }

    #[cfg(feature = "serde_support")]
    #[test]
    fn test_chrono_utils() {
        use chrono_utils::*;
        
        let now = now_utc();
        let iso_string = format_iso_date(now);
        assert!(!iso_string.is_empty());
        
        let parsed = parse_iso_date(&iso_string);
        assert!(parsed.is_ok());
    }
}