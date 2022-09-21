use regex::Match;

pub trait ToInt {
    fn to_int(self) -> i32;
    fn to_int_or_null(self) -> Option<i32>;
}

impl<'t> ToInt for Option<Match<'t>> {
    fn to_int(self) -> i32 {
        self.unwrap().as_str().parse::<i32>().unwrap()
    }

    fn to_int_or_null(self) -> Option<i32> {
        self.and_then(|m| m.as_str().parse::<i32>().ok())
    }
}
