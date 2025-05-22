pub use crate::generated::succession::{
    Succession, SuccessionMethods, SuccessionRef, SuccessionRefMethods, SuccessionRefMut,
    SuccessionRefMutMethods,
};

impl SuccessionMethods for Succession {}

impl SuccessionRefMutMethods for SuccessionRefMut<'_> {}

impl SuccessionRefMethods for SuccessionRef<'_> {}
