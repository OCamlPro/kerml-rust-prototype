pub use crate::generated::function::{
    Function, FunctionMethods, FunctionRef, FunctionRefMethods, FunctionRefMut,
    FunctionRefMutMethods,
};

impl FunctionMethods for Function {}

impl FunctionRefMutMethods for FunctionRefMut<'_> {}

impl FunctionRefMethods for FunctionRef<'_> {}
