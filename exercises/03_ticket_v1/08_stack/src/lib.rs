// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), 2); // 16 bits = 2 bytes
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), 4); // 32 bits = 4 bytes
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), 1); // bool is 1 byte
    }
}
