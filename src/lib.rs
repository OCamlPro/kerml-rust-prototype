mod generated;

pub mod core;
pub mod kernel;
pub mod root;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{generated::{
        data_type::DataType,
        element::{ElementStructRefMut, ElementUpcast},
        feature::{Feature, FeatureStructRefMut},
        namespace::NamespaceStructRefMut,
        package::Package,
        structure::Structure,
        type_::{TypeStruct, TypeStructRefMut, TypeUpcast, TypeUpcastRefMut},
    }, utils::new_rc_ref};

    // #[test]
    // fn test_element() {
    //     let elt = element::Element::Itself(ElementInner {
    //         owning_membership: None,
    //         owned_relationship: None,
    //         owning_relationship: None,
    //         owning_namespace: None,
    //         element_id: String::from("1"),
    //         owner: None,
    //         owned_element: Vec::new(),
    //         documentation: Vec::new(),
    //         owned_annotation: Vec::new(),
    //         textual_representation: Vec::new(),
    //         alias_ids: Vec::new(),
    //         declared_short_name: None,
    //         declared_name: None,
    //         short_name: None,
    //         name: None,
    //         qualified_name: None,
    //         is_implied_included: false,
    //         is_library_element: false,
    //     });
    // }
    mod scalar_value {
        use crate::generated::{data_type::DataType, element::ElementStructRefMut};

        pub fn string() -> DataType {
            let mut d = DataType::default();
            d.set_name(Some("String".into()));
            d
        }
    }

    mod collections {
        use std::{cell::RefCell, rc::Rc};

        use crate::{generated::{
            data_type::DataType,
            element::ElementStructRefMut,
            feature::{Feature, FeatureStructRefMut},
            type_::{TypeStructRefMut, TypeUpcast},
        }, utils::new_rc_ref};

        pub fn set(value_type: Rc<RefCell<DataType>>) -> DataType {
            let mut d = DataType::default();
            let mut element_feature = Feature::default();
            element_feature.set_declared_name(Some("elements".into()));
            element_feature
                .type_ref_mut()
                .push(new_rc_ref(value_type.clone().take().into_type_()));
            d.feature_ref_mut()
                .push(new_rc_ref(element_feature));
            d
        }
    }

    #[test]
    fn test_kerml() {
        // package AddressBooks {
        //     datatype Entry {
        //         feature name[1]: String;
        //         feature address[1]: String;
        //     }
        //     struct AddressBook {
        //         composite feature entries[*]: Entry;
        //     }
        // }
        //
        let entry_type = {
            let mut entry_type: DataType = DataType::default();
            let name_feature: Feature = {
                let mut f: Feature = Feature::default();
                f.set_name(Some("name".into()));
                f.type_ref_mut()
                    .push(new_rc_ref(scalar_value::string().into_type_()));
                f
            };
            let address_feature = {
                let mut f = Feature::default();
                f.set_name(Some("address".into()));
                f.type_ref_mut()
                    .push(new_rc_ref(scalar_value::string().into_type_()));
                f
            };
            entry_type
                .feature_ref_mut()
                .push(new_rc_ref(name_feature));
            entry_type
                .feature_ref_mut()
                .push(new_rc_ref(address_feature));
            new_rc_ref(entry_type)
        };
        let _ = Option::default();
        let address_book = {
            let mut s = Structure::default();
            let entries = {
                let t: DataType = collections::set(entry_type.clone());

                let mut entries = Feature::default();
                entries.set_is_composite(true);
                entries.set_name(Some("entries".into()));
                entries
                    .type_ref_mut()
                    .push(new_rc_ref(t.into_type_()));
                entries
            };
            s.feature_ref_mut().push(new_rc_ref(entries));
            s
        };

        let mut pkg = Package::default();
        pkg.owned_member_ref_mut()
            .push(new_rc_ref(entry_type.clone().take().into_element()));

        pkg.owned_member_ref_mut()
            .push(new_rc_ref(address_book.into_element()));
    }
}
