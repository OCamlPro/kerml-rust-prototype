pub use crate::generated::literal_rational::{
    LiteralRational, LiteralRationalMethods, LiteralRationalRef, LiteralRationalRefMethods,
    LiteralRationalRefMut, LiteralRationalRefMutMethods,
};

impl LiteralRationalMethods for LiteralRational {}

impl LiteralRationalRefMutMethods for LiteralRationalRefMut<'_> {}

impl LiteralRationalRefMethods for LiteralRationalRef<'_> {}
