pub use crate::generated::behavior::{
    Behavior, BehaviorMethods, BehaviorRef, BehaviorRefMethods, BehaviorRefMut,
    BehaviorRefMutMethods,
};

impl BehaviorMethods for Behavior {}

impl BehaviorRefMutMethods for BehaviorRefMut<'_> {}

impl BehaviorRefMethods for BehaviorRef<'_> {}
