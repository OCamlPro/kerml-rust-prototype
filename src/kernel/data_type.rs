pub use crate::generated::data_type::{
    DataType, DataTypeMethods, DataTypeRef, DataTypeRefMethods, DataTypeRefMut,
    DataTypeRefMutMethods,
};

impl DataTypeMethods for DataType {}

impl DataTypeRefMutMethods for DataTypeRefMut<'_> {}

impl DataTypeRefMethods for DataTypeRef<'_> {}


