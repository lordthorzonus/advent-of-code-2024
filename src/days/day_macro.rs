#[macro_export]
macro_rules! define_advent_days {
    ($($day:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Day {
            $($day),*
        }

        impl TryFrom<u8> for Day {
            type Error = DayErrors;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        n if n == stringify!($day).chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap() =>
                            Ok(Day::$day),
                    )*
                    x => Err(DayErrors::DayDoesNotExist(x))
                }
            }
        }
    }
}
