pub use crate::generated::namespace_import::{
    NamespaceImport, NamespaceImportMethods, NamespaceImportRef, NamespaceImportRefMethods,
    NamespaceImportRefMut, NamespaceImportRefMutMethods,
};

impl NamespaceImportMethods for NamespaceImport {}
impl NamespaceImportRefMethods for NamespaceImportRef<'_> {}
impl NamespaceImportRefMutMethods for NamespaceImportRefMut<'_> {}
