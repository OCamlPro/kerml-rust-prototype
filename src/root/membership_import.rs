pub use crate::generated::membership_import::{
    MembershipImport, MembershipImportMethods, MembershipImportRef, MembershipImportRefMethods,
    MembershipImportRefMut, MembershipImportRefMutMethods,
};

impl MembershipImportMethods for MembershipImport {}
impl MembershipImportRefMethods for MembershipImportRef<'_> {}
impl MembershipImportRefMutMethods for MembershipImportRefMut<'_> {}
