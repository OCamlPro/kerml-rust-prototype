pub use crate::generated::feature_reference_expression::{
    FeatureReferenceExpression, FeatureReferenceExpressionMethods, FeatureReferenceExpressionRef,
    FeatureReferenceExpressionRefMethods, FeatureReferenceExpressionRefMut,
    FeatureReferenceExpressionRefMutMethods,
};

impl FeatureReferenceExpressionMethods for FeatureReferenceExpression {}

impl FeatureReferenceExpressionRefMutMethods for FeatureReferenceExpressionRefMut<'_> {}

impl FeatureReferenceExpressionRefMethods for FeatureReferenceExpressionRef<'_> {}
