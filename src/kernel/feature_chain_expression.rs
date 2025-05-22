pub use crate::generated::feature_chain_expression::{
    FeatureChainExpression, FeatureChainExpressionMethods, FeatureChainExpressionRef,
    FeatureChainExpressionRefMethods, FeatureChainExpressionRefMut,
    FeatureChainExpressionRefMutMethods,
};

impl FeatureChainExpressionMethods for FeatureChainExpression {}

impl FeatureChainExpressionRefMutMethods for FeatureChainExpressionRefMut<'_> {}

impl FeatureChainExpressionRefMethods for FeatureChainExpressionRef<'_> {
    fn source_target_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>> {
        todo!()
    }
}
