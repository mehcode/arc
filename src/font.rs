use crate::os;

pub struct Font {
    crate inner: os::Font,
}

impl Font {
    #[inline]
    pub fn builder() -> FontBuilder {
        FontBuilder::new()
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub struct FontBuilder {
    family: &'static str,
    weight: u16,
    italic: bool,
}

impl FontBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            family: ".SF NS Text",
            weight: 400,
            italic: false,
        }
    }

    // TODO: Should we expose family name as "family" or "name"
    // TODO: Is a non literal needed here?

    #[inline]
    pub fn name(&mut self, name: &'static str) -> &mut Self {
        self.family = name;
        self
    }

    // TODO: How should we handle invalid font weights?

    #[inline]
    pub fn weight(&mut self, weight: u16) -> &mut Self {
        self.weight = weight;
        self
    }

    #[inline]
    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.italic = italic;
        self
    }

    #[inline]
    pub fn build(&self) -> Font {
        Font {
            inner: os::Font::new(&self.family, self.weight, self.italic),
        }
    }
}
