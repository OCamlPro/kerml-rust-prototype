pub use crate::generated::feature_inverting::{
    FeatureInverting, FeatureInvertingMethods, FeatureInvertingRef, FeatureInvertingRefMethods,
    FeatureInvertingRefMut, FeatureInvertingRefMutMethods,
};

impl FeatureInvertingMethods for FeatureInverting {}

impl FeatureInvertingRefMutMethods for FeatureInvertingRefMut<'_> {}

impl FeatureInvertingRefMethods for FeatureInvertingRef<'_> {}
