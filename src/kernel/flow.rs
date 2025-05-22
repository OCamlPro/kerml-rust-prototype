pub use crate::generated::flow::{
    Flow, FlowMethods, FlowRef, FlowRefMethods, FlowRefMut, FlowRefMutMethods,
};

impl FlowMethods for Flow {}

impl FlowRefMutMethods for FlowRefMut<'_> {}

impl FlowRefMethods for FlowRef<'_> {}
