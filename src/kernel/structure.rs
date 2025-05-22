pub use crate::generated::structure::{
    Structure, StructureMethods, StructureRef, StructureRefMethods, StructureRefMut,
    StructureRefMutMethods,
};

impl StructureMethods for Structure {}

impl StructureRefMutMethods for StructureRefMut<'_> {}

impl StructureRefMethods for StructureRef<'_> {}



