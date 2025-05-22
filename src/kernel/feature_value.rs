pub use crate::generated::feature_value::{
    FeatureValue, FeatureValueMethods, FeatureValueRef, FeatureValueRefMethods, FeatureValueRefMut,
    FeatureValueRefMutMethods,
};

impl FeatureValueMethods for FeatureValue {}

impl FeatureValueRefMutMethods for FeatureValueRefMut<'_> {}

impl FeatureValueRefMethods for FeatureValueRef<'_> {}
