pub use crate::generated::literal_expression::{
    LiteralExpression, LiteralExpressionMethods, LiteralExpressionRef, LiteralExpressionRefMethods,
    LiteralExpressionRefMut, LiteralExpressionRefMutMethods,
};

impl LiteralExpressionMethods for LiteralExpression {}

impl LiteralExpressionRefMutMethods for LiteralExpressionRefMut<'_> {}

impl LiteralExpressionRefMethods for LiteralExpressionRef<'_> {}
