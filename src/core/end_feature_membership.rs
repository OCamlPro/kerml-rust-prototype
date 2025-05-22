pub use crate::generated::end_feature_membership::{
    EndFeatureMembership, EndFeatureMembershipMethods, EndFeatureMembershipRef,
    EndFeatureMembershipRefMethods, EndFeatureMembershipRefMut, EndFeatureMembershipRefMutMethods,
};

impl EndFeatureMembershipMethods for EndFeatureMembership {}

impl EndFeatureMembershipRefMutMethods for EndFeatureMembershipRefMut<'_> {}

impl EndFeatureMembershipRefMethods for EndFeatureMembershipRef<'_> {}
