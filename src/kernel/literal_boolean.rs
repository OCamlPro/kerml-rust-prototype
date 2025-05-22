pub use crate::generated::literal_boolean::{
    LiteralBoolean, LiteralBooleanMethods, LiteralBooleanRef, LiteralBooleanRefMethods,
    LiteralBooleanRefMut, LiteralBooleanRefMutMethods,
};

impl LiteralBooleanMethods for LiteralBoolean {}

impl LiteralBooleanRefMutMethods for LiteralBooleanRefMut<'_> {}

impl LiteralBooleanRefMethods for LiteralBooleanRef<'_> {}
