pub use crate::generated::step::{
    Step, StepMethods, StepRef, StepRefMethods, StepRefMut, StepRefMutMethods,
};

impl StepMethods for Step {}

impl StepRefMutMethods for StepRefMut<'_> {}

impl StepRefMethods for StepRef<'_> {}
