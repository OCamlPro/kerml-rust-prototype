pub use crate::generated::import::{Import, ImportMethods, ImportRef, ImportRefMethods, ImportRefMut, ImportRefMutMethods};

impl ImportMethods for Import {}

impl ImportRefMethods for ImportRef<'_> {
    fn imported_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }
}

impl ImportRefMutMethods for ImportRefMut<'_> {}
