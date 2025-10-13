//! 字符串处理工具

use super::UtilError;

/// 将字符串首字母大写
/// 
/// # Examples
/// 
/// ```
/// use module_system_demo::utils::capitalize;
/// 
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize("WORLD"), "WORLD");
/// assert_eq!(capitalize(""), "");
/// ```
pub fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// 截断字符串到指定长度
/// 
/// # Examples
/// 
/// ```
/// use module_system_demo::utils::truncate;
/// 
/// assert_eq!(truncate("Hello, World!", 5), "Hello");
/// assert_eq!(truncate("Hi", 10), "Hi");
/// ```
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len).collect()
    }
}

/// 检查字符串是否为空或只包含空白字符
/// 
/// # Examples
/// 
/// ```
/// use module_system_demo::utils::is_empty_or_whitespace;
/// 
/// assert!(is_empty_or_whitespace(""));
/// assert!(is_empty_or_whitespace("   "));
/// assert!(is_empty_or_whitespace("\t\n"));
/// assert!(!is_empty_or_whitespace("hello"));
/// ```
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

/// 移除字符串中的所有空白字符
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// 反转字符串
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// 计算字符串中单词的数量
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// 将字符串转换为蛇形命名法 (snake_case)
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            // 添加下划线的条件：
            // 1. 不是第一个字符
            // 2. 前一个字符是小写字母（小写转大写的边界）
            // 3. 当前是大写字母，前一个也是大写，但下一个是小写
            //    这表示从连续大写字母转到新单词（如 XMLHttp 中的 H）
            if i > 0 {
                let prev_char = chars[i-1];
                let next_char = if i + 1 < chars.len() { Some(chars[i+1]) } else { None };
                
                let should_add_underscore = 
                    // 从小写转到大写
                    prev_char.is_lowercase() ||
                    // 从连续大写字母转到新单词开始（如 XMLHttp 中的 H）
                    (prev_char.is_uppercase() && 
                     next_char.map_or(false, |nc| nc.is_lowercase()));
                
                if should_add_underscore {
                    result.push('_');
                }
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    
    result
}

/// 将字符串转换为驼峰命名法 (camelCase)
pub fn to_camel_case(s: &str) -> String {
    let words: Vec<&str> = s.split('_').collect();
    if words.is_empty() {
        return String::new();
    }
    
    let mut result = words[0].to_lowercase();
    for word in &words[1..] {
        result.push_str(&capitalize(word));
    }
    
    result
}

/// 安全地解析字符串为数字
pub fn safe_parse_number<T>(s: &str) -> Result<T, UtilError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    s.trim().parse().map_err(|e| {
        UtilError::StringError(format!("无法解析数字 '{}': {}", s, e))
    })
}

/// 字符串相似度计算（简单的编辑距离）
pub fn string_similarity(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }
    
    let len1 = s1.len();
    let len2 = s2.len();
    
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }
    
    let max_len = len1.max(len2);
    let distance = levenshtein_distance(s1, s2);
    
    1.0 - (distance as f64 / max_len as f64)
}

// 私有辅助函数：计算编辑距离
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    // 初始化第一行和第一列
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    // 填充矩阵
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    
    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize("WORLD"), "WORLD");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("a"), "A");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello, World!", 5), "Hello");
        assert_eq!(truncate("Hi", 10), "Hi");
        assert_eq!(truncate("Test", 4), "Test");
        assert_eq!(truncate("Test", 0), "");
    }

    #[test]
    fn test_is_empty_or_whitespace() {
        assert!(is_empty_or_whitespace(""));
        assert!(is_empty_or_whitespace("   "));
        assert!(is_empty_or_whitespace("\t\n"));
        assert!(!is_empty_or_whitespace("hello"));
        assert!(!is_empty_or_whitespace(" hello "));
    }

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(remove_whitespace("hello world"), "helloworld");
        assert_eq!(remove_whitespace("  a  b  c  "), "abc");
        assert_eq!(remove_whitespace(""), "");
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("  one   two   three  "), 3);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("single"), 1);
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        // XMLHttpRequest -> xml_http_request (XML作为一个单词，Http作为一个单词，Request作为一个单词)
        assert_eq!(to_snake_case("XMLHttpRequest"), "xml_http_request");
        assert_eq!(to_snake_case("simple"), "simple");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("simple"), "simple");
        assert_eq!(to_camel_case("one_two_three"), "oneTwoThree");
    }

    #[test]
    fn test_safe_parse_number() {
        assert_eq!(safe_parse_number::<i32>("123").unwrap(), 123);
        assert_eq!(safe_parse_number::<f64>("3.14").unwrap(), 3.14);
        assert!(safe_parse_number::<i32>("abc").is_err());
    }

    #[test]
    fn test_string_similarity() {
        assert_eq!(string_similarity("hello", "hello"), 1.0);
        assert_eq!(string_similarity("", ""), 1.0);
        assert_eq!(string_similarity("hello", ""), 0.0);
        assert!(string_similarity("hello", "hallo") > 0.5);
    }
}