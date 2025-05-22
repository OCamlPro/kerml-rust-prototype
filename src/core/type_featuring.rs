pub use crate::generated::type_featuring::{
    TypeFeaturing, TypeFeaturingMethods, TypeFeaturingRef, TypeFeaturingRefMethods,
    TypeFeaturingRefMut, TypeFeaturingRefMutMethods,
};

impl TypeFeaturingMethods for TypeFeaturing {}

impl TypeFeaturingRefMutMethods for TypeFeaturingRefMut<'_> {}

impl TypeFeaturingRefMethods for TypeFeaturingRef<'_> {}
