pub use crate::generated::return_parameter_membership::{
    ReturnParameterMembership, ReturnParameterMembershipMethods, ReturnParameterMembershipRef,
    ReturnParameterMembershipRefMethods, ReturnParameterMembershipRefMut,
    ReturnParameterMembershipRefMutMethods,
};

impl ReturnParameterMembershipMethods for ReturnParameterMembership {}

impl ReturnParameterMembershipRefMutMethods for ReturnParameterMembershipRefMut<'_> {}

impl ReturnParameterMembershipRefMethods for ReturnParameterMembershipRef<'_> {}
