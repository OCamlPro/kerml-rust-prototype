pub use crate::generated::null_expression::{
    NullExpression, NullExpressionMethods, NullExpressionRef, NullExpressionRefMethods,
    NullExpressionRefMut, NullExpressionRefMutMethods,
};

impl NullExpressionMethods for NullExpression {}

impl NullExpressionRefMutMethods for NullExpressionRefMut<'_> {}

impl NullExpressionRefMethods for NullExpressionRef<'_> {}
