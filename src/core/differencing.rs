pub use crate::generated::differencing::{
    Differencing, DifferencingMethods, DifferencingRef, DifferencingRefMethods, DifferencingRefMut,
    DifferencingRefMutMethods,
};

impl DifferencingMethods for Differencing {}

impl DifferencingRefMutMethods for DifferencingRefMut<'_> {}

impl DifferencingRefMethods for DifferencingRef<'_> {}

