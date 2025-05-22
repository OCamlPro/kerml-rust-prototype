pub use crate::generated::result_expression_membership::{
    ResultExpressionMembership, ResultExpressionMembershipMethods, ResultExpressionMembershipRef,
    ResultExpressionMembershipRefMethods, ResultExpressionMembershipRefMut,
    ResultExpressionMembershipRefMutMethods,
};

impl ResultExpressionMembershipMethods for ResultExpressionMembership {}

impl ResultExpressionMembershipRefMutMethods for ResultExpressionMembershipRefMut<'_> {}

impl ResultExpressionMembershipRefMethods for ResultExpressionMembershipRef<'_> {}
