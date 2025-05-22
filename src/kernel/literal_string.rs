pub use crate::generated::literal_string::{
    LiteralString, LiteralStringMethods, LiteralStringRef, LiteralStringRefMethods,
    LiteralStringRefMut, LiteralStringRefMutMethods,
};

impl LiteralStringMethods for LiteralString {}

impl LiteralStringRefMutMethods for LiteralStringRefMut<'_> {}

impl LiteralStringRefMethods for LiteralStringRef<'_> {}
