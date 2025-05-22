pub use crate::generated::reference_subsetting::{
    ReferenceSubsetting, ReferenceSubsettingMethods, ReferenceSubsettingRef,
    ReferenceSubsettingRefMethods, ReferenceSubsettingRefMut, ReferenceSubsettingRefMutMethods,
};

impl ReferenceSubsettingMethods for ReferenceSubsetting {}

impl ReferenceSubsettingRefMutMethods for ReferenceSubsettingRefMut<'_> {}

impl ReferenceSubsettingRefMethods for ReferenceSubsettingRef<'_> {}
