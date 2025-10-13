//! 内部模块
//! 
//! 这个模块包含库的内部实现细节，不对外公开

use crate::{LibError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 内部状态管理器
#[derive(Debug)]
pub(crate) struct StateManager {
    state: Arc<Mutex<InternalState>>,
}

/// 内部状态
#[derive(Debug, Default)]
struct InternalState {
    counters: HashMap<String, u64>,
    flags: HashMap<String, bool>,
    data: HashMap<String, String>,
}

impl StateManager {
    /// 创建新的状态管理器
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(InternalState::default())),
        }
    }
    
    /// 增加计数器
    pub(crate) fn increment_counter(&self, key: &str) -> Result<u64> {
        let mut state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        let counter = state.counters.entry(key.to_string()).or_insert(0);
        *counter += 1;
        
        #[cfg(feature = "logging")]
        log::debug!("计数器 '{}' 增加到 {}", key, *counter);
        
        Ok(*counter)
    }
    
    /// 获取计数器值
    pub(crate) fn get_counter(&self, key: &str) -> Result<u64> {
        let state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        Ok(state.counters.get(key).copied().unwrap_or(0))
    }
    
    /// 设置标志
    pub(crate) fn set_flag(&self, key: &str, value: bool) -> Result<()> {
        let mut state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        state.flags.insert(key.to_string(), value);
        
        #[cfg(feature = "logging")]
        log::debug!("标志 '{}' 设置为 {}", key, value);
        
        Ok(())
    }
    
    /// 获取标志值
    pub(crate) fn get_flag(&self, key: &str) -> Result<bool> {
        let state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        Ok(state.flags.get(key).copied().unwrap_or(false))
    }
    
    /// 存储数据
    pub(crate) fn store_data(&self, key: &str, value: &str) -> Result<()> {
        let mut state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        state.data.insert(key.to_string(), value.to_string());
        
        #[cfg(feature = "logging")]
        log::debug!("数据 '{}' 存储", key);
        
        Ok(())
    }
    
    /// 获取数据
    pub(crate) fn get_data(&self, key: &str) -> Result<Option<String>> {
        let state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        Ok(state.data.get(key).cloned())
    }
    
    /// 清除所有数据
    pub(crate) fn _clear(&self) -> Result<()> {
        let mut state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        state.counters.clear();
        state.flags.clear();
        state.data.clear();
        
        #[cfg(feature = "logging")]
        log::debug!("内部状态已清除");
        
        Ok(())
    }
    
    /// 获取状态统计信息
    pub(crate) fn _get_stats(&self) -> Result<_StateStats> {
        let state = self.state.lock()
            .map_err(|e| LibError::Internal(format!("状态锁定失败: {}", e)))?;
        
        Ok(_StateStats {
            counter_count: state.counters.len(),
            flag_count: state.flags.len(),
            data_count: state.data.len(),
            total_counter_value: state.counters.values().sum(),
        })
    }
}

/// 状态统计信息
#[derive(Debug, Clone)]
pub(crate) struct _StateStats {
    pub counter_count: usize,
    pub flag_count: usize,
    pub data_count: usize,
    pub total_counter_value: u64,
}

/// 内部工具函数
pub(crate) mod utils {
    use super::*;
    
    /// 生成唯一ID
    pub(crate) fn _generate_id() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::SystemTime;
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        std::thread::current().id().hash(&mut hasher);
        
        format!("id_{:x}", hasher.finish())
    }
    
    /// 验证内部数据格式
    pub(crate) fn _validate_internal_data(data: &str) -> Result<()> {
        if data.is_empty() {
            return Err(LibError::Internal("内部数据不能为空".to_string()));
        }
        
        if data.len() > 1024 {
            return Err(LibError::Internal("内部数据长度超过限制".to_string()));
        }
        
        // 检查是否包含无效字符
        if data.contains('\0') {
            return Err(LibError::Internal("内部数据包含无效字符".to_string()));
        }
        
        Ok(())
    }
    
    /// 格式化内部错误消息
    pub(crate) fn _format_internal_error(context: &str, error: &str) -> String {
        format!("[内部错误] {}: {}", context, error)
    }
    
    /// 安全地转换字符串
    pub(crate) fn _safe_string_convert(input: &[u8]) -> Result<String> {
        String::from_utf8(input.to_vec())
            .map_err(|e| LibError::Internal(format!("字符串转换失败: {}", e)))
    }
}

/// 内部缓存系统
#[derive(Debug)]
pub(crate) struct _Cache<T> {
    data: Arc<Mutex<HashMap<String, _CacheEntry<T>>>>,
    max_size: usize,
}

#[derive(Debug, Clone)]
struct _CacheEntry<T> {
    value: T,
    created_at: std::time::SystemTime,
    access_count: u64,
}

