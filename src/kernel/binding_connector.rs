pub use crate::generated::binding_connector::{
    BindingConnector, BindingConnectorMethods, BindingConnectorRef, BindingConnectorRefMethods,
    BindingConnectorRefMut, BindingConnectorRefMutMethods,
};

impl BindingConnectorMethods for BindingConnector {}

impl BindingConnectorRefMutMethods for BindingConnectorRefMut<'_> {}

impl BindingConnectorRefMethods for BindingConnectorRef<'_> {}
