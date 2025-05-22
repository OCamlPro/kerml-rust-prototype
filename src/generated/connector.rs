#![allow(unused)]
use super::utils::DescendantOf;
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
use super::succession::{Succession, SuccessionRefMut, SuccessionRef};
use super::flow::{Flow, FlowRefMut, FlowRef};
use super::binding_connector::{
    BindingConnector, BindingConnectorRefMut, BindingConnectorRef,
};
pub struct ConnectorInner {
    pub(super) sup_relationship: RelationshipInner,
    pub(super) sup_feature: FeatureInner,
}
pub trait ConnectorStruct
where
    Self: ConnectorStructRefMut,
    Self: ConnectorStructRef,
    Self: RelationshipStruct,
    Self: FeatureStruct,
{}
pub trait ConnectorStructRefMut
where
    Self: ConnectorStructRef,
    Self: RelationshipStructRefMut,
    Self: FeatureStructRefMut,
{}
pub trait ConnectorStructRef
where
    Self: RelationshipStructRef,
    Self: FeatureStructRef,
{}
pub trait ConnectorUpcast: ConnectorStruct {
    fn into_connector(self) -> Connector;
}
pub trait ConnectorUpcastRefMut<'a>: ConnectorStructRefMut {
    fn as_connector_ref_mut(self) -> ConnectorRefMut<'a>;
}
pub trait ConnectorUpcastRef<'a>: ConnectorStructRef {
    fn as_connector_ref(self) -> ConnectorRef<'a>;
}
impl ConnectorStruct for ConnectorInner {}
impl ConnectorStructRefMut for ConnectorInner {}
impl ConnectorStructRef for ConnectorInner {}
impl FeatureStruct for ConnectorInner {
    fn is_unique(self) -> bool {
        self.sup_feature.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_feature.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_feature.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_feature.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_feature.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_feature.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_feature.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_feature.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_feature.direction()
    }
}
impl FeatureStructRefMut for ConnectorInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_feature.direction_ref_mut()
    }
}
impl FeatureStructRef for ConnectorInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_feature.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_feature.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_feature.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_feature.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_feature.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_feature.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_feature.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_feature.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_feature.direction_ref()
    }
}
impl TypeStruct for ConnectorInner {
    fn is_abstract(self) -> bool {
        self.sup_feature.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_feature.is_sufficient()
    }
}
impl TypeStructRefMut for ConnectorInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for ConnectorInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_feature.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_feature.is_sufficient_ref()
    }
}
impl NamespaceStruct for ConnectorInner {}
impl NamespaceStructRefMut for ConnectorInner {}
impl NamespaceStructRef for ConnectorInner {}
impl ElementStruct for ConnectorInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_feature.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_feature.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_feature.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_feature.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_feature.is_implied_included()
    }
}
impl ElementStructRefMut for ConnectorInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_feature.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_feature.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_feature.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_feature.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for ConnectorInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_feature.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_feature.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_feature.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_feature.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_feature.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_feature.is_implied_included_ref()
    }
}
impl RelationshipStruct for ConnectorInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.target()
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.source()
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owning_related_element()
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owned_related_element()
    }
    fn is_implied(self) -> bool {
        self.sup_relationship.is_implied()
    }
}
impl RelationshipStructRefMut for ConnectorInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.target_ref_mut()
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.source_ref_mut()
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owning_related_element_ref_mut()
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owned_related_element_ref_mut()
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        self.sup_relationship.is_implied_ref_mut()
    }
}
impl RelationshipStructRef for ConnectorInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.target_ref()
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.source_ref()
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owning_related_element_ref()
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.sup_relationship.owned_related_element_ref()
    }
    fn is_implied_ref(&self) -> &bool {
        self.sup_relationship.is_implied_ref()
    }
}
pub enum Connector {
    Itself(ConnectorInner),
    Succession(Succession),
    Flow(Flow),
    BindingConnector(BindingConnector),
}
pub enum ConnectorRefMut<'a> {
    Itself(&'a mut ConnectorInner),
    Succession(SuccessionRefMut<'a>),
    Flow(FlowRefMut<'a>),
    BindingConnector(BindingConnectorRefMut<'a>),
}
pub enum ConnectorRef<'a> {
    Itself(&'a ConnectorInner),
    Succession(SuccessionRef<'a>),
    Flow(FlowRef<'a>),
    BindingConnector(BindingConnectorRef<'a>),
}
impl Connector {
    pub fn as_ref(&self) -> ConnectorRef {
        match self {
            Connector::Itself(inner) => ConnectorRef::Itself(&inner),
            Connector::Succession(inner) => ConnectorRef::Succession(inner.as_ref()),
            Connector::Flow(inner) => ConnectorRef::Flow(inner.as_ref()),
            Connector::BindingConnector(inner) => {
                ConnectorRef::BindingConnector(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ConnectorRefMut {
        match self {
            Connector::Itself(inner) => ConnectorRefMut::Itself(inner),
            Connector::Succession(inner) => {
                ConnectorRefMut::Succession(inner.as_ref_mut())
            }
            Connector::Flow(inner) => ConnectorRefMut::Flow(inner.as_ref_mut()),
            Connector::BindingConnector(inner) => {
                ConnectorRefMut::BindingConnector(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> ConnectorRefMut<'a> {
    pub fn as_ref(self) -> ConnectorRef<'a> {
        match self {
            ConnectorRefMut::Itself(inner) => ConnectorRef::Itself(inner),
            ConnectorRefMut::Succession(inner) => {
                ConnectorRef::Succession(inner.as_ref())
            }
            ConnectorRefMut::Flow(inner) => ConnectorRef::Flow(inner.as_ref()),
            ConnectorRefMut::BindingConnector(inner) => {
                ConnectorRef::BindingConnector(inner.as_ref())
            }
        }
    }
}
impl ConnectorStruct for Connector {}
impl ConnectorStructRefMut for Connector {}
impl ConnectorStructRef for Connector {}
impl<'a> ConnectorStructRefMut for ConnectorRefMut<'a> {}
impl<'a> ConnectorStructRef for ConnectorRefMut<'a> {}
impl<'a> ConnectorStructRef for ConnectorRef<'a> {}
impl FeatureStruct for Connector {
    fn is_unique(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_unique(),
            Connector::Succession(x) => x.is_unique(),
            Connector::Flow(x) => x.is_unique(),
            Connector::BindingConnector(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_ordered(),
            Connector::Succession(x) => x.is_ordered(),
            Connector::Flow(x) => x.is_ordered(),
            Connector::BindingConnector(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_composite(),
            Connector::Succession(x) => x.is_composite(),
            Connector::Flow(x) => x.is_composite(),
            Connector::BindingConnector(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_end(),
            Connector::Succession(x) => x.is_end(),
            Connector::Flow(x) => x.is_end(),
            Connector::BindingConnector(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_derived(),
            Connector::Succession(x) => x.is_derived(),
            Connector::Flow(x) => x.is_derived(),
            Connector::BindingConnector(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_portion(),
            Connector::Succession(x) => x.is_portion(),
            Connector::Flow(x) => x.is_portion(),
            Connector::BindingConnector(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_variable(),
            Connector::Succession(x) => x.is_variable(),
            Connector::Flow(x) => x.is_variable(),
            Connector::BindingConnector(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_constant(),
            Connector::Succession(x) => x.is_constant(),
            Connector::Flow(x) => x.is_constant(),
            Connector::BindingConnector(x) => x.is_constant(),
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
            Connector::Itself(x) => x.direction(),
            Connector::Succession(x) => x.direction(),
            Connector::Flow(x) => x.direction(),
            Connector::BindingConnector(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Connector {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_unique_ref_mut(),
            Connector::Succession(x) => x.is_unique_ref_mut(),
            Connector::Flow(x) => x.is_unique_ref_mut(),
            Connector::BindingConnector(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_ordered_ref_mut(),
            Connector::Succession(x) => x.is_ordered_ref_mut(),
            Connector::Flow(x) => x.is_ordered_ref_mut(),
            Connector::BindingConnector(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_composite_ref_mut(),
            Connector::Succession(x) => x.is_composite_ref_mut(),
            Connector::Flow(x) => x.is_composite_ref_mut(),
            Connector::BindingConnector(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_end_ref_mut(),
            Connector::Succession(x) => x.is_end_ref_mut(),
            Connector::Flow(x) => x.is_end_ref_mut(),
            Connector::BindingConnector(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_derived_ref_mut(),
            Connector::Succession(x) => x.is_derived_ref_mut(),
            Connector::Flow(x) => x.is_derived_ref_mut(),
            Connector::BindingConnector(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_portion_ref_mut(),
            Connector::Succession(x) => x.is_portion_ref_mut(),
            Connector::Flow(x) => x.is_portion_ref_mut(),
            Connector::BindingConnector(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_variable_ref_mut(),
            Connector::Succession(x) => x.is_variable_ref_mut(),
            Connector::Flow(x) => x.is_variable_ref_mut(),
            Connector::BindingConnector(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_constant_ref_mut(),
            Connector::Succession(x) => x.is_constant_ref_mut(),
            Connector::Flow(x) => x.is_constant_ref_mut(),
            Connector::BindingConnector(x) => x.is_constant_ref_mut(),
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
            Connector::Itself(x) => x.direction_ref_mut(),
            Connector::Succession(x) => x.direction_ref_mut(),
            Connector::Flow(x) => x.direction_ref_mut(),
            Connector::BindingConnector(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Connector {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_unique_ref(),
            Connector::Succession(x) => x.is_unique_ref(),
            Connector::Flow(x) => x.is_unique_ref(),
            Connector::BindingConnector(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_ordered_ref(),
            Connector::Succession(x) => x.is_ordered_ref(),
            Connector::Flow(x) => x.is_ordered_ref(),
            Connector::BindingConnector(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_composite_ref(),
            Connector::Succession(x) => x.is_composite_ref(),
            Connector::Flow(x) => x.is_composite_ref(),
            Connector::BindingConnector(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_end_ref(),
            Connector::Succession(x) => x.is_end_ref(),
            Connector::Flow(x) => x.is_end_ref(),
            Connector::BindingConnector(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_derived_ref(),
            Connector::Succession(x) => x.is_derived_ref(),
            Connector::Flow(x) => x.is_derived_ref(),
            Connector::BindingConnector(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_portion_ref(),
            Connector::Succession(x) => x.is_portion_ref(),
            Connector::Flow(x) => x.is_portion_ref(),
            Connector::BindingConnector(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_variable_ref(),
            Connector::Succession(x) => x.is_variable_ref(),
            Connector::Flow(x) => x.is_variable_ref(),
            Connector::BindingConnector(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_constant_ref(),
            Connector::Succession(x) => x.is_constant_ref(),
            Connector::Flow(x) => x.is_constant_ref(),
            Connector::BindingConnector(x) => x.is_constant_ref(),
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
            Connector::Itself(x) => x.direction_ref(),
            Connector::Succession(x) => x.direction_ref(),
            Connector::Flow(x) => x.direction_ref(),
            Connector::BindingConnector(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for ConnectorRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_unique_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_unique_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_unique_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_ordered_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_ordered_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_ordered_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_composite_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_composite_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_composite_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_end_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_end_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_end_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_derived_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_derived_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_derived_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_portion_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_portion_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_portion_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_variable_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_variable_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_variable_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_constant_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_constant_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_constant_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_constant_ref_mut(),
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
            ConnectorRefMut::Itself(x) => x.direction_ref_mut(),
            ConnectorRefMut::Succession(x) => x.direction_ref_mut(),
            ConnectorRefMut::Flow(x) => x.direction_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for ConnectorRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_unique_ref(),
            ConnectorRefMut::Succession(x) => x.is_unique_ref(),
            ConnectorRefMut::Flow(x) => x.is_unique_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_ordered_ref(),
            ConnectorRefMut::Succession(x) => x.is_ordered_ref(),
            ConnectorRefMut::Flow(x) => x.is_ordered_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_composite_ref(),
            ConnectorRefMut::Succession(x) => x.is_composite_ref(),
            ConnectorRefMut::Flow(x) => x.is_composite_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_end_ref(),
            ConnectorRefMut::Succession(x) => x.is_end_ref(),
            ConnectorRefMut::Flow(x) => x.is_end_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_derived_ref(),
            ConnectorRefMut::Succession(x) => x.is_derived_ref(),
            ConnectorRefMut::Flow(x) => x.is_derived_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_portion_ref(),
            ConnectorRefMut::Succession(x) => x.is_portion_ref(),
            ConnectorRefMut::Flow(x) => x.is_portion_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_variable_ref(),
            ConnectorRefMut::Succession(x) => x.is_variable_ref(),
            ConnectorRefMut::Flow(x) => x.is_variable_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_constant_ref(),
            ConnectorRefMut::Succession(x) => x.is_constant_ref(),
            ConnectorRefMut::Flow(x) => x.is_constant_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_constant_ref(),
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
            ConnectorRefMut::Itself(x) => x.direction_ref(),
            ConnectorRefMut::Succession(x) => x.direction_ref(),
            ConnectorRefMut::Flow(x) => x.direction_ref(),
            ConnectorRefMut::BindingConnector(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for ConnectorRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_unique_ref(),
            ConnectorRef::Succession(x) => x.is_unique_ref(),
            ConnectorRef::Flow(x) => x.is_unique_ref(),
            ConnectorRef::BindingConnector(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_ordered_ref(),
            ConnectorRef::Succession(x) => x.is_ordered_ref(),
            ConnectorRef::Flow(x) => x.is_ordered_ref(),
            ConnectorRef::BindingConnector(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_composite_ref(),
            ConnectorRef::Succession(x) => x.is_composite_ref(),
            ConnectorRef::Flow(x) => x.is_composite_ref(),
            ConnectorRef::BindingConnector(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_end_ref(),
            ConnectorRef::Succession(x) => x.is_end_ref(),
            ConnectorRef::Flow(x) => x.is_end_ref(),
            ConnectorRef::BindingConnector(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_derived_ref(),
            ConnectorRef::Succession(x) => x.is_derived_ref(),
            ConnectorRef::Flow(x) => x.is_derived_ref(),
            ConnectorRef::BindingConnector(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_portion_ref(),
            ConnectorRef::Succession(x) => x.is_portion_ref(),
            ConnectorRef::Flow(x) => x.is_portion_ref(),
            ConnectorRef::BindingConnector(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_variable_ref(),
            ConnectorRef::Succession(x) => x.is_variable_ref(),
            ConnectorRef::Flow(x) => x.is_variable_ref(),
            ConnectorRef::BindingConnector(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_constant_ref(),
            ConnectorRef::Succession(x) => x.is_constant_ref(),
            ConnectorRef::Flow(x) => x.is_constant_ref(),
            ConnectorRef::BindingConnector(x) => x.is_constant_ref(),
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
            ConnectorRef::Itself(x) => x.direction_ref(),
            ConnectorRef::Succession(x) => x.direction_ref(),
            ConnectorRef::Flow(x) => x.direction_ref(),
            ConnectorRef::BindingConnector(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Connector {
    fn is_abstract(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_abstract(),
            Connector::Succession(x) => x.is_abstract(),
            Connector::Flow(x) => x.is_abstract(),
            Connector::BindingConnector(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_sufficient(),
            Connector::Succession(x) => x.is_sufficient(),
            Connector::Flow(x) => x.is_sufficient(),
            Connector::BindingConnector(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Connector {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_abstract_ref_mut(),
            Connector::Succession(x) => x.is_abstract_ref_mut(),
            Connector::Flow(x) => x.is_abstract_ref_mut(),
            Connector::BindingConnector(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_sufficient_ref_mut(),
            Connector::Succession(x) => x.is_sufficient_ref_mut(),
            Connector::Flow(x) => x.is_sufficient_ref_mut(),
            Connector::BindingConnector(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Connector {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_abstract_ref(),
            Connector::Succession(x) => x.is_abstract_ref(),
            Connector::Flow(x) => x.is_abstract_ref(),
            Connector::BindingConnector(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_sufficient_ref(),
            Connector::Succession(x) => x.is_sufficient_ref(),
            Connector::Flow(x) => x.is_sufficient_ref(),
            Connector::BindingConnector(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for ConnectorRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_abstract_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_abstract_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_abstract_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_sufficient_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_sufficient_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for ConnectorRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_abstract_ref(),
            ConnectorRefMut::Succession(x) => x.is_abstract_ref(),
            ConnectorRefMut::Flow(x) => x.is_abstract_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_sufficient_ref(),
            ConnectorRefMut::Succession(x) => x.is_sufficient_ref(),
            ConnectorRefMut::Flow(x) => x.is_sufficient_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for ConnectorRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_abstract_ref(),
            ConnectorRef::Succession(x) => x.is_abstract_ref(),
            ConnectorRef::Flow(x) => x.is_abstract_ref(),
            ConnectorRef::BindingConnector(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_sufficient_ref(),
            ConnectorRef::Succession(x) => x.is_sufficient_ref(),
            ConnectorRef::Flow(x) => x.is_sufficient_ref(),
            ConnectorRef::BindingConnector(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Connector {}
impl NamespaceStructRefMut for Connector {}
impl NamespaceStructRef for Connector {}
impl<'a> NamespaceStructRefMut for ConnectorRefMut<'a> {}
impl<'a> NamespaceStructRef for ConnectorRefMut<'a> {}
impl<'a> NamespaceStructRef for ConnectorRef<'a> {}
impl ElementStruct for Connector {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Connector::Itself(x) => x.owned_relationship(),
            Connector::Succession(x) => x.owned_relationship(),
            Connector::Flow(x) => x.owned_relationship(),
            Connector::BindingConnector(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Connector::Itself(x) => x.owning_relationship(),
            Connector::Succession(x) => x.owning_relationship(),
            Connector::Flow(x) => x.owning_relationship(),
            Connector::BindingConnector(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Connector::Itself(x) => x.element_id(),
            Connector::Succession(x) => x.element_id(),
            Connector::Flow(x) => x.element_id(),
            Connector::BindingConnector(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Connector::Itself(x) => x.alias_ids(),
            Connector::Succession(x) => x.alias_ids(),
            Connector::Flow(x) => x.alias_ids(),
            Connector::BindingConnector(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Connector::Itself(x) => x.declared_short_name(),
            Connector::Succession(x) => x.declared_short_name(),
            Connector::Flow(x) => x.declared_short_name(),
            Connector::BindingConnector(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Connector::Itself(x) => x.declared_name(),
            Connector::Succession(x) => x.declared_name(),
            Connector::Flow(x) => x.declared_name(),
            Connector::BindingConnector(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_implied_included(),
            Connector::Succession(x) => x.is_implied_included(),
            Connector::Flow(x) => x.is_implied_included(),
            Connector::BindingConnector(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Connector {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Connector::Itself(x) => x.owned_relationship_ref_mut(),
            Connector::Succession(x) => x.owned_relationship_ref_mut(),
            Connector::Flow(x) => x.owned_relationship_ref_mut(),
            Connector::BindingConnector(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Connector::Itself(x) => x.owning_relationship_ref_mut(),
            Connector::Succession(x) => x.owning_relationship_ref_mut(),
            Connector::Flow(x) => x.owning_relationship_ref_mut(),
            Connector::BindingConnector(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Connector::Itself(x) => x.element_id_ref_mut(),
            Connector::Succession(x) => x.element_id_ref_mut(),
            Connector::Flow(x) => x.element_id_ref_mut(),
            Connector::BindingConnector(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Connector::Itself(x) => x.alias_ids_ref_mut(),
            Connector::Succession(x) => x.alias_ids_ref_mut(),
            Connector::Flow(x) => x.alias_ids_ref_mut(),
            Connector::BindingConnector(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Connector::Itself(x) => x.declared_short_name_ref_mut(),
            Connector::Succession(x) => x.declared_short_name_ref_mut(),
            Connector::Flow(x) => x.declared_short_name_ref_mut(),
            Connector::BindingConnector(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Connector::Itself(x) => x.declared_name_ref_mut(),
            Connector::Succession(x) => x.declared_name_ref_mut(),
            Connector::Flow(x) => x.declared_name_ref_mut(),
            Connector::BindingConnector(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_implied_included_ref_mut(),
            Connector::Succession(x) => x.is_implied_included_ref_mut(),
            Connector::Flow(x) => x.is_implied_included_ref_mut(),
            Connector::BindingConnector(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Connector {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Connector::Itself(x) => x.owned_relationship_ref(),
            Connector::Succession(x) => x.owned_relationship_ref(),
            Connector::Flow(x) => x.owned_relationship_ref(),
            Connector::BindingConnector(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Connector::Itself(x) => x.owning_relationship_ref(),
            Connector::Succession(x) => x.owning_relationship_ref(),
            Connector::Flow(x) => x.owning_relationship_ref(),
            Connector::BindingConnector(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Connector::Itself(x) => x.element_id_ref(),
            Connector::Succession(x) => x.element_id_ref(),
            Connector::Flow(x) => x.element_id_ref(),
            Connector::BindingConnector(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Connector::Itself(x) => x.alias_ids_ref(),
            Connector::Succession(x) => x.alias_ids_ref(),
            Connector::Flow(x) => x.alias_ids_ref(),
            Connector::BindingConnector(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Connector::Itself(x) => x.declared_short_name_ref(),
            Connector::Succession(x) => x.declared_short_name_ref(),
            Connector::Flow(x) => x.declared_short_name_ref(),
            Connector::BindingConnector(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Connector::Itself(x) => x.declared_name_ref(),
            Connector::Succession(x) => x.declared_name_ref(),
            Connector::Flow(x) => x.declared_name_ref(),
            Connector::BindingConnector(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_implied_included_ref(),
            Connector::Succession(x) => x.is_implied_included_ref(),
            Connector::Flow(x) => x.is_implied_included_ref(),
            Connector::BindingConnector(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ConnectorRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ConnectorRefMut::Succession(x) => x.owned_relationship_ref_mut(),
            ConnectorRefMut::Flow(x) => x.owned_relationship_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ConnectorRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ConnectorRefMut::Succession(x) => x.owning_relationship_ref_mut(),
            ConnectorRefMut::Flow(x) => x.owning_relationship_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ConnectorRefMut::Itself(x) => x.element_id_ref_mut(),
            ConnectorRefMut::Succession(x) => x.element_id_ref_mut(),
            ConnectorRefMut::Flow(x) => x.element_id_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ConnectorRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ConnectorRefMut::Succession(x) => x.alias_ids_ref_mut(),
            ConnectorRefMut::Flow(x) => x.alias_ids_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConnectorRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ConnectorRefMut::Succession(x) => x.declared_short_name_ref_mut(),
            ConnectorRefMut::Flow(x) => x.declared_short_name_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ConnectorRefMut::Itself(x) => x.declared_name_ref_mut(),
            ConnectorRefMut::Succession(x) => x.declared_name_ref_mut(),
            ConnectorRefMut::Flow(x) => x.declared_name_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_implied_included_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_implied_included_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ConnectorRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owned_relationship_ref(),
            ConnectorRefMut::Succession(x) => x.owned_relationship_ref(),
            ConnectorRefMut::Flow(x) => x.owned_relationship_ref(),
            ConnectorRefMut::BindingConnector(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owning_relationship_ref(),
            ConnectorRefMut::Succession(x) => x.owning_relationship_ref(),
            ConnectorRefMut::Flow(x) => x.owning_relationship_ref(),
            ConnectorRefMut::BindingConnector(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConnectorRefMut::Itself(x) => x.element_id_ref(),
            ConnectorRefMut::Succession(x) => x.element_id_ref(),
            ConnectorRefMut::Flow(x) => x.element_id_ref(),
            ConnectorRefMut::BindingConnector(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConnectorRefMut::Itself(x) => x.alias_ids_ref(),
            ConnectorRefMut::Succession(x) => x.alias_ids_ref(),
            ConnectorRefMut::Flow(x) => x.alias_ids_ref(),
            ConnectorRefMut::BindingConnector(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConnectorRefMut::Itself(x) => x.declared_short_name_ref(),
            ConnectorRefMut::Succession(x) => x.declared_short_name_ref(),
            ConnectorRefMut::Flow(x) => x.declared_short_name_ref(),
            ConnectorRefMut::BindingConnector(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConnectorRefMut::Itself(x) => x.declared_name_ref(),
            ConnectorRefMut::Succession(x) => x.declared_name_ref(),
            ConnectorRefMut::Flow(x) => x.declared_name_ref(),
            ConnectorRefMut::BindingConnector(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_implied_included_ref(),
            ConnectorRefMut::Succession(x) => x.is_implied_included_ref(),
            ConnectorRefMut::Flow(x) => x.is_implied_included_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ConnectorRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConnectorRef::Itself(x) => x.owned_relationship_ref(),
            ConnectorRef::Succession(x) => x.owned_relationship_ref(),
            ConnectorRef::Flow(x) => x.owned_relationship_ref(),
            ConnectorRef::BindingConnector(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ConnectorRef::Itself(x) => x.owning_relationship_ref(),
            ConnectorRef::Succession(x) => x.owning_relationship_ref(),
            ConnectorRef::Flow(x) => x.owning_relationship_ref(),
            ConnectorRef::BindingConnector(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ConnectorRef::Itself(x) => x.element_id_ref(),
            ConnectorRef::Succession(x) => x.element_id_ref(),
            ConnectorRef::Flow(x) => x.element_id_ref(),
            ConnectorRef::BindingConnector(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ConnectorRef::Itself(x) => x.alias_ids_ref(),
            ConnectorRef::Succession(x) => x.alias_ids_ref(),
            ConnectorRef::Flow(x) => x.alias_ids_ref(),
            ConnectorRef::BindingConnector(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ConnectorRef::Itself(x) => x.declared_short_name_ref(),
            ConnectorRef::Succession(x) => x.declared_short_name_ref(),
            ConnectorRef::Flow(x) => x.declared_short_name_ref(),
            ConnectorRef::BindingConnector(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ConnectorRef::Itself(x) => x.declared_name_ref(),
            ConnectorRef::Succession(x) => x.declared_name_ref(),
            ConnectorRef::Flow(x) => x.declared_name_ref(),
            ConnectorRef::BindingConnector(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_implied_included_ref(),
            ConnectorRef::Succession(x) => x.is_implied_included_ref(),
            ConnectorRef::Flow(x) => x.is_implied_included_ref(),
            ConnectorRef::BindingConnector(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for Connector {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.target(),
            Connector::Succession(x) => x.target(),
            Connector::Flow(x) => x.target(),
            Connector::BindingConnector(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.source(),
            Connector::Succession(x) => x.source(),
            Connector::Flow(x) => x.source(),
            Connector::BindingConnector(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.owning_related_element(),
            Connector::Succession(x) => x.owning_related_element(),
            Connector::Flow(x) => x.owning_related_element(),
            Connector::BindingConnector(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.owned_related_element(),
            Connector::Succession(x) => x.owned_related_element(),
            Connector::Flow(x) => x.owned_related_element(),
            Connector::BindingConnector(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Connector::Itself(x) => x.is_implied(),
            Connector::Succession(x) => x.is_implied(),
            Connector::Flow(x) => x.is_implied(),
            Connector::BindingConnector(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Connector {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.target_ref_mut(),
            Connector::Succession(x) => x.target_ref_mut(),
            Connector::Flow(x) => x.target_ref_mut(),
            Connector::BindingConnector(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.source_ref_mut(),
            Connector::Succession(x) => x.source_ref_mut(),
            Connector::Flow(x) => x.source_ref_mut(),
            Connector::BindingConnector(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.owning_related_element_ref_mut(),
            Connector::Succession(x) => x.owning_related_element_ref_mut(),
            Connector::Flow(x) => x.owning_related_element_ref_mut(),
            Connector::BindingConnector(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.owned_related_element_ref_mut(),
            Connector::Succession(x) => x.owned_related_element_ref_mut(),
            Connector::Flow(x) => x.owned_related_element_ref_mut(),
            Connector::BindingConnector(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Connector::Itself(x) => x.is_implied_ref_mut(),
            Connector::Succession(x) => x.is_implied_ref_mut(),
            Connector::Flow(x) => x.is_implied_ref_mut(),
            Connector::BindingConnector(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Connector {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.target_ref(),
            Connector::Succession(x) => x.target_ref(),
            Connector::Flow(x) => x.target_ref(),
            Connector::BindingConnector(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.source_ref(),
            Connector::Succession(x) => x.source_ref(),
            Connector::Flow(x) => x.source_ref(),
            Connector::BindingConnector(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.owning_related_element_ref(),
            Connector::Succession(x) => x.owning_related_element_ref(),
            Connector::Flow(x) => x.owning_related_element_ref(),
            Connector::BindingConnector(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Connector::Itself(x) => x.owned_related_element_ref(),
            Connector::Succession(x) => x.owned_related_element_ref(),
            Connector::Flow(x) => x.owned_related_element_ref(),
            Connector::BindingConnector(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Connector::Itself(x) => x.is_implied_ref(),
            Connector::Succession(x) => x.is_implied_ref(),
            Connector::Flow(x) => x.is_implied_ref(),
            Connector::BindingConnector(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for ConnectorRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.target_ref_mut(),
            ConnectorRefMut::Succession(x) => x.target_ref_mut(),
            ConnectorRefMut::Flow(x) => x.target_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.source_ref_mut(),
            ConnectorRefMut::Succession(x) => x.source_ref_mut(),
            ConnectorRefMut::Flow(x) => x.source_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            ConnectorRefMut::Succession(x) => x.owning_related_element_ref_mut(),
            ConnectorRefMut::Flow(x) => x.owning_related_element_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            ConnectorRefMut::Succession(x) => x.owned_related_element_ref_mut(),
            ConnectorRefMut::Flow(x) => x.owned_related_element_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_implied_ref_mut(),
            ConnectorRefMut::Succession(x) => x.is_implied_ref_mut(),
            ConnectorRefMut::Flow(x) => x.is_implied_ref_mut(),
            ConnectorRefMut::BindingConnector(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for ConnectorRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.target_ref(),
            ConnectorRefMut::Succession(x) => x.target_ref(),
            ConnectorRefMut::Flow(x) => x.target_ref(),
            ConnectorRefMut::BindingConnector(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.source_ref(),
            ConnectorRefMut::Succession(x) => x.source_ref(),
            ConnectorRefMut::Flow(x) => x.source_ref(),
            ConnectorRefMut::BindingConnector(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owning_related_element_ref(),
            ConnectorRefMut::Succession(x) => x.owning_related_element_ref(),
            ConnectorRefMut::Flow(x) => x.owning_related_element_ref(),
            ConnectorRefMut::BindingConnector(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRefMut::Itself(x) => x.owned_related_element_ref(),
            ConnectorRefMut::Succession(x) => x.owned_related_element_ref(),
            ConnectorRefMut::Flow(x) => x.owned_related_element_ref(),
            ConnectorRefMut::BindingConnector(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ConnectorRefMut::Itself(x) => x.is_implied_ref(),
            ConnectorRefMut::Succession(x) => x.is_implied_ref(),
            ConnectorRefMut::Flow(x) => x.is_implied_ref(),
            ConnectorRefMut::BindingConnector(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for ConnectorRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRef::Itself(x) => x.target_ref(),
            ConnectorRef::Succession(x) => x.target_ref(),
            ConnectorRef::Flow(x) => x.target_ref(),
            ConnectorRef::BindingConnector(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRef::Itself(x) => x.source_ref(),
            ConnectorRef::Succession(x) => x.source_ref(),
            ConnectorRef::Flow(x) => x.source_ref(),
            ConnectorRef::BindingConnector(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRef::Itself(x) => x.owning_related_element_ref(),
            ConnectorRef::Succession(x) => x.owning_related_element_ref(),
            ConnectorRef::Flow(x) => x.owning_related_element_ref(),
            ConnectorRef::BindingConnector(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            ConnectorRef::Itself(x) => x.owned_related_element_ref(),
            ConnectorRef::Succession(x) => x.owned_related_element_ref(),
            ConnectorRef::Flow(x) => x.owned_related_element_ref(),
            ConnectorRef::BindingConnector(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            ConnectorRef::Itself(x) => x.is_implied_ref(),
            ConnectorRef::Succession(x) => x.is_implied_ref(),
            ConnectorRef::Flow(x) => x.is_implied_ref(),
            ConnectorRef::BindingConnector(x) => x.is_implied_ref(),
        }
    }
}
impl ConnectorUpcast for Connector {
    fn into_connector(self) -> Connector {
        self
    }
}
impl<'a> ConnectorUpcastRefMut<'a> for ConnectorRefMut<'a> {
    fn as_connector_ref_mut(self) -> ConnectorRefMut<'a> {
        self
    }
}
impl<'a> ConnectorUpcastRef<'a> for ConnectorRef<'a> {
    fn as_connector_ref(self) -> ConnectorRef<'a> {
        self
    }
}
impl FeatureUpcast for Connector {
    fn into_feature(self) -> Feature {
        Feature::Connector(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for ConnectorRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FeatureRefMut::Connector(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for ConnectorRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FeatureRef::Connector(self).as_feature_ref()
    }
}
impl TypeUpcast for Connector {
    fn into_type_(self) -> Type {
        Feature::Connector(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for ConnectorRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FeatureRefMut::Connector(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for ConnectorRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FeatureRef::Connector(self).as_type_ref()
    }
}
impl NamespaceUpcast for Connector {
    fn into_namespace(self) -> Namespace {
        Feature::Connector(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for ConnectorRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FeatureRefMut::Connector(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for ConnectorRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FeatureRef::Connector(self).as_namespace_ref()
    }
}
impl ElementUpcast for Connector {
    fn into_element(self) -> Element {
        Feature::Connector(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ConnectorRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureRefMut::Connector(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ConnectorRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureRef::Connector(self).as_element_ref()
    }
}
impl RelationshipUpcast for Connector {
    fn into_relationship(self) -> Relationship {
        Relationship::Connector(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for ConnectorRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        RelationshipRefMut::Connector(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for ConnectorRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        RelationshipRef::Connector(self).as_relationship_ref()
    }
}
pub trait ConnectorDowncast {
    fn try_into_succession(self) -> Result<Succession, String>;
    fn try_into_flow(self) -> Result<Flow, String>;
    fn try_into_binding_connector(self) -> Result<BindingConnector, String>;
}
pub trait ConnectorDowncastRefMut<'a> {
    fn try_as_succession_ref_mut(self) -> Result<SuccessionRefMut<'a>, String>;
    fn try_as_flow_ref_mut(self) -> Result<FlowRefMut<'a>, String>;
    fn try_as_binding_connector_ref_mut(
        self,
    ) -> Result<BindingConnectorRefMut<'a>, String>;
}
pub trait ConnectorDowncastRef<'a> {
    fn try_as_succession_ref(self) -> Result<SuccessionRef<'a>, String>;
    fn try_as_flow_ref(self) -> Result<FlowRef<'a>, String>;
    fn try_as_binding_connector_ref(self) -> Result<BindingConnectorRef<'a>, String>;
}
impl ConnectorDowncast for Connector {
    fn try_into_succession(self) -> Result<Succession, String> {
        match self {
            Connector::Succession(e) => Ok(e),
            _ => Err("Not a Succession".into()),
        }
    }
    fn try_into_flow(self) -> Result<Flow, String> {
        match self {
            Connector::Flow(e) => Ok(e),
            _ => Err("Not a Flow".into()),
        }
    }
    fn try_into_binding_connector(self) -> Result<BindingConnector, String> {
        match self {
            Connector::BindingConnector(e) => Ok(e),
            _ => Err("Not a BindingConnector".into()),
        }
    }
}
impl<'a> ConnectorDowncastRefMut<'a> for ConnectorRefMut<'a> {
    fn try_as_succession_ref_mut(self) -> Result<SuccessionRefMut<'a>, String> {
        match self {
            ConnectorRefMut::Succession(e) => Ok(e),
            _ => Err("Not a Succession".into()),
        }
    }
    fn try_as_flow_ref_mut(self) -> Result<FlowRefMut<'a>, String> {
        match self {
            ConnectorRefMut::Flow(e) => Ok(e),
            _ => Err("Not a Flow".into()),
        }
    }
    fn try_as_binding_connector_ref_mut(
        self,
    ) -> Result<BindingConnectorRefMut<'a>, String> {
        match self {
            ConnectorRefMut::BindingConnector(e) => Ok(e),
            _ => Err("Not a BindingConnector".into()),
        }
    }
}
impl<'a> ConnectorDowncastRef<'a> for ConnectorRef<'a> {
    fn try_as_succession_ref(self) -> Result<SuccessionRef<'a>, String> {
        match self {
            ConnectorRef::Succession(e) => Ok(e),
            _ => Err("Not a Succession".into()),
        }
    }
    fn try_as_flow_ref(self) -> Result<FlowRef<'a>, String> {
        match self {
            ConnectorRef::Flow(e) => Ok(e),
            _ => Err("Not a Flow".into()),
        }
    }
    fn try_as_binding_connector_ref(self) -> Result<BindingConnectorRef<'a>, String> {
        match self {
            ConnectorRef::BindingConnector(e) => Ok(e),
            _ => Err("Not a BindingConnector".into()),
        }
    }
}
pub trait ConnectorMethodsDescendants
where
    Self: DescendantOf<Connector>,
    Self::Via: ConnectorMethods,
    Self: Sized,
{}
pub trait ConnectorMethods: Sized {}
impl<T: ConnectorMethodsDescendants> ConnectorMethods for T
where
    T::Via: ConnectorMethods,
{}
impl DescendantOf<Relationship> for Connector {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl RelationshipMethodsDescendants for Connector {}
impl DescendantOf<Element> for Connector {
    type Via = Relationship;
    fn into_via(self) -> Self::Via {
        self.into_relationship()
    }
}
impl ElementMethodsDescendants for Connector {}
impl DescendantOf<Feature> for Connector {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl FeatureMethodsDescendants for Connector {}
impl DescendantOf<Type> for Connector {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl TypeMethodsDescendants for Connector {}
impl DescendantOf<Namespace> for Connector {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl NamespaceMethodsDescendants for Connector {}
pub trait ConnectorRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ConnectorRefMut<'a>>,
    Self::Via: ConnectorRefMutMethods,
    Self: Sized,
{}
pub trait ConnectorRefMutMethods: Sized {}
impl<'a, T: ConnectorRefMutMethodsDescendants<'a>> ConnectorRefMutMethods for T
where
    T::Via: ConnectorRefMutMethods,
{}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for ConnectorRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for ConnectorRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ConnectorRefMut<'a> {
    type Via = RelationshipRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ConnectorRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for ConnectorRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for ConnectorRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for ConnectorRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for ConnectorRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for ConnectorRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for ConnectorRefMut<'a> {}
pub trait ConnectorRefMethodsDescendants<'a>
where
    Self: DescendantOf<ConnectorRef<'a>>,
    Self::Via: ConnectorRefMethods,
    Self: Sized,
{}
pub trait ConnectorRefMethods: Sized {}
impl<'a, T: ConnectorRefMethodsDescendants<'a>> ConnectorRefMethods for T
where
    T::Via: ConnectorRefMethods,
{}
impl<'a> DescendantOf<RelationshipRef<'a>> for ConnectorRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for ConnectorRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ConnectorRef<'a> {
    type Via = RelationshipRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_relationship_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ConnectorRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for ConnectorRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for ConnectorRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for ConnectorRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for ConnectorRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for ConnectorRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for ConnectorRef<'a> {}

