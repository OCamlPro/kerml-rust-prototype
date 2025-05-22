pub use crate::generated::feature_chaining::{
    FeatureChaining, FeatureChainingMethods, FeatureChainingRef, FeatureChainingRefMethods,
    FeatureChainingRefMut, FeatureChainingRefMutMethods,
};

impl FeatureChainingMethods for FeatureChaining {}

impl FeatureChainingRefMutMethods for FeatureChainingRefMut<'_> {}

impl FeatureChainingRefMethods for FeatureChainingRef<'_> {}
