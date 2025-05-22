pub use crate::generated::parameter_membership::{
    ParameterMembership, ParameterMembershipMethods, ParameterMembershipRef, ParameterMembershipRefMethods, ParameterMembershipRefMut, ParameterMembershipRefMutMethods,
};

impl ParameterMembershipMethods for ParameterMembership {}

impl ParameterMembershipRefMutMethods for ParameterMembershipRefMut<'_> {}

impl ParameterMembershipRefMethods for ParameterMembershipRef<'_> {
    fn parameter_direction(
        self,
    ) -> std::rc::Rc<
        std::cell::RefCell<crate::generated::feature_direction_kind::FeatureDirectionKind>,
    > {
        todo!()
    }
}


