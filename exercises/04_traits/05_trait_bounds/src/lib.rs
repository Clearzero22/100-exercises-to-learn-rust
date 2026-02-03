// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

/// Return the minimum of two values.
/// 
/// Trait 约束 限制泛型类型必须实现某些 trait
/// T: PartialOrd T 必须实现 PartialOrd (可比较)
/// PartialOrd 提供比较运算符 <,>,<=,>=
/// where 子句 另一种约束语法，更清晰
/// 
/// 
pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
