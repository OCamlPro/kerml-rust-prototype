pub use crate::generated::feature::{
    Feature, FeatureMethods, FeatureRef, FeatureRefMethods, FeatureRefMut, FeatureRefMutMethods,
};

impl FeatureMethods for Feature {}

impl FeatureRefMutMethods for FeatureRefMut<'_> {}

impl FeatureRefMethods for FeatureRef<'_> {
    fn direction_for(
        self,
        type_: std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<crate::generated::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        todo!()
    }

    fn naming_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>> {
        todo!()
    }

    fn redefines(
        self,
        redefined_feature: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
    ) -> bool {
        todo!()
    }

    fn redefines_from_library(self, library_feature_name: String) -> bool {
        todo!()
    }

    fn subsets_chain(
        self,
        first: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
        second: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
    ) -> bool {
        todo!()
    }

    fn typing_features(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>> {
        todo!()
    }

    fn as_cartesian_product(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>> {
        todo!()
    }

    fn is_cartesian_product(self) -> bool {
        todo!()
    }

    fn is_owned_cross_feature(self) -> bool {
        todo!()
    }

    fn owned_cross_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>> {
        todo!()
    }

    fn all_redefined_features(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>> {
        todo!()
    }

    fn is_featured_within(
        self,
        type_: Option<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>>,
    ) -> bool {
        todo!()
    }

    fn can_access(
        self,
        feature: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
    ) -> bool {
        todo!()
    }

    fn is_featuring_type(
        self,
        type_: std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>,
    ) -> bool {
        todo!()
    }
}
