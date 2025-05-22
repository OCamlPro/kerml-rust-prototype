pub use crate::generated::feature_membership::{
    FeatureMembership, FeatureMembershipMethods, FeatureMembershipRef, FeatureMembershipRefMethods,
    FeatureMembershipRefMut, FeatureMembershipRefMutMethods,
};

impl FeatureMembershipMethods for FeatureMembership {}

impl FeatureMembershipRefMutMethods for FeatureMembershipRefMut<'_> {}

impl FeatureMembershipRefMethods for FeatureMembershipRef<'_> {}
