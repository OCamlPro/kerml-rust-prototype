pub use crate::generated::collect_expression::{
    CollectExpression, CollectExpressionMethods, CollectExpressionRef, CollectExpressionRefMethods,
    CollectExpressionRefMut, CollectExpressionRefMutMethods,
};

impl CollectExpressionMethods for CollectExpression {}

impl CollectExpressionRefMutMethods for CollectExpressionRefMut<'_> {}

impl CollectExpressionRefMethods for CollectExpressionRef<'_> {}
