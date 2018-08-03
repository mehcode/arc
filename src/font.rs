use crate::{os, Context, WeakContext};

pub struct Font {
    crate inner: os::Font,
}

impl Font {
    #[inline]
    pub fn builder(context: &Context) -> FontBuilder {
        FontBuilder::new(context)
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub struct FontBuilder {
    family: &'static str,
    weight: u16,
    italic: bool,

    // NOTE: Currently this is only here to force Fonts to be made after `Context` creation
    #[allow(dead_code)]
    context: WeakContext,
}

impl FontBuilder {
    #[inline]
    pub fn new(context: &Context) -> Self {
        Self {
            family: ".SF NS Text",
            weight: 400,
            italic: false,
            context: context.downgrade(),
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
