use core::{fmt, str};

use crate::curve_select::CurveSelect;

use super::Point;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl str::FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((curve_select, hex_value)) = s.split_once(":") else  {
            return Err(())
        };
        let curve_select = CurveSelect::from_str(curve_select).map_err(|_| ())?;
        Ok(Self(curve_select, hex_value.to_string()))
    }
}
