//! # Environment Demo
//!
//! 本库演示了 Rust 开发环境的基本信息获取和验证功能。
//! 涵盖了版本检查、系统信息、编译器特性等内容。


use std::fmt;

/// 基本环境信息模块
pub mod basic {

    /// 获取 Rust 版本信息
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::basic::get_rust_version;
    ///
    /// let version = get_rust_version();
    /// println!("Rust version: {}", version);
    /// ```
    pub fn get_rust_version() -> String {
        option_env!("CARGO_PKG_RUST_VERSION")
            .unwrap_or("unknown")
            .to_string()
    }

    /// 获取当前操作系统信息
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::basic::get_os_info;
    ///
    /// let os = get_os_info();
    /// println!("Operating System: {}", os);
    /// ```
    pub fn get_os_info() -> String {
        if cfg!(target_os = "windows") {
            "Windows".to_string()
        } else if cfg!(target_os = "macos") {
            "macOS".to_string()
        } else if cfg!(target_os = "linux") {
            "Linux".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// 获取目标架构信息
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::basic::get_target_arch;
    ///
    /// let arch = get_target_arch();
    /// println!("Target Architecture: {}", arch);
    /// ```
    pub fn get_target_arch() -> String {
        if cfg!(target_arch = "x86_64") {
            "x86_64".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "aarch64".to_string()
        } else if cfg!(target_arch = "x86") {
            "x86".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// 检查是否为调试构建
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::basic::is_debug_build;
    ///
    /// if is_debug_build() {
    ///     println!("This is a debug build");
    /// } else {
    ///     println!("This is a release build");
    /// }
    /// ```
    pub fn is_debug_build() -> bool {
        cfg!(debug_assertions)
    }
}

/// 环境变量操作模块
pub mod env_vars {
    use std::env;

    /// 获取环境变量值
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::env_vars::get_env_var;
    ///
    /// match get_env_var("HOME") {
    ///     Some(home) => println!("Home directory: {}", home),
    ///     None => println!("HOME environment variable not set"),
    /// }
    /// ```
    pub fn get_env_var(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// 获取当前工作目录
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::env_vars::get_current_dir;
    ///
    /// match get_current_dir() {
    ///     Ok(dir) => println!("Current directory: {:?}", dir),
    ///     Err(e) => println!("Error getting current directory: {}", e),
    /// }
    /// ```
    pub fn get_current_dir() -> Result<std::path::PathBuf, std::io::Error> {
        env::current_dir()
    }

    /// 获取所有环境变量
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::env_vars::get_all_env_vars;
    ///
    /// let vars = get_all_env_vars();
    /// println!("Found {} environment variables", vars.len());
    /// ```
    pub fn get_all_env_vars() -> Vec<(String, String)> {
        env::vars().collect()
    }

    /// 检查环境变量是否存在
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::env_vars::env_var_exists;
    ///
    /// if env_var_exists("PATH") {
    ///     println!("PATH environment variable is set");
    /// }
    /// ```
    pub fn env_var_exists(key: &str) -> bool {
        env::var(key).is_ok()
    }
}

/// 系统信息结构体
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub rust_version: String,
    pub os: String,
    pub arch: String,
    pub debug_build: bool,
    pub current_dir: Option<std::path::PathBuf>,
}

impl SystemInfo {
    /// 创建新的系统信息实例
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::SystemInfo;
    ///
    /// let info = SystemInfo::new();
    /// println!("{}", info);
    /// ```
    pub fn new() -> Self {
        SystemInfo {
            rust_version: basic::get_rust_version(),
            os: basic::get_os_info(),
            arch: basic::get_target_arch(),
            debug_build: basic::is_debug_build(),
            current_dir: env_vars::get_current_dir().ok(),
        }
    }

    /// 检查环境是否满足最低要求
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::SystemInfo;
    ///
    /// let info = SystemInfo::new();
    /// if info.is_environment_ready() {
    ///     println!("Environment is ready for Rust development");
    /// }
    /// ```
    pub fn is_environment_ready(&self) -> bool {
        // 基本检查：有操作系统信息和架构信息
        !self.os.is_empty() && !self.arch.is_empty() && self.current_dir.is_some()
    }
}

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Rust Development Environment ===")?;
        writeln!(f, "Rust Version: {}", self.rust_version)?;
        writeln!(f, "Operating System: {}", self.os)?;
        writeln!(f, "Architecture: {}", self.arch)?;
        writeln!(f, "Build Type: {}", if self.debug_build { "Debug" } else { "Release" })?;
        if let Some(ref dir) = self.current_dir {
            writeln!(f, "Current Directory: {:?}", dir)?;
        }
        writeln!(f, "Environment Ready: {}", self.is_environment_ready())?;
        Ok(())
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// 编译时信息模块
pub mod compile_time {
    /// 获取编译时间戳
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::compile_time::get_compile_time;
    ///
    /// let compile_time = get_compile_time();
    /// println!("Compiled at: {}", compile_time);
    /// ```
    pub fn get_compile_time() -> String {
        format!("{} {}", env!("CARGO_PKG_VERSION"), "(compile time info)")
    }

    /// 获取包名称
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::compile_time::get_package_name;
    ///
    /// let name = get_package_name();
    /// assert_eq!(name, "environment-demo");
    /// ```
    pub fn get_package_name() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    /// 获取包版本
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::compile_time::get_package_version;
    ///
    /// let version = get_package_version();
    /// println!("Package version: {}", version);
    /// ```
    pub fn get_package_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// 获取包作者信息
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::compile_time::get_package_authors;
    ///
    /// let authors = get_package_authors();
    /// println!("Authors: {}", authors);
    /// ```
    pub fn get_package_authors() -> String {
        env!("CARGO_PKG_AUTHORS").to_string()
    }
}

/// 特性检查模块
pub mod features {
    /// 检查是否支持 64 位指针
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::features::has_64bit_pointers;
    ///
    /// if has_64bit_pointers() {
    ///     println!("This platform supports 64-bit pointers");
    /// }
    /// ```
    pub fn has_64bit_pointers() -> bool {
        cfg!(target_pointer_width = "64")
    }

    /// 检查是否为 Unix 系统
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::features::is_unix;
    ///
    /// if is_unix() {
    ///     println!("Running on a Unix-like system");
    /// }
    /// ```
    pub fn is_unix() -> bool {
        cfg!(unix)
    }

    /// 检查是否为 Windows 系统
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::features::is_windows;
    ///
    /// if is_windows() {
    ///     println!("Running on Windows");
    /// }
    /// ```
    pub fn is_windows() -> bool {
        cfg!(windows)
    }

    /// 获取平台特性摘要
    ///
    /// # 示例
    ///
    /// ```
    /// use environment_demo::features::get_platform_features;
    ///
    /// let features = get_platform_features();
    /// for feature in features {
    ///     println!("Feature: {}", feature);
    /// }
    /// ```
    pub fn get_platform_features() -> Vec<String> {
        let mut features = Vec::new();
        
        if has_64bit_pointers() {
            features.push("64-bit pointers".to_string());
        } else {
            features.push("32-bit pointers".to_string());
        }
        
        if is_unix() {
            features.push("Unix-like system".to_string());
        }
        
        if is_windows() {
            features.push("Windows system".to_string());
        }
        
        if cfg!(debug_assertions) {
            features.push("Debug assertions enabled".to_string());
        } else {
            features.push("Release build".to_string());
        }
        
        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_creation() {
        let info = SystemInfo::new();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
    }

    #[test]
    fn test_environment_ready() {
        let info = SystemInfo::new();
        assert!(info.is_environment_ready());
    }

    #[test]
    fn test_os_detection() {
        let os = basic::get_os_info();
        assert!(os == "Windows" || os == "macOS" || os == "Linux" || os == "Unknown");
    }

    #[test]
    fn test_arch_detection() {
        let arch = basic::get_target_arch();
        assert!(arch == "x86_64" || arch == "aarch64" || arch == "x86" || arch == "Unknown");
    }

    #[test]
    fn test_debug_build_detection() {
        let is_debug = basic::is_debug_build();
        // 在测试中，通常是 debug 构建
        assert!(is_debug);
    }

    #[test]
    fn test_current_dir() {
        let dir = env_vars::get_current_dir();
        assert!(dir.is_ok());
    }

    #[test]
    fn test_env_var_operations() {
        // PATH 环境变量在所有系统上都应该存在
        assert!(env_vars::env_var_exists("PATH"));
        assert!(env_vars::get_env_var("PATH").is_some());
    }

    #[test]
    fn test_all_env_vars() {
        let vars = env_vars::get_all_env_vars();
        assert!(!vars.is_empty());
    }

    #[test]
    fn test_compile_time_info() {
        let name = compile_time::get_package_name();
        assert_eq!(name, "environment-demo");
        
        let version = compile_time::get_package_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_platform_features() {
        let features = features::get_platform_features();
        assert!(!features.is_empty());
        
        // 至少应该有指针宽度信息
        assert!(features.iter().any(|f| f.contains("pointers")));
    }

    #[test]
    fn test_system_info_display() {
        let info = SystemInfo::new();
        let display_str = format!("{}", info);
        assert!(display_str.contains("Rust Development Environment"));
        assert!(display_str.contains("Operating System"));
        assert!(display_str.contains("Architecture"));
    }

    #[test]
    fn test_system_info_default() {
        let info1 = SystemInfo::new();
        let info2 = SystemInfo::default();
        assert_eq!(info1.os, info2.os);
        assert_eq!(info1.arch, info2.arch);
    }
}
