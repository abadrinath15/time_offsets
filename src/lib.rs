pub trait DateOffsetable {
    fn day(&self) -> u16;

    fn month(&self) -> u16;

    fn year(&self) -> u16;

    fn new(&self, day: u16, month: u16, year: u16) -> Self;
}

pub trait DateOffset {
    fn offset_date(&self, day: u16, month: u16, year: u16) -> (u16, u16, u16);
}

fn add_offset<T: DateOffsetable>(dt: T, offset: impl DateOffset) -> T {
    let (offset_day, offset_month, offset_year) =
        offset.offset_date(dt.day(), dt.month(), dt.year());
    dt.new(offset_day, offset_month, offset_year)
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

    impl DateOffset for SimpleOffset {
        fn offset_date(&self, day: u16, month: u16, year: u16) -> (u16, u16, u16) {
            (day + 1, month, year)
        }
    }

    #[test]
    fn test_add_offset() {
        let dt = SimpleDate {
            day: 1,
            month: 1,
            year: 2001,
        };
        let res = add_offset(dt, SimpleOffset {});
        assert_eq!(
            res,
            SimpleDate {
                day: 2,
                month: 1,
                year: 2001
            }
        )
    }
}
