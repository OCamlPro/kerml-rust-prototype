pub use crate::generated::type_::{
    Type, TypeMethods, TypeRef, TypeRefMethods, TypeRefMut,
    TypeRefMutMethods,
};

impl TypeMethods for Type {}

impl TypeRefMutMethods for TypeRefMut<'_> {}

impl TypeRefMethods for TypeRef<'_> {
    fn inherited_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn inheritable_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn non_private_memberships(
        self,
        excluded_namespaces: Vec<
            std::rc::Rc<std::cell::RefCell<crate::generated::namespace::Namespace>>,
        >,
        excluded_types: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>>,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn remove_redefined_features(
        self,
        memberships: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>> {
        todo!()
    }

    fn all_redefined_features_of(
        self,
        membership: std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>> {
        todo!()
    }

    fn direction_of(
        self,
        feature: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<crate::generated::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        todo!()
    }

    fn direction_of_excluding(
        self,
        feature: std::rc::Rc<std::cell::RefCell<crate::generated::feature::Feature>>,
        excluded: Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<crate::generated::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        todo!()
    }

    fn supertypes(
        self,
        exclude_implied: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>> {
        todo!()
    }

    fn all_supertypes(self) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>> {
        todo!()
    }

    fn specializes(
        self,
        supertype: std::rc::Rc<std::cell::RefCell<crate::generated::type_::Type>>,
    ) -> bool {
        todo!()
    }

    fn specializes_from_library(self, library_type_name: String) -> bool {
        todo!()
    }

    fn multiplicities(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<crate::generated::multiplicity::Multiplicity>>> {
        todo!()
    }
}

