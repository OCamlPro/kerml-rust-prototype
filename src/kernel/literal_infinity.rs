pub use crate::generated::literal_infinity::{
    LiteralInfinity, LiteralInfinityMethods, LiteralInfinityRef, LiteralInfinityRefMethods,
    LiteralInfinityRefMut, LiteralInfinityRefMutMethods,
};

impl LiteralInfinityMethods for LiteralInfinity {}

impl LiteralInfinityRefMutMethods for LiteralInfinityRefMut<'_> {}

impl LiteralInfinityRefMethods for LiteralInfinityRef<'_> {}
