pub use crate::generated::succession_flow::{
    SuccessionFlow, SuccessionFlowMethods, SuccessionFlowRef, SuccessionFlowRefMethods,
    SuccessionFlowRefMut, SuccessionFlowRefMutMethods,
};

impl SuccessionFlowMethods for SuccessionFlow {}

impl SuccessionFlowRefMutMethods for SuccessionFlowRefMut<'_> {}

impl SuccessionFlowRefMethods for SuccessionFlowRef<'_> {}
