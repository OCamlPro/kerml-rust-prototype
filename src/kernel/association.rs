pub use crate::generated::association::{
    Association, AssociationMethods, AssociationRef, AssociationRefMethods, AssociationRefMut,
    AssociationRefMutMethods,
};

impl AssociationMethods for Association {}

impl AssociationRefMutMethods for AssociationRefMut<'_> {}

impl AssociationRefMethods for AssociationRef<'_> {}



