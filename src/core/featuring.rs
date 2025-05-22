pub use crate::generated::featuring::{
    Featuring, FeaturingMethods, FeaturingRef, FeaturingRefMethods, FeaturingRefMut,
    FeaturingRefMutMethods,
};

impl FeaturingMethods for Featuring {}

impl FeaturingRefMutMethods for FeaturingRefMut<'_> {}

impl FeaturingRefMethods for FeaturingRef<'_> {}

