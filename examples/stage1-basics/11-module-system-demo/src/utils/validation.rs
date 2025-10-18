//! 数据验证工具

use std::fmt;

/// 验证错误类型
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub enum ValidationError {
    /// 邮箱格式无效
    InvalidEmail(String),
    /// URL格式无效
    InvalidUrl(String),
    /// 长度无效
    InvalidLength {
        field: String,
        actual: usize,
        min: Option<usize>,
        max: Option<usize>,
    },
    /// 格式无效
    InvalidFormat {
        field: String,
        expected: String,
        actual: String,
    },
    /// 值超出范围
    OutOfRange {
        field: String,
        value: String,
        min: Option<String>,
        max: Option<String>,
    },
    /// 必填字段为空
    Required(String),
    /// 自定义验证错误
    Custom(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidEmail(email) => {
                write!(f, "无效的邮箱地址: {}", email)
            }
            ValidationError::InvalidUrl(url) => {
                write!(f, "无效的URL: {}", url)
            }
            ValidationError::InvalidLength { field, actual, min, max } => {
                let mut msg = format!("字段 '{}' 长度无效 (当前: {})", field, actual);
                if let Some(min_val) = min {
                    msg.push_str(&format!(", 最小: {}", min_val));
                }
                if let Some(max_val) = max {
                    msg.push_str(&format!(", 最大: {}", max_val));
                }
                write!(f, "{}", msg)
            }
            ValidationError::InvalidFormat { field, expected, actual } => {
                write!(f, "字段 '{}' 格式无效，期望: {}，实际: {}", field, expected, actual)
            }
            ValidationError::OutOfRange { field, value, min, max } => {
                let mut msg = format!("字段 '{}' 值超出范围 (当前: {})", field, value);
                if let Some(min_val) = min {
                    msg.push_str(&format!(", 最小: {}", min_val));
                }
                if let Some(max_val) = max {
                    msg.push_str(&format!(", 最大: {}", max_val));
                }
                write!(f, "{}", msg)
            }
            ValidationError::Required(field) => {
                write!(f, "必填字段 '{}' 不能为空", field)
            }
            ValidationError::Custom(msg) => {
                write!(f, "验证错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// 验证结果类型
pub type ValidationResult<T> = Result<T, ValidationError>;

/// 验证邮箱地址格式
/// 
/// # Examples
/// 
/// ```
/// use module_system_demo::utils::validate_email;
/// 
/// assert!(validate_email("user@example.com").is_ok());
/// assert!(validate_email("invalid-email").is_err());
/// ```
pub fn validate_email(email: &str) -> ValidationResult<()> {
    if email.is_empty() {
        return Err(ValidationError::Required("email".to_string()));
    }
    
    // 简单的邮箱格式验证
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    
    let local = parts[0];
    let domain = parts[1];
    
    if local.is_empty() || domain.is_empty() {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    
    if !domain.contains('.') {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    
    // 检查是否包含无效字符
    if email.contains(' ') || email.contains('\t') || email.contains('\n') {
        return Err(ValidationError::InvalidEmail(email.to_string()));
    }
    
    Ok(())
}

/// 验证URL格式
/// 
/// # Examples
/// 
/// ```
/// use module_system_demo::utils::validate_url;
/// 
/// assert!(validate_url("https://example.com").is_ok());
/// assert!(validate_url("invalid-url").is_err());
/// ```
pub fn validate_url(url: &str) -> ValidationResult<()> {
    if url.is_empty() {
        return Err(ValidationError::Required("url".to_string()));
    }
    
    // 简单的URL格式验证
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ValidationError::InvalidUrl(url.to_string()));
    }
    
    // 检查是否有域名部分
    let without_protocol = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap();
    
    if without_protocol.is_empty() {
        return Err(ValidationError::InvalidUrl(url.to_string()));
    }
    
    // 检查域名部分是否包含点
    let domain_part = without_protocol.split('/').next().unwrap_or("");
    if !domain_part.contains('.') && domain_part != "localhost" {
        return Err(ValidationError::InvalidUrl(url.to_string()));
    }
    
    Ok(())
}

/// 验证字符串长度
pub fn validate_length(value: &str, field: &str, min: Option<usize>, max: Option<usize>) -> ValidationResult<()> {
    let len = value.len();
    
    if let Some(min_len) = min {
        if len < min_len {
            return Err(ValidationError::InvalidLength {
                field: field.to_string(),
                actual: len,
                min,
                max,
            });
        }
    }
    
    if let Some(max_len) = max {
        if len > max_len {
            return Err(ValidationError::InvalidLength {
                field: field.to_string(),
                actual: len,
                min,
                max,
            });
        }
    }
    
    Ok(())
}

/// 验证数值范围
pub fn validate_range<T>(value: T, field: &str, min: Option<T>, max: Option<T>) -> ValidationResult<()>
where
    T: PartialOrd + fmt::Display + Clone,
{
    if let Some(min_val) = &min {
        if value < *min_val {
            return Err(ValidationError::OutOfRange {
                field: field.to_string(),
                value: value.to_string(),
                min: Some(min_val.to_string()),
                max: max.as_ref().map(|v| v.to_string()),
            });
        }
    }
    
    if let Some(max_val) = &max {
        if value > *max_val {
            return Err(ValidationError::OutOfRange {
                field: field.to_string(),
                value: value.to_string(),
                min: min.as_ref().map(|v| v.to_string()),
                max: Some(max_val.to_string()),
            });
        }
    }
    
    Ok(())
}

/// 验证必填字段
pub fn validate_required(value: &str, field: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        Err(ValidationError::Required(field.to_string()))
    } else {
        Ok(())
    }
}

/// 验证正则表达式匹配
pub fn validate_pattern(value: &str, field: &str, pattern: &str, description: &str) -> ValidationResult<()> {
    // 简单的模式匹配，这里只实现几个常用的
    let matches = match pattern {
        "^[0-9]+$" | r"^\d+$" => value.chars().all(|c| c.is_ascii_digit()),
        "^[a-zA-Z]+$" => value.chars().all(|c| c.is_ascii_alphabetic()),
        "^[a-zA-Z0-9]+$" => value.chars().all(|c| c.is_ascii_alphanumeric()),
        "^[a-zA-Z0-9_]+$" => value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
        _ => true, // 对于复杂的正则表达式，我们暂时返回true
    };
    
    if matches {
        Ok(())
    } else {
        Err(ValidationError::InvalidFormat {
            field: field.to_string(),
            expected: description.to_string(),
            actual: value.to_string(),
        })
    }
}

/// 验证器构建器
#[derive(Debug)]
pub struct Validator {
    errors: Vec<ValidationError>,
}

impl Validator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    /// 添加验证规则
    pub fn validate<F>(&mut self, validation: F) -> &mut Self
    where
        F: FnOnce() -> ValidationResult<()>,
    {
        if let Err(error) = validation() {
            self.errors.push(error);
        }
        self
    }
    
    /// 检查是否有错误
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// 获取所有错误
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }
    
    /// 完成验证，如果有错误则返回第一个错误
    pub fn finish(self) -> ValidationResult<()> {
        if let Some(error) = self.errors.into_iter().next() {
            Err(error)
        } else {
            Ok(())
        }
    }
    
    /// 完成验证，返回所有错误
    pub fn finish_all(self) -> Result<(), Vec<ValidationError>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// 常用验证宏
#[macro_export]
macro_rules! validate {
    ($validator:expr, $validation:expr) => {
        $validator.validate(|| $validation)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.email@domain.co.uk").is_ok());
        
        assert!(validate_email("").is_err());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@domain").is_err());
        assert!(validate_email("user @example.com").is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://localhost").is_ok());
        assert!(validate_url("https://sub.domain.com/path").is_ok());
        
        assert!(validate_url("").is_err());
        assert!(validate_url("invalid-url").is_err());
        assert!(validate_url("ftp://example.com").is_err());
        assert!(validate_url("https://").is_err());
    }

    #[test]
    fn test_validate_length() {
        assert!(validate_length("hello", "test", Some(3), Some(10)).is_ok());
        assert!(validate_length("hi", "test", Some(3), None).is_err());
        assert!(validate_length("very long string", "test", None, Some(10)).is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, "number", Some(1), Some(10)).is_ok());
        assert!(validate_range(0, "number", Some(1), Some(10)).is_err());
        assert!(validate_range(15, "number", Some(1), Some(10)).is_err());
    }

    #[test]
    fn test_validate_required() {
        assert!(validate_required("hello", "field").is_ok());
        assert!(validate_required("", "field").is_err());
        assert!(validate_required("   ", "field").is_err());
    }

    #[test]
    fn test_validate_pattern() {
        assert!(validate_pattern("123", "field", "^[0-9]+$", "数字").is_ok());
        assert!(validate_pattern("abc", "field", "^[a-zA-Z]+$", "字母").is_ok());
        assert!(validate_pattern("abc123", "field", "^[0-9]+$", "数字").is_err());
    }

    #[test]
    fn test_validator() {
        let mut validator = Validator::new();
        
        validator
            .validate(|| validate_email("user@example.com"))
            .validate(|| validate_required("hello", "field"))
            .validate(|| validate_length("test", "field", Some(2), Some(10)));
        
        assert!(!validator.has_errors());
        assert!(validator.finish().is_ok());
    }

    #[test]
    fn test_validator_with_errors() {
        let mut validator = Validator::new();
        
        validator
            .validate(|| validate_email("invalid-email"))
            .validate(|| validate_required("", "field"));
        
        assert!(validator.has_errors());
        assert_eq!(validator.errors().len(), 2);
        assert!(validator.finish().is_err());
    }
}