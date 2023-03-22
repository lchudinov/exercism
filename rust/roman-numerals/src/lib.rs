use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.num >= 1000 {
            write!(f, "M{}", Roman::from(self.num - 1000))
        } else if self.num >= 900 {
            write!(f, "CM{}", Roman::from(self.num - 900))
        } else if self.num >= 500 {
            write!(f, "D{}", Roman::from(self.num - 500))
        } else if self.num >= 400 {
            write!(f, "CD{}", Roman::from(self.num - 400))
        } else if self.num >= 100 {
            write!(f, "C{}", Roman::from(self.num - 100))
        } else if self.num >= 90 {
            write!(f, "XC{}", Roman::from(self.num - 90))
        } else if self.num >= 50 {
            write!(f, "L{}", Roman::from(self.num - 50))
        } else if self.num >= 40 {
            write!(f, "XL{}", Roman::from(self.num - 40))
        } else if self.num >= 10 {
            write!(f, "X{}", Roman::from(self.num - 10))
        } else if self.num == 9 {
            write!(f, "IX{}", Roman::from(self.num - 9))
        } else if self.num >= 5 {
            write!(f, "V{}", Roman::from(self.num - 5))
        } else if self.num == 4 {
            write!(f, "IV{}", Roman::from(self.num - 4))
        } else if self.num >= 1 {
            write!(f, "I{}", Roman::from(self.num - 1))
        } else if self.num == 0 {
            write!(f, "")
        } else {
            Ok(())
        }
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman { num }
    }
}
