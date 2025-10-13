//! 工具模块
//! 
//! 提供各种实用工具函数和类型

pub mod string_utils;
pub mod time_utils;
pub mod validation;

// 重新导出常用类型和函数
pub use string_utils::{capitalize, truncate, is_empty_or_whitespace};
pub use time_utils::{format_duration, current_timestamp, parse_iso_date};
pub use validation::{validate_email, validate_url, ValidationError};

/// 工具模块的版本信息
pub const UTILS_VERSION: &str = "1.0.0";

/// 通用结果类型
pub type UtilResult<T> = std::result::Result<T, UtilError>;

/// 工具模块错误类型
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub enum UtilError {
    /// 字符串处理错误
    StringError(String),
    /// 时间处理错误
    TimeError(String),
    /// 验证错误
    ValidationError(ValidationError),
    /// 通用错误
    Generic(String),
}

impl std::fmt::Display for UtilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UtilError::StringError(msg) => write!(f, "字符串处理错误: {}", msg),
            UtilError::TimeError(msg) => write!(f, "时间处理错误: {}", msg),
            UtilError::ValidationError(err) => write!(f, "验证错误: {}", err),
            UtilError::Generic(msg) => write!(f, "工具错误: {}", msg),
        }
    }
}

impl std::error::Error for UtilError {}

impl From<ValidationError> for UtilError {
    fn from(err: ValidationError) -> Self {
        UtilError::ValidationError(err)
    }
}

/// 工具模块初始化函数
pub fn init_utils() {
    #[cfg(feature = "logging")]
    log::info!("工具模块已初始化，版本: {}", UTILS_VERSION);
}

/// 获取工具模块信息
pub fn get_utils_info() -> UtilsInfo {
    UtilsInfo {
        version: UTILS_VERSION.to_string(),
        features: get_enabled_features(),
    }
}

/// 工具模块信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub struct UtilsInfo {
    pub version: String,
    pub features: Vec<String>,
}

// 私有辅助函数
fn get_enabled_features() -> Vec<String> {
    let mut features = Vec::new();
    
    // 默认功能
    features.push("core".to_string());
    
    #[cfg(feature = "serde_support")]
    features.push("serde_support".to_string());
    
    #[cfg(feature = "logging")]
    features.push("logging".to_string());
    
    features
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_version() {
        assert_eq!(UTILS_VERSION, "1.0.0");
    }

    #[test]
    fn test_get_utils_info() {
        let info = get_utils_info();
        assert_eq!(info.version, "1.0.0");
        assert!(!info.features.is_empty());
    }

    #[test]
    fn test_util_error_display() {
        let error = UtilError::Generic("测试错误".to_string());
        assert_eq!(error.to_string(), "工具错误: 测试错误");
    }

    #[test]
    fn test_init_utils() {
        // 这个函数主要是日志输出，没有返回值
        init_utils();
    }
}