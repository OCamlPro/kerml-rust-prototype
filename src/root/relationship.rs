pub use crate::generated::relationship::{
    Relationship, RelationshipMethods, RelationshipRef, RelationshipRefMethods, RelationshipRefMut,
    RelationshipRefMutMethods,
};

impl RelationshipMethods for Relationship {}

impl<'a> RelationshipRefMutMethods for RelationshipRefMut<'a> {}

impl<'a> RelationshipRefMethods for RelationshipRef<'a> {}
