pub use crate::generated::expression::{
    Expression, ExpressionMethods, ExpressionRef, ExpressionRefMethods, ExpressionRefMut,
    ExpressionRefMutMethods,
};

impl ExpressionMethods for Expression {}

impl ExpressionRefMutMethods for ExpressionRefMut<'_> {}

impl ExpressionRefMethods for ExpressionRef<'_> {
    fn model_level_evaluable(
        self,
        visited: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>>,
    ) -> bool {
        todo!()
    }

    fn evaluate(
        self,
        target: std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>> {
        todo!()
    }

    fn check_condition(
        self,
        target: std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>,
    ) -> bool {
        todo!()
    }
}
