// TODO: Re-implement `Ticket`'s accessor methods. This time return a `&str` rather than a `&String`.

// 核心概念：%String VS &str


/// &String 指向 String 的引用 8字节指针,只能指向 String
/// &str 字符串切片 8字节指针,可指向任何字符串数据
/// 关键点 %str 是更通用的类型
/// 转换规则
/// 
/// let s: String = "hello".to_string();
// let s_ref: &String = &s;      // ✅ String 的引用
// let s_slice: &str = &s;       // ✅ String 可以转换为 &str
// let s_slice2: &str = &s[..];  // ✅ 也可以显式切片
// let s_literal: &str = "world"; // ✅ 字符串字面量就是 &str

/// 为什么返回 &str 更好
/// 1.更灵活-调用者可以使用任何字符串类型
/// 2.更通用-符合Rust官吏
/// 3.零成本-没有性能成本
/// 
/// 

/// 为什么编译器允许 &String -> &str
/// 这叫做 Deref 强制转换 (Deref Coercion) String 实现了 Deref<target=str>，所以 &string 会自动转换 &str
/// &String (ptr, len, cap) -> (ptr, len) 
/// 

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // Some dark magic to verify that you used the expected return types
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
