pub use crate::generated::operator_expression::{
    OperatorExpression, OperatorExpressionMethods, OperatorExpressionRef,
    OperatorExpressionRefMethods, OperatorExpressionRefMut, OperatorExpressionRefMutMethods,
};

impl OperatorExpressionMethods for OperatorExpression {}

impl OperatorExpressionRefMutMethods for OperatorExpressionRefMut<'_> {}

impl OperatorExpressionRefMethods for OperatorExpressionRef<'_> {}
