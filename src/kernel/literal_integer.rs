pub use crate::generated::literal_integer::{
    LiteralInteger, LiteralIntegerMethods, LiteralIntegerRef, LiteralIntegerRefMethods,
    LiteralIntegerRefMut, LiteralIntegerRefMutMethods,
};

impl LiteralIntegerMethods for LiteralInteger {}

impl LiteralIntegerRefMutMethods for LiteralIntegerRefMut<'_> {}

impl LiteralIntegerRefMethods for LiteralIntegerRef<'_> {}
