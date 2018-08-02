use core_foundation::{
    base::{FromVoid, TCFType},
    dictionary::{CFDictionary, CFMutableDictionary},
    number::CFNumber,
    string::CFString,
};
use core_foundation_sys::base::TCFTypeRef;
use core_graphics::base::CGFloat;
use core_text::font_descriptor::{
    self, kCTFontFamilyNameAttribute, kCTFontItalicTrait, kCTFontSymbolicTrait,
    kCTFontTraitsAttribute, kCTFontUIOptimizedTrait, kCTFontWeightTrait, CTFontDescriptor,
};

#[derive(Clone)]
pub(crate) struct Font(pub(crate) CTFontDescriptor);

impl Font {
    pub(crate) fn new(family: &str, weight: u16, italic: bool) -> Self {
        // Font weights in macOS must use constants
        // Map normal weights to macOS constants
        let weight = CFNumber::from(match weight {
            100 => unsafe { NSFontWeightUltraLight },
            200 => unsafe { NSFontWeightThin },
            300 => unsafe { NSFontWeightLight },
            400 => unsafe { NSFontWeightRegular },
            500 => unsafe { NSFontWeightMedium },
            600 => unsafe { NSFontWeightSemibold },
            700 => unsafe { NSFontWeightBold },
            800 => unsafe { NSFontWeightHeavy },
            900 => unsafe { NSFontWeightBlack },

            _ => {
                // This should have been caught by the public Font API
                panic!("unsupported font weight: {}", weight);
            }
        });

        let mut traits = CFMutableDictionary::new();

        traits.set(
            unsafe { CFString::wrap_under_create_rule(kCTFontWeightTrait) },
            weight,
        );

        // Symbolic traits include
        //  - italic
        //  - letter spacing
        //  - "ui optimized" (not sure what that means)

        // TODO: Investigate what this actually does and if we should have it always-on, flag, etc.
        let mut sym = kCTFontUIOptimizedTrait;

        if italic {
            sym |= kCTFontItalicTrait;
        }

        traits.set(
            unsafe { CFString::wrap_under_create_rule(kCTFontSymbolicTrait) },
            CFNumber::from(i64::from(sym)),
        );

        let mut attrs = CFMutableDictionary::new();

        attrs.set(
            unsafe { CFString::wrap_under_create_rule(kCTFontFamilyNameAttribute) },
            CFString::from(family).as_CFType(),
        );

        attrs.set(
            unsafe { CFString::wrap_under_create_rule(kCTFontTraitsAttribute) },
            traits.as_CFType(),
        );

        let attrs = unsafe { CFDictionary::from_void(attrs.as_concrete_TypeRef().as_void_ptr()) };
        let desc = font_descriptor::new_from_attributes(&attrs);

        Font(desc)
    }
}

extern "C" {
    static NSFontWeightUltraLight: CGFloat;
    static NSFontWeightThin: CGFloat;
    static NSFontWeightLight: CGFloat;
    static NSFontWeightRegular: CGFloat;
    static NSFontWeightMedium: CGFloat;
    static NSFontWeightSemibold: CGFloat;
    static NSFontWeightBold: CGFloat;
    static NSFontWeightHeavy: CGFloat;
    static NSFontWeightBlack: CGFloat;
}
