pub use crate::generated::namespace::{
    Namespace, NamespaceMethods, NamespaceRef, NamespaceRefMethods,
    NamespaceRefMut, NamespaceRefMutMethods,
};

impl NamespaceMethods for Namespace {}
impl NamespaceRefMethods for NamespaceRef<'_> {
    fn names_of(
        self,
        element: std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>,
    ) -> Vec<String> {
        todo!()
    }

    fn visibility_of(
        self,
        mem: std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>,
    ) -> std::rc::Rc<std::cell::RefCell<crate::generated::visibility_kind::VisibilityKind>> {
        todo!()
    }

    fn visible_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>>,
        is_recursive: bool,
        include_all: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn imported_memberships(
        self,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn memberships_of_visibility(
        self,
        visibility: Option<
            std::rc::Rc<std::cell::RefCell<crate::generated::visibility_kind::VisibilityKind>>,
        >,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn resolve(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn resolve_global(
        self,
        qualified_name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn resolve_local(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn resolve_visible(
        self,
        name: String,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn qualification_of(self, qualified_name: String) -> Option<String> {
        todo!()
    }

    fn unqualified_name_of(self, qualified_name: String) -> String {
        todo!()
    }
}
impl NamespaceRefMutMethods for NamespaceRefMut<'_> {}

