pub use crate::generated::textual_representation::{
    TextualRepresentation, TextualRepresentationMethods, TextualRepresentationRef,
    TextualRepresentationRefMethods, TextualRepresentationRefMut,
    TextualRepresentationRefMutMethods,
};

impl TextualRepresentationMethods for TextualRepresentation {}

impl TextualRepresentationRefMethods for TextualRepresentationRef<'_> {}

impl TextualRepresentationRefMutMethods for TextualRepresentationRefMut<'_> {}
