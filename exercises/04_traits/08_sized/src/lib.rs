/// DST (Dynamically Sized Types 动态大小类型)
/// 问题
/// 
// 什么是 DST？
// 类型	大小	是否 DST
// str	未知（取决于字符串长度）	✅ DST
// [T]	未知（取决于数组长度）	✅ DST
// String	24 字节（ptr + len + cap）	❌ 不是 DST
// &str	8 字节（指针 + 长度）	❌ 不是 DST
// [u8; 10]	10 字节	❌ 不是 DST
// &[u8]	16 字节（指针 + 长度）	❌ 不是 DST
// 关键点：引用/智能指针包含 DST 的大小信息，所以它们是 Sized 的。

pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>();

}
