pub use crate::generated::connector::{
    Connector, ConnectorMethods, ConnectorRef, ConnectorRefMethods, ConnectorRefMut,
    ConnectorRefMutMethods,
};

impl ConnectorMethods for Connector {}

impl ConnectorRefMutMethods for ConnectorRefMut<'_> {}

impl ConnectorRefMethods for ConnectorRef<'_> {}
