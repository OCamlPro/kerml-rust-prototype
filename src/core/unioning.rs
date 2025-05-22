pub use crate::generated::unioning::{
    Unioning, UnioningMethods, UnioningRef, UnioningRefMethods, UnioningRefMut,
    UnioningRefMutMethods,
};

impl UnioningMethods for Unioning {}

impl UnioningRefMutMethods for UnioningRefMut<'_> {}

impl UnioningRefMethods for UnioningRef<'_> {}

