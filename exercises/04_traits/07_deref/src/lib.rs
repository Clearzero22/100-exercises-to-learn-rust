// TODO: whenever `title` and `description` are returned via their accessor methods, they
//   should be normalized—i.e. leading and trailing whitespace should be removed.
//   There is a method in Rust's standard library that can help with this, but you won't
//   find it in the documentation for `String`.
//   Can you figure out where it is defined and how to use it?

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// 去除字符串的首位空格
impl Ticket {
    pub fn title(&self) -> &str {
        self.title.trim()
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }
}

/// 为什么这样可行?
/// 这展示了方法查找的 Deref 强制转换
/// 1. self.title 是 String
/// 2. 编译器查找 trim() 方法
/// 3.String 本身没有trim()
/// 4.编译器尝试Deref: String -> str
/// 5.str 有trim() 方法
/// 
/// Deref 链
/// String impl Deref<Target=str>
/// str
/// trim


// 方法调用过程

// let s: String = "  hello  ".to_string();
// s.trim();  // 编译器如何找到 trim()？

// // 编译器内部逻辑：
// // 1. 在 String 上找 trim()？ ❌ 没有
// // 2. String 实现了 Deref<Target=str>？ ✅
// // 3. 在 str 上找 trim()？ ✅ 找到了！
// // 4. 自动插入 &*self.title：(&(*self.title)).trim()


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
