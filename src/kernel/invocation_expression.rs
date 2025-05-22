pub use crate::generated::invocation_expression::{
    InvocationExpression, InvocationExpressionMethods, InvocationExpressionRef,
    InvocationExpressionRefMethods, InvocationExpressionRefMut, InvocationExpressionRefMutMethods,
};

impl InvocationExpressionMethods for InvocationExpression {}

impl InvocationExpressionRefMutMethods for InvocationExpressionRefMut<'_> {}

impl InvocationExpressionRefMethods for InvocationExpressionRef<'_> {}
