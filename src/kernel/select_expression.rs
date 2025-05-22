pub use crate::generated::select_expression::{
    SelectExpression, SelectExpressionMethods, SelectExpressionRef, SelectExpressionRefMethods,
    SelectExpressionRefMut, SelectExpressionRefMutMethods,
};

impl SelectExpressionMethods for SelectExpression {}

impl SelectExpressionRefMutMethods for SelectExpressionRefMut<'_> {}

impl SelectExpressionRefMethods for SelectExpressionRef<'_> {}
