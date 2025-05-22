pub use crate::generated::membership::{Membership, MembershipMethods, MembershipRef, MembershipRefMethods, MembershipRefMut, MembershipRefMutMethods};

impl MembershipMethods for Membership {}

impl MembershipRefMutMethods for MembershipRefMut<'_> {}

impl MembershipRefMethods for MembershipRef<'_> {
    fn is_distinguishable_from(
        self,
        other: std::rc::Rc<std::cell::RefCell<crate::generated::membership::Membership>>,
    ) -> bool {
        todo!()
    }
}
