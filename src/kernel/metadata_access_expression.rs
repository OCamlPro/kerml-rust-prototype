pub use crate::generated::metadata_access_expression::{
    MetadataAccessExpression, MetadataAccessExpressionMethods, MetadataAccessExpressionRef,
    MetadataAccessExpressionRefMethods, MetadataAccessExpressionRefMut,
    MetadataAccessExpressionRefMutMethods,
};

impl MetadataAccessExpressionMethods for MetadataAccessExpression {}

impl MetadataAccessExpressionRefMutMethods for MetadataAccessExpressionRefMut<'_> {}

impl MetadataAccessExpressionRefMethods for MetadataAccessExpressionRef<'_> {
    fn metaclass_feature(
        self,
    ) -> std::rc::Rc<std::cell::RefCell<crate::generated::metadata_feature::MetadataFeature>> {
        todo!()
    }
}
