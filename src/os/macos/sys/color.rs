use super::super::super::super::Color;
use core_graphics::{base::CGFloat, color::CGColor};

impl Color {
    #[inline]
    pub(crate) fn into_cgcolor(self) -> CGColor {
        CGColor::rgb(
            CGFloat::from(self.red),
            CGFloat::from(self.green),
            CGFloat::from(self.blue),
            CGFloat::from(self.alpha),
        )
    }
}
