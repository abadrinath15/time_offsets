pub trait DateOffsetable {
    fn day(&self) -> u16;

    fn month(&self) -> u16;

    fn year(&self) -> u16;

    fn new(&self, day: u16, month: u16, year: u16) -> Self;
}

pub trait DateOffset<T: DateOffsetable + PartialEq> {
    fn add_offset(&self, dt: &T, num_offsets: u8) -> T;
    fn subtract_offset(&self, dt: T, num_offsets: u8) -> T;
    fn on_offset(&self, dt: T) -> bool {
        self.subtract_offset(self.add_offset(&dt, 1), 1) == dt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct SimpleDate {
        day: u16,
        month: u16,
        year: u16,
    }
    impl DateOffsetable for SimpleDate {
        fn day(&self) -> u16 {
            self.day
        }
        fn month(&self) -> u16 {
            self.month
        }
        fn year(&self) -> u16 {
            self.year
        }
        fn new(&self, day: u16, month: u16, year: u16) -> SimpleDate {
            SimpleDate { day, month, year }
        }
    }
    struct SimpleOffset {}

    #[test]
    fn test_add_offset() {}
}
