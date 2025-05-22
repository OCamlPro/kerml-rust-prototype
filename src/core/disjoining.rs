pub use crate::generated::disjoining::{
    Disjoining, DisjoiningMethods, DisjoiningRef, DisjoiningRefMethods, DisjoiningRefMut,
    DisjoiningRefMutMethods,
};

impl DisjoiningMethods for Disjoining {}

impl DisjoiningRefMutMethods for DisjoiningRefMut<'_> {}

impl DisjoiningRefMethods for DisjoiningRef<'_> {}

