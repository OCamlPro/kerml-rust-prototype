pub use crate::generated::redefinition::{
    Redefinition, RedefinitionMethods, RedefinitionRef, RedefinitionRefMethods, RedefinitionRefMut,
    RedefinitionRefMutMethods,
};

impl RedefinitionMethods for Redefinition {}

impl RedefinitionRefMutMethods for RedefinitionRefMut<'_> {}

impl RedefinitionRefMethods for RedefinitionRef<'_> {}


