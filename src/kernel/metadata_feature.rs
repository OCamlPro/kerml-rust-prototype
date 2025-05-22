pub use crate::generated::metadata_feature::{
    MetadataFeature, MetadataFeatureMethods, MetadataFeatureRef, MetadataFeatureRefMethods,
    MetadataFeatureRefMut, MetadataFeatureRefMutMethods,
};

impl MetadataFeatureMethods for MetadataFeature {}

impl MetadataFeatureRefMutMethods for MetadataFeatureRefMut<'_> {}

impl MetadataFeatureRefMethods for MetadataFeatureRef<'_> {
    fn evaluate_feature(
        self,
        base_feature: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>> {
        todo!()
    }

    fn is_semantic(self) -> bool {
        todo!()
    }

    fn is_syntactic(self) -> bool {
        todo!()
    }

    fn syntax_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::element::Element>>> {
        todo!()
    }
}
