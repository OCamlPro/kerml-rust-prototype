pub use crate::generated::element_filter_membership::{
    ElementFilterMembership, ElementFilterMembershipMethods, ElementFilterMembershipRef,
    ElementFilterMembershipRefMethods, ElementFilterMembershipRefMut,
    ElementFilterMembershipRefMutMethods,
};

impl ElementFilterMembershipMethods for ElementFilterMembership {}

impl ElementFilterMembershipRefMutMethods for ElementFilterMembershipRefMut<'_> {}

impl ElementFilterMembershipRefMethods for ElementFilterMembershipRef<'_> {}
