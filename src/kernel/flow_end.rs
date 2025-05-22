pub use crate::generated::flow_end::{
    FlowEnd, FlowEndMethods, FlowEndRef, FlowEndRefMethods, FlowEndRefMut, FlowEndRefMutMethods,
};

impl FlowEndMethods for FlowEnd {}

impl FlowEndRefMutMethods for FlowEndRefMut<'_> {}

impl FlowEndRefMethods for FlowEndRef<'_> {}