impl<T: Clone> _Cache<T> {
    /// 创建新的缓存
    pub(crate) fn _new(max_size: usize) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            max_size,
        }
    }
    
    /// 插入缓存项
    pub(crate) fn _insert(&self, key: String, value: T) -> Result<()> {
        let mut data = self.data.lock()
            .map_err(|e| LibError::Internal(format!("缓存锁定失败: {}", e)))?;
        
        // 如果缓存已满，移除最旧的项
        if data.len() >= self.max_size {
            if let Some(oldest_key) = self._find_oldest_key(&data) {
                data.remove(&oldest_key);
            }
        }
        
        let entry = _CacheEntry {
            value,
            created_at: std::time::SystemTime::now(),
            access_count: 0,
        };
        
        data.insert(key, entry);
        Ok(())
    }
    
    /// 获取缓存项
    pub(crate) fn _get(&self, key: &str) -> Result<Option<T>> {
        let mut data = self.data.lock()
            .map_err(|e| LibError::Internal(format!("缓存锁定失败: {}", e)))?;
        
        if let Some(entry) = data.get_mut(key) {
            entry.access_count += 1;
            Ok(Some(entry.value.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// 移除缓存项
    pub(crate) fn _remove(&self, key: &str) -> Result<Option<T>> {
        let mut data = self.data.lock()
            .map_err(|e| LibError::Internal(format!("缓存锁定失败: {}", e)))?;
        
        Ok(data.remove(key).map(|entry| entry.value))
    }
    
    /// 清空缓存
    pub(crate) fn clear(&self) -> Result<()> {
        let mut data = self.data.lock()
            .map_err(|e| LibError::Internal(format!("缓存锁定失败: {}", e)))?;
        
        data.clear();
        Ok(())
    }
    
    /// 获取缓存大小
    pub(crate) fn _size(&self) -> Result<usize> {
        let data = self.data.lock()
            .map_err(|e| LibError::Internal(format!("缓存锁定失败: {}", e)))?;
        
        Ok(data.len())
    }
    
    // 私有辅助方法：找到最旧的键
    fn _find_oldest_key(&self, data: &HashMap<String, _CacheEntry<T>>) -> Option<String> {
        data.iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
    }
}

/// 全局状态管理器实例
static _GLOBAL_STATE: std::sync::OnceLock<StateManager> = std::sync::OnceLock::new();

/// 获取全局状态管理器
pub(crate) fn _get_global_state() -> &'static StateManager {
    _GLOBAL_STATE.get_or_init(|| StateManager::new())
}

/// 初始化内部模块
pub(crate) fn _init_internal() -> Result<()> {
    let state = _get_global_state();
    state.set_flag("initialized", true)?;
    
    #[cfg(feature = "logging")]
    log::debug!("内部模块已初始化");
    
    Ok(())
}

/// 清理内部模块
pub(crate) fn _cleanup_internal() -> Result<()> {
    let state = _get_global_state();
    state._clear()?;
    
    #[cfg(feature = "logging")]
    log::debug!("内部模块已清理");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_manager() {
        let manager = StateManager::new();
        
        // 测试计数器
        assert_eq!(manager.increment_counter("test").unwrap(), 1);
        assert_eq!(manager.increment_counter("test").unwrap(), 2);
        assert_eq!(manager.get_counter("test").unwrap(), 2);
        
        // 测试标志
        manager.set_flag("test_flag", true).unwrap();
        assert!(manager.get_flag("test_flag").unwrap());
        
        // 测试数据存储
        manager.store_data("key", "value").unwrap();
        assert_eq!(manager.get_data("key").unwrap(), Some("value".to_string()));
    }

    #[test]
    fn test_cache() {
        let cache = _Cache::_new(2);
        
        cache._insert("key1".to_string(), "value1".to_string()).unwrap();
        cache._insert("key2".to_string(), "value2".to_string()).unwrap();
        
        assert_eq!(cache._get("key1").unwrap(), Some("value1".to_string()));
        assert_eq!(cache._size().unwrap(), 2);
        
        // 插入第三个项应该移除最旧的
        cache._insert("key3".to_string(), "value3".to_string()).unwrap();
        assert_eq!(cache._size().unwrap(), 2);
    }

    #[test]
    fn test_utils() {
        use utils::*;
        
        // 测试ID生成
        let id1 = _generate_id();
        let id2 = _generate_id();
        assert_ne!(id1, id2);
        assert!(id1.starts_with("id_"));
        
        // 测试数据验证
        assert!(_validate_internal_data("valid data").is_ok());
        assert!(_validate_internal_data("").is_err());
        assert!(_validate_internal_data(&"x".repeat(2000)).is_err());
        
        // 测试错误格式化
        let error_msg = _format_internal_error("test", "error");
        assert!(error_msg.contains("[内部错误]"));
        
        // 测试字符串转换
        assert!(_safe_string_convert(b"hello").is_ok());
        assert!(_safe_string_convert(&[0xFF, 0xFE]).is_err());
    }

    #[test]
    fn test_global_state() {
        let state1 = _get_global_state();
        let state2 = _get_global_state();
        
        // 应该是同一个实例
        assert!(std::ptr::eq(state1, state2));
    }

    #[test]
    fn test_init_cleanup() {
        assert!(_init_internal().is_ok());
        
        let state = _get_global_state();
        assert!(state.get_flag("initialized").unwrap());
        
        assert!(_cleanup_internal().is_ok());
    }
}