pub use crate::generated::intersecting::{
    Intersecting, IntersectingMethods, IntersectingRef, IntersectingRefMethods, IntersectingRefMut,
    IntersectingRefMutMethods,
};

impl IntersectingMethods for Intersecting {}

impl IntersectingRefMutMethods for IntersectingRefMut<'_> {}

impl IntersectingRefMethods for IntersectingRef<'_> {}
