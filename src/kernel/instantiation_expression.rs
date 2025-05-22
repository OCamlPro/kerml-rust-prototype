pub use crate::generated::instantiation_expression::{
    InstantiationExpression, InstantiationExpressionMethods, InstantiationExpressionRef,
    InstantiationExpressionRefMethods, InstantiationExpressionRefMut,
    InstantiationExpressionRefMutMethods,
};

impl InstantiationExpressionMethods for InstantiationExpression {}

impl InstantiationExpressionRefMutMethods for InstantiationExpressionRefMut<'_> {}

impl InstantiationExpressionRefMethods for InstantiationExpressionRef<'_> {
    fn instantiated_type(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>> {
        todo!()
    }
}
