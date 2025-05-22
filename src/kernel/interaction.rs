pub use crate::generated::interaction::{
    Interaction, InteractionMethods, InteractionRef, InteractionRefMethods, InteractionRefMut,
    InteractionRefMutMethods,
};

impl InteractionMethods for Interaction {}

impl InteractionRefMutMethods for InteractionRefMut<'_> {}

impl InteractionRefMethods for InteractionRef<'_> {}
