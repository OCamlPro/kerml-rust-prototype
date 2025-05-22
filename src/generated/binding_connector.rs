#![allow(unused)]
use super::utils::DescendantOf;
use super::connector::{
    Connector, ConnectorRefMut, ConnectorRef, ConnectorStruct, ConnectorStructRefMut,
    ConnectorStructRef, ConnectorInner, ConnectorUpcast, ConnectorUpcastRefMut,
    ConnectorUpcastRef, ConnectorMethodsDescendants, ConnectorRefMutMethodsDescendants,
    ConnectorRefMethodsDescendants,
};
use super::feature::{
    Feature, FeatureRefMut, FeatureRef, FeatureStruct, FeatureStructRefMut,
    FeatureStructRef, FeatureInner, FeatureUpcast, FeatureUpcastRefMut, FeatureUpcastRef,
    FeatureMethodsDescendants, FeatureRefMutMethodsDescendants,
    FeatureRefMethodsDescendants,
};
use super::type_::{
    Type, TypeRefMut, TypeRef, TypeStruct, TypeStructRefMut, TypeStructRef, TypeInner,
    TypeUpcast, TypeUpcastRefMut, TypeUpcastRef, TypeMethodsDescendants,
    TypeRefMutMethodsDescendants, TypeRefMethodsDescendants,
};
use super::namespace::{
    Namespace, NamespaceRefMut, NamespaceRef, NamespaceStruct, NamespaceStructRefMut,
    NamespaceStructRef, NamespaceInner, NamespaceUpcast, NamespaceUpcastRefMut,
    NamespaceUpcastRef, NamespaceMethodsDescendants, NamespaceRefMutMethodsDescendants,
    NamespaceRefMethodsDescendants,
};
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
use super::relationship::{
    Relationship, RelationshipRefMut, RelationshipRef, RelationshipStruct,
    RelationshipStructRefMut, RelationshipStructRef, RelationshipInner,
    RelationshipUpcast, RelationshipUpcastRefMut, RelationshipUpcastRef,
    RelationshipMethodsDescendants, RelationshipRefMutMethodsDescendants,
    RelationshipRefMethodsDescendants,
};
pub struct BindingConnectorInner {
    pub(super) sup_connector: ConnectorInner,
}
pub trait BindingConnectorStruct
where
    Self: BindingConnectorStructRefMut,
    Self: BindingConnectorStructRef,
    Self: ConnectorStruct,
{}
pub trait BindingConnectorStructRefMut
where
    Self: BindingConnectorStructRef,
    Self: ConnectorStructRefMut,
{}
pub trait BindingConnectorStructRef
where
    Self: ConnectorStructRef,
{}
pub trait BindingConnectorUpcast: BindingConnectorStruct {
    fn into_binding_connector(self) -> BindingConnector;
}
pub trait BindingConnectorUpcastRefMut<'a>: BindingConnectorStructRefMut {
    fn as_binding_connector_ref_mut(self) -> BindingConnectorRefMut<'a>;
}
pub trait BindingConnectorUpcastRef<'a>: BindingConnectorStructRef {
    fn as_binding_connector_ref(self) -> BindingConnectorRef<'a>;
}
impl BindingConnectorStruct for BindingConnectorInner {}
impl BindingConnectorStructRefMut for BindingConnectorInner {}
impl BindingConnectorStructRef for BindingConnectorInner {}
impl ConnectorStruct for BindingConnectorInner {}
impl ConnectorStructRefMut for BindingConnectorInner {}
impl ConnectorStructRef for BindingConnectorInner {}
impl FeatureStruct for BindingConnectorInner {
    fn is_unique(self) -> bool {
        self.sup_connector.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_connector.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_connector.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_connector.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_connector.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_connector.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_connector.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_connector.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_connector.direction()
    }
}
impl FeatureStructRefMut for BindingConnectorInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_connector.direction_ref_mut()
    }
}
impl FeatureStructRef for BindingConnectorInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_connector.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_connector.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_connector.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_connector.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_connector.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_connector.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_connector.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_connector.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_connector.direction_ref()
    }
}
impl TypeStruct for BindingConnectorInner {
    fn is_abstract(self) -> bool {
        self.sup_connector.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_connector.is_sufficient()
    }
}
impl TypeStructRefMut for BindingConnectorInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for BindingConnectorInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_connector.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_connector.is_sufficient_ref()
    }
}
impl NamespaceStruct for BindingConnectorInner {}
impl NamespaceStructRefMut for BindingConnectorInner {}
impl NamespaceStructRef for BindingConnectorInner {}
impl ElementStruct for BindingConnectorInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_connector.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_connector.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_connector.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_connector.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_connector.is_implied_included()
    }
}
impl ElementStructRefMut for BindingConnectorInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_connector.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_connector.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_connector.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_connector.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_connector.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for BindingConnectorInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_connector.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_connector.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_connector.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_connector.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_connector.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_connector.is_implied_included_ref()
    }
}
impl RelationshipStruct for BindingConnectorInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_connector.is_implied()
    }
}
impl RelationshipStructRefMut for BindingConnectorInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for BindingConnectorInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_connector.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_connector.is_implied_ref()
    }
}
pub enum BindingConnector {
    Itself(BindingConnectorInner),
}
pub enum BindingConnectorRefMut<'a> {
    Itself(&'a mut BindingConnectorInner),
}
pub enum BindingConnectorRef<'a> {
    Itself(&'a BindingConnectorInner),
}
impl BindingConnector {
    pub fn as_ref(&self) -> BindingConnectorRef {
        match self {
            BindingConnector::Itself(inner) => BindingConnectorRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> BindingConnectorRefMut {
        match self {
            BindingConnector::Itself(inner) => BindingConnectorRefMut::Itself(inner),
        }
    }
}
impl<'a> BindingConnectorRefMut<'a> {
    pub fn as_ref(self) -> BindingConnectorRef<'a> {
        match self {
            BindingConnectorRefMut::Itself(inner) => BindingConnectorRef::Itself(inner),
        }
    }
}
impl BindingConnectorStruct for BindingConnector {}
impl BindingConnectorStructRefMut for BindingConnector {}
impl BindingConnectorStructRef for BindingConnector {}
impl<'a> BindingConnectorStructRefMut for BindingConnectorRefMut<'a> {}
impl<'a> BindingConnectorStructRef for BindingConnectorRefMut<'a> {}
impl<'a> BindingConnectorStructRef for BindingConnectorRef<'a> {}
impl ConnectorStruct for BindingConnector {}
impl ConnectorStructRefMut for BindingConnector {}
impl ConnectorStructRef for BindingConnector {}
impl<'a> ConnectorStructRefMut for BindingConnectorRefMut<'a> {}
impl<'a> ConnectorStructRef for BindingConnectorRefMut<'a> {}
impl<'a> ConnectorStructRef for BindingConnectorRef<'a> {}
impl FeatureStruct for BindingConnector {
    fn is_unique(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_constant(),
        }
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            BindingConnector::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for BindingConnector {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_constant_ref_mut(),
        }
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            BindingConnector::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for BindingConnector {
    fn is_unique_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_constant_ref(),
        }
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            BindingConnector::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for BindingConnectorRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_constant_ref_mut(),
        }
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            BindingConnectorRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for BindingConnectorRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_constant_ref(),
        }
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            BindingConnectorRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for BindingConnectorRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_constant_ref(),
        }
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        match self {
            BindingConnectorRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for BindingConnector {
    fn is_abstract(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for BindingConnector {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for BindingConnector {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for BindingConnectorRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for BindingConnectorRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for BindingConnectorRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for BindingConnector {}
impl NamespaceStructRefMut for BindingConnector {}
impl NamespaceStructRef for BindingConnector {}
impl<'a> NamespaceStructRefMut for BindingConnectorRefMut<'a> {}
impl<'a> NamespaceStructRef for BindingConnectorRefMut<'a> {}
impl<'a> NamespaceStructRef for BindingConnectorRef<'a> {}
impl ElementStruct for BindingConnector {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnector::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnector::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            BindingConnector::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            BindingConnector::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            BindingConnector::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            BindingConnector::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for BindingConnector {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnector::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            BindingConnector::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            BindingConnector::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            BindingConnector::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BindingConnector::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BindingConnector::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for BindingConnector {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnector::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnector::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BindingConnector::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BindingConnector::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BindingConnector::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BindingConnector::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for BindingConnectorRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            BindingConnectorRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for BindingConnectorRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BindingConnectorRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for BindingConnectorRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnectorRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            BindingConnectorRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            BindingConnectorRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            BindingConnectorRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            BindingConnectorRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            BindingConnectorRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for BindingConnector {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            BindingConnector::Itself(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for BindingConnector {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnector::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for BindingConnector {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnector::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            BindingConnector::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for BindingConnectorRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for BindingConnectorRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRefMut::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            BindingConnectorRefMut::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for BindingConnectorRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRef::Itself(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRef::Itself(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRef::Itself(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            BindingConnectorRef::Itself(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            BindingConnectorRef::Itself(x) => x.is_implied_ref(),
        }
    }
}
impl BindingConnectorUpcast for BindingConnector {
    fn into_binding_connector(self) -> BindingConnector {
        self
    }
}
impl<'a> BindingConnectorUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_binding_connector_ref_mut(self) -> BindingConnectorRefMut<'a> {
        self
    }
}
impl<'a> BindingConnectorUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_binding_connector_ref(self) -> BindingConnectorRef<'a> {
        self
    }
}
impl ConnectorUpcast for BindingConnector {
    fn into_connector(self) -> Connector {
        Connector::BindingConnector(self).into_connector()
    }
}
impl<'a> ConnectorUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_connector_ref_mut(self) -> ConnectorRefMut<'a> {
        ConnectorRefMut::BindingConnector(self).as_connector_ref_mut()
    }
}
impl<'a> ConnectorUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_connector_ref(self) -> ConnectorRef<'a> {
        ConnectorRef::BindingConnector(self).as_connector_ref()
    }
}
impl FeatureUpcast for BindingConnector {
    fn into_feature(self) -> Feature {
        Connector::BindingConnector(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ConnectorRefMut::BindingConnector(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ConnectorRef::BindingConnector(self).as_feature_ref()
    }
}
impl TypeUpcast for BindingConnector {
    fn into_type_(self) -> Type {
        Connector::BindingConnector(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ConnectorRefMut::BindingConnector(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ConnectorRef::BindingConnector(self).as_type_ref()
    }
}
impl NamespaceUpcast for BindingConnector {
    fn into_namespace(self) -> Namespace {
        Connector::BindingConnector(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ConnectorRefMut::BindingConnector(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ConnectorRef::BindingConnector(self).as_namespace_ref()
    }
}
impl ElementUpcast for BindingConnector {
    fn into_element(self) -> Element {
        Connector::BindingConnector(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ConnectorRefMut::BindingConnector(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ConnectorRef::BindingConnector(self).as_element_ref()
    }
}
impl RelationshipUpcast for BindingConnector {
    fn into_relationship(self) -> Relationship {
        Connector::BindingConnector(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for BindingConnectorRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        ConnectorRefMut::BindingConnector(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for BindingConnectorRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        ConnectorRef::BindingConnector(self).as_relationship_ref()
    }
}
pub trait BindingConnectorDowncast {}
pub trait BindingConnectorDowncastRefMut<'a> {}
pub trait BindingConnectorDowncastRef<'a> {}
impl BindingConnectorDowncast for BindingConnector {}
impl<'a> BindingConnectorDowncastRefMut<'a> for BindingConnectorRefMut<'a> {}
impl<'a> BindingConnectorDowncastRef<'a> for BindingConnectorRef<'a> {}
pub trait BindingConnectorMethodsDescendants
where
    Self: DescendantOf<BindingConnector>,
    Self::Via: BindingConnectorMethods,
    Self: Sized,
{}
pub trait BindingConnectorMethods: Sized {}
impl<T: BindingConnectorMethodsDescendants> BindingConnectorMethods for T
where
    T::Via: BindingConnectorMethods,
{}
impl DescendantOf<Connector> for BindingConnector {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl ConnectorMethodsDescendants for BindingConnector {}
impl DescendantOf<Feature> for BindingConnector {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl FeatureMethodsDescendants for BindingConnector {}
impl DescendantOf<Type> for BindingConnector {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl TypeMethodsDescendants for BindingConnector {}
impl DescendantOf<Namespace> for BindingConnector {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl NamespaceMethodsDescendants for BindingConnector {}
impl DescendantOf<Element> for BindingConnector {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl ElementMethodsDescendants for BindingConnector {}
impl DescendantOf<Relationship> for BindingConnector {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl RelationshipMethodsDescendants for BindingConnector {}
pub trait BindingConnectorRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<BindingConnectorRefMut<'a>>,
    Self::Via: BindingConnectorRefMutMethods,
    Self: Sized,
{}
pub trait BindingConnectorRefMutMethods: Sized {}
impl<'a, T: BindingConnectorRefMutMethodsDescendants<'a>> BindingConnectorRefMutMethods
for T
where
    T::Via: BindingConnectorRefMutMethods,
{}
impl<'a> DescendantOf<ConnectorRefMut<'a>> for BindingConnectorRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> ConnectorRefMutMethodsDescendants<'a> for BindingConnectorRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for BindingConnectorRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for BindingConnectorRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for BindingConnectorRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for BindingConnectorRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for BindingConnectorRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for BindingConnectorRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for BindingConnectorRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for BindingConnectorRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for BindingConnectorRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for BindingConnectorRefMut<'a> {}
pub trait BindingConnectorRefMethodsDescendants<'a>
where
    Self: DescendantOf<BindingConnectorRef<'a>>,
    Self::Via: BindingConnectorRefMethods,
    Self: Sized,
{}
pub trait BindingConnectorRefMethods: Sized {}
impl<'a, T: BindingConnectorRefMethodsDescendants<'a>> BindingConnectorRefMethods for T
where
    T::Via: BindingConnectorRefMethods,
{}
impl<'a> DescendantOf<ConnectorRef<'a>> for BindingConnectorRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> ConnectorRefMethodsDescendants<'a> for BindingConnectorRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for BindingConnectorRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for BindingConnectorRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for BindingConnectorRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for BindingConnectorRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for BindingConnectorRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for BindingConnectorRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for BindingConnectorRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for BindingConnectorRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for BindingConnectorRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for BindingConnectorRef<'a> {}

