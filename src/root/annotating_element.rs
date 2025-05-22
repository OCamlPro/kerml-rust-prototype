pub use crate::generated::annotating_element::{
    AnnotatingElement, AnnotatingElementMethods, AnnotatingElementRef, AnnotatingElementRefMethods,
    AnnotatingElementRefMut, AnnotatingElementRefMutMethods,
};

impl AnnotatingElementMethods for AnnotatingElement {}

impl AnnotatingElementRefMethods for AnnotatingElementRef<'_> {}

impl AnnotatingElementRefMutMethods for AnnotatingElementRefMut<'_> {}
