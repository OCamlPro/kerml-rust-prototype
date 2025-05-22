pub use crate::generated::association_structure::{
    AssociationStructure, AssociationStructureMethods, AssociationStructureRef,
    AssociationStructureRefMethods, AssociationStructureRefMut, AssociationStructureRefMutMethods,
};

impl AssociationStructureMethods for AssociationStructure {}

impl AssociationStructureRefMutMethods for AssociationStructureRefMut<'_> {}

impl AssociationStructureRefMethods for AssociationStructureRef<'_> {}
