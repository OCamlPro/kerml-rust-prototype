pub use crate::generated::annotation::{
    Annotation, AnnotationMethods, AnnotationRef, AnnotationRefMethods, AnnotationRefMut,
    AnnotationRefMutMethods,
};

impl AnnotationMethods for Annotation {}

impl AnnotationRefMethods for AnnotationRef<'_> {}

impl AnnotationRefMutMethods for AnnotationRefMut<'_> {}
