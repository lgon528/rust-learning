//! 序列化支持模块
//!
//! 这个模块提供了序列化和反序列化的功能，仅在启用 `serde_support` 功能时可用。

#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde_support")]
use crate::LibError;

/// 序列化格式枚举
#[cfg(feature = "serde_support")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    /// JSON 格式
    Json,
    /// TOML 格式
    Toml,
}

/// 序列化特征
#[cfg(feature = "serde_support")]
pub trait Serializable: Serialize + for<'de> Deserialize<'de> {
    /// 序列化为字符串
    fn serialize_to_string(&self, format: SerializationFormat) -> Result<String, LibError> {
        match format {
            SerializationFormat::Json => {
                serde_json::to_string_pretty(self)
                    .map_err(|e| LibError::SerializationError(e.to_string()))
            }
            SerializationFormat::Toml => {
                toml::to_string_pretty(self)
                    .map_err(|e| LibError::SerializationError(e.to_string()))
            }
        }
    }

    /// 从字符串反序列化
    fn deserialize_from_string(data: &str, format: SerializationFormat) -> Result<Self, LibError>
    where
        Self: Sized,
    {
        match format {
            SerializationFormat::Json => {
                serde_json::from_str(data)
                    .map_err(|e| LibError::SerializationError(e.to_string()))
            }
            SerializationFormat::Toml => {
                toml::from_str(data)
                    .map_err(|e| LibError::SerializationError(e.to_string()))
            }
        }
    }
}

/// 为支持序列化的类型自动实现 Serializable 特征
#[cfg(feature = "serde_support")]
impl<T> Serializable for T where T: Serialize + for<'de> Deserialize<'de> {}

/// 序列化工具函数
#[cfg(feature = "serde_support")]
pub mod utils {
    use super::*;

    /// 将对象序列化为 JSON 字符串
    pub fn to_json<T: Serialize>(value: &T) -> Result<String, LibError> {
        serde_json::to_string_pretty(value)
            .map_err(|e| LibError::SerializationError(e.to_string()))
    }

    /// 从 JSON 字符串反序列化对象
    pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T, LibError> {
        serde_json::from_str(json)
            .map_err(|e| LibError::SerializationError(e.to_string()))
    }

    /// 将对象序列化为 TOML 字符串
    pub fn to_toml<T: Serialize>(value: &T) -> Result<String, LibError> {
        toml::to_string_pretty(value)
            .map_err(|e| LibError::SerializationError(e.to_string()))
    }

    /// 从 TOML 字符串反序列化对象
    pub fn from_toml<T: for<'de> Deserialize<'de>>(toml_str: &str) -> Result<T, LibError> {
        toml::from_str(toml_str)
            .map_err(|e| LibError::SerializationError(e.to_string()))
    }

    /// 检测字符串格式
    pub fn detect_format(data: &str) -> Option<SerializationFormat> {
        let trimmed = data.trim();
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            Some(SerializationFormat::Json)
        } else if trimmed.contains('=') || trimmed.starts_with('[') {
            Some(SerializationFormat::Toml)
        } else {
            None
        }
    }

    /// 自动检测格式并反序列化
    pub fn auto_deserialize<T: for<'de> Deserialize<'de>>(data: &str) -> Result<T, LibError> {
        match detect_format(data) {
            Some(SerializationFormat::Json) => from_json(data),
            Some(SerializationFormat::Toml) => from_toml(data),
            None => Err(LibError::SerializationError(
                "无法检测数据格式".to_string(),
            )),
        }
    }
}

#[cfg(all(test, feature = "serde_support"))]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        name: String,
        value: i32,
        active: bool,
    }

    #[test]
    fn test_json_serialization() {
        let data = TestData {
            name: "测试".to_string(),
            value: 42,
            active: true,
        };

        let json = utils::to_json(&data).unwrap();
        assert!(json.contains("测试"));
        assert!(json.contains("42"));
        assert!(json.contains("true"));

        let deserialized: TestData = utils::from_json(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_toml_serialization() {
        let data = TestData {
            name: "测试".to_string(),
            value: 42,
            active: true,
        };

        let toml_str = utils::to_toml(&data).unwrap();
        assert!(toml_str.contains("测试"));
        assert!(toml_str.contains("42"));
        assert!(toml_str.contains("true"));

        let deserialized: TestData = utils::from_toml(&toml_str).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_serializable_trait() {
        let data = TestData {
            name: "特征测试".to_string(),
            value: 100,
            active: false,
        };

        // 测试 JSON 序列化
        let json = data.serialize_to_string(SerializationFormat::Json).unwrap();
        let from_json = TestData::deserialize_from_string(&json, SerializationFormat::Json).unwrap();
        assert_eq!(data, from_json);

        // 测试 TOML 序列化
        let toml_str = data.serialize_to_string(SerializationFormat::Toml).unwrap();
        let from_toml = TestData::deserialize_from_string(&toml_str, SerializationFormat::Toml).unwrap();
        assert_eq!(data, from_toml);
    }

    #[test]
    fn test_format_detection() {
        let json_data = r#"{"name": "test"}
"#;
        let toml_data = "name = \"test\"";
        let invalid_data = "just some text";

        assert_eq!(utils::detect_format(json_data), Some(SerializationFormat::Json));
        assert_eq!(utils::detect_format(toml_data), Some(SerializationFormat::Toml));
        assert_eq!(utils::detect_format(invalid_data), None);
    }

    #[test]
    fn test_auto_deserialize() {
        let data = TestData {
            name: "自动测试".to_string(),
            value: 200,
            active: true,
        };

        // 测试 JSON 自动反序列化
        let json = utils::to_json(&data).unwrap();
        let from_auto: TestData = utils::auto_deserialize(&json).unwrap();
        assert_eq!(data, from_auto);

        // 测试 TOML 自动反序列化
        let toml_str = utils::to_toml(&data).unwrap();
        let from_auto: TestData = utils::auto_deserialize(&toml_str).unwrap();
        assert_eq!(data, from_auto);
    }

    #[test]
    fn test_serialization_errors() {
        let invalid_json = "{invalid json}";
        let result: Result<TestData, LibError> = utils::from_json(invalid_json);
        assert!(result.is_err());
        
        if let Err(LibError::SerializationError(msg)) = result {
            // 检查错误消息包含常见的 JSON 解析错误关键词
            assert!(msg.contains("expected") || msg.contains("invalid") || 
                   msg.contains("EOF") || msg.contains("character") ||
                   msg.contains("parse") || msg.contains("syntax") ||
                   msg.contains("key") || msg.contains("value") ||
                   msg.contains("must") || msg.contains("string"));
        } else {
            panic!("期望序列化错误");
        }
    }
}

// 当没有启用 serde_support 功能时的占位符
#[cfg(not(feature = "serde_support"))]
pub mod placeholder {
    //! 当未启用 serde_support 功能时，此模块提供占位符
    
    /// 占位符函数，提示需要启用 serde_support 功能
    pub fn serialization_not_available() -> &'static str {
        "序列化功能需要启用 'serde_support' 功能标志"
    }
}