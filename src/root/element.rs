pub use crate::generated::element::{
    Element, ElementMethods, ElementRef, ElementRefMethods, ElementRefMut, ElementRefMutMethods,
};

impl ElementMethods for Element {}
impl<'a> ElementRefMethods for ElementRef<'a> {
    fn path(self) -> String {
        todo!()
    }

    fn escaped_name(self) -> Option<String> {
        todo!()
    }

    fn effective_short_name(self) -> Option<String> {
        todo!()
    }

    fn effective_name(self) -> Option<String> {
        todo!()
    }

    fn library_namespace(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>> {
        todo!()
    }
}
impl<'a> ElementRefMutMethods for ElementRefMut<'a> {}
