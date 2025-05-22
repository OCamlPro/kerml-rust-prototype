pub use crate::generated::boolean_expression::{
    BooleanExpression, BooleanExpressionMethods, BooleanExpressionRef, BooleanExpressionRefMethods,
    BooleanExpressionRefMut, BooleanExpressionRefMutMethods,
};

impl BooleanExpressionMethods for BooleanExpression {}

impl BooleanExpressionRefMutMethods for BooleanExpressionRefMut<'_> {}

impl BooleanExpressionRefMethods for BooleanExpressionRef<'_> {}
