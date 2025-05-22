pub use crate::generated::subsetting::{
    Subsetting, SubsettingMethods, SubsettingRef, SubsettingRefMethods, SubsettingRefMut,
    SubsettingRefMutMethods,
};

impl SubsettingMethods for Subsetting {}

impl SubsettingRefMutMethods for SubsettingRefMut<'_> {}

impl SubsettingRefMethods for SubsettingRef<'_> {}
