pub use crate::generated::documentation::{Documentation, DocumentationMethods, DocumentationRef, DocumentationRefMethods, DocumentationRefMut, DocumentationRefMutMethods};

impl DocumentationMethods for Documentation {}
impl DocumentationRefMethods for DocumentationRef<'_> {}
impl DocumentationRefMutMethods for DocumentationRefMut<'_> {}
