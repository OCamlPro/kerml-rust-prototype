pub use crate::generated::owning_membership::{
    OwningMembership, OwningMembershipMethods, OwningMembershipRef, OwningMembershipRefMethods,
    OwningMembershipRefMut, OwningMembershipRefMutMethods,
};

impl OwningMembershipMethods for OwningMembership {}
impl OwningMembershipRefMethods for OwningMembershipRef<'_> {}
impl OwningMembershipRefMutMethods for OwningMembershipRefMut<'_> {}

