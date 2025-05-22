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
use super::succession_flow::{SuccessionFlow, SuccessionFlowRefMut, SuccessionFlowRef};
pub struct SuccessionInner {
    pub(super) sup_connector: ConnectorInner,
}
pub trait SuccessionStruct
where
    Self: SuccessionStructRefMut,
    Self: SuccessionStructRef,
    Self: ConnectorStruct,
{}
pub trait SuccessionStructRefMut
where
    Self: SuccessionStructRef,
    Self: ConnectorStructRefMut,
{}
pub trait SuccessionStructRef
where
    Self: ConnectorStructRef,
{}
pub trait SuccessionUpcast: SuccessionStruct {
    fn into_succession(self) -> Succession;
}
pub trait SuccessionUpcastRefMut<'a>: SuccessionStructRefMut {
    fn as_succession_ref_mut(self) -> SuccessionRefMut<'a>;
}
pub trait SuccessionUpcastRef<'a>: SuccessionStructRef {
    fn as_succession_ref(self) -> SuccessionRef<'a>;
}
impl SuccessionStruct for SuccessionInner {}
impl SuccessionStructRefMut for SuccessionInner {}
impl SuccessionStructRef for SuccessionInner {}
impl ConnectorStruct for SuccessionInner {}
impl ConnectorStructRefMut for SuccessionInner {}
impl ConnectorStructRef for SuccessionInner {}
impl FeatureStruct for SuccessionInner {
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
impl FeatureStructRefMut for SuccessionInner {
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
impl FeatureStructRef for SuccessionInner {
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
impl TypeStruct for SuccessionInner {
    fn is_abstract(self) -> bool {
        self.sup_connector.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_connector.is_sufficient()
    }
}
impl TypeStructRefMut for SuccessionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_connector.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for SuccessionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_connector.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_connector.is_sufficient_ref()
    }
}
impl NamespaceStruct for SuccessionInner {}
impl NamespaceStructRefMut for SuccessionInner {}
impl NamespaceStructRef for SuccessionInner {}
impl ElementStruct for SuccessionInner {
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
impl ElementStructRefMut for SuccessionInner {
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
impl ElementStructRef for SuccessionInner {
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
impl RelationshipStruct for SuccessionInner {
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
impl RelationshipStructRefMut for SuccessionInner {
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
impl RelationshipStructRef for SuccessionInner {
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
pub enum Succession {
    Itself(SuccessionInner),
    SuccessionFlow(SuccessionFlow),
}
pub enum SuccessionRefMut<'a> {
    Itself(&'a mut SuccessionInner),
    SuccessionFlow(SuccessionFlowRefMut<'a>),
}
pub enum SuccessionRef<'a> {
    Itself(&'a SuccessionInner),
    SuccessionFlow(SuccessionFlowRef<'a>),
}
impl Succession {
    pub fn as_ref(&self) -> SuccessionRef {
        match self {
            Succession::Itself(inner) => SuccessionRef::Itself(&inner),
            Succession::SuccessionFlow(inner) => {
                SuccessionRef::SuccessionFlow(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> SuccessionRefMut {
        match self {
            Succession::Itself(inner) => SuccessionRefMut::Itself(inner),
            Succession::SuccessionFlow(inner) => {
                SuccessionRefMut::SuccessionFlow(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> SuccessionRefMut<'a> {
    pub fn as_ref(self) -> SuccessionRef<'a> {
        match self {
            SuccessionRefMut::Itself(inner) => SuccessionRef::Itself(inner),
            SuccessionRefMut::SuccessionFlow(inner) => {
                SuccessionRef::SuccessionFlow(inner.as_ref())
            }
        }
    }
}
impl SuccessionStruct for Succession {}
impl SuccessionStructRefMut for Succession {}
impl SuccessionStructRef for Succession {}
impl<'a> SuccessionStructRefMut for SuccessionRefMut<'a> {}
impl<'a> SuccessionStructRef for SuccessionRefMut<'a> {}
impl<'a> SuccessionStructRef for SuccessionRef<'a> {}
impl ConnectorStruct for Succession {}
impl ConnectorStructRefMut for Succession {}
impl ConnectorStructRef for Succession {}
impl<'a> ConnectorStructRefMut for SuccessionRefMut<'a> {}
impl<'a> ConnectorStructRef for SuccessionRefMut<'a> {}
impl<'a> ConnectorStructRef for SuccessionRef<'a> {}
impl FeatureStruct for Succession {
    fn is_unique(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_unique(),
            Succession::SuccessionFlow(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_ordered(),
            Succession::SuccessionFlow(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_composite(),
            Succession::SuccessionFlow(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_end(),
            Succession::SuccessionFlow(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_derived(),
            Succession::SuccessionFlow(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_portion(),
            Succession::SuccessionFlow(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_variable(),
            Succession::SuccessionFlow(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_constant(),
            Succession::SuccessionFlow(x) => x.is_constant(),
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
            Succession::Itself(x) => x.direction(),
            Succession::SuccessionFlow(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Succession {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_unique_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_ordered_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_composite_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_end_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_derived_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_portion_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_variable_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_constant_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_constant_ref_mut(),
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
            Succession::Itself(x) => x.direction_ref_mut(),
            Succession::SuccessionFlow(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Succession {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_unique_ref(),
            Succession::SuccessionFlow(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_ordered_ref(),
            Succession::SuccessionFlow(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_composite_ref(),
            Succession::SuccessionFlow(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_end_ref(),
            Succession::SuccessionFlow(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_derived_ref(),
            Succession::SuccessionFlow(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_portion_ref(),
            Succession::SuccessionFlow(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_variable_ref(),
            Succession::SuccessionFlow(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_constant_ref(),
            Succession::SuccessionFlow(x) => x.is_constant_ref(),
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
            Succession::Itself(x) => x.direction_ref(),
            Succession::SuccessionFlow(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for SuccessionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_unique_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_composite_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_end_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_derived_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_portion_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_variable_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_constant_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_constant_ref_mut(),
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
            SuccessionRefMut::Itself(x) => x.direction_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for SuccessionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_unique_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_ordered_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_composite_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_end_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_derived_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_portion_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_variable_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_constant_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_constant_ref(),
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
            SuccessionRefMut::Itself(x) => x.direction_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for SuccessionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_unique_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_ordered_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_composite_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_end_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_derived_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_portion_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_variable_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_constant_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_constant_ref(),
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
            SuccessionRef::Itself(x) => x.direction_ref(),
            SuccessionRef::SuccessionFlow(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Succession {
    fn is_abstract(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_abstract(),
            Succession::SuccessionFlow(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_sufficient(),
            Succession::SuccessionFlow(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Succession {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_abstract_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_sufficient_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Succession {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_abstract_ref(),
            Succession::SuccessionFlow(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_sufficient_ref(),
            Succession::SuccessionFlow(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for SuccessionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for SuccessionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_abstract_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_sufficient_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for SuccessionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_abstract_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_sufficient_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Succession {}
impl NamespaceStructRefMut for Succession {}
impl NamespaceStructRef for Succession {}
impl<'a> NamespaceStructRefMut for SuccessionRefMut<'a> {}
impl<'a> NamespaceStructRef for SuccessionRefMut<'a> {}
impl<'a> NamespaceStructRef for SuccessionRef<'a> {}
impl ElementStruct for Succession {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Succession::Itself(x) => x.owned_relationship(),
            Succession::SuccessionFlow(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Succession::Itself(x) => x.owning_relationship(),
            Succession::SuccessionFlow(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Succession::Itself(x) => x.element_id(),
            Succession::SuccessionFlow(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Succession::Itself(x) => x.alias_ids(),
            Succession::SuccessionFlow(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Succession::Itself(x) => x.declared_short_name(),
            Succession::SuccessionFlow(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Succession::Itself(x) => x.declared_name(),
            Succession::SuccessionFlow(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_implied_included(),
            Succession::SuccessionFlow(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Succession {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Succession::Itself(x) => x.owned_relationship_ref_mut(),
            Succession::SuccessionFlow(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Succession::Itself(x) => x.owning_relationship_ref_mut(),
            Succession::SuccessionFlow(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Succession::Itself(x) => x.element_id_ref_mut(),
            Succession::SuccessionFlow(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Succession::Itself(x) => x.alias_ids_ref_mut(),
            Succession::SuccessionFlow(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Succession::Itself(x) => x.declared_short_name_ref_mut(),
            Succession::SuccessionFlow(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Succession::Itself(x) => x.declared_name_ref_mut(),
            Succession::SuccessionFlow(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_implied_included_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Succession {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Succession::Itself(x) => x.owned_relationship_ref(),
            Succession::SuccessionFlow(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Succession::Itself(x) => x.owning_relationship_ref(),
            Succession::SuccessionFlow(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Succession::Itself(x) => x.element_id_ref(),
            Succession::SuccessionFlow(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Succession::Itself(x) => x.alias_ids_ref(),
            Succession::SuccessionFlow(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Succession::Itself(x) => x.declared_short_name_ref(),
            Succession::SuccessionFlow(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Succession::Itself(x) => x.declared_name_ref(),
            Succession::SuccessionFlow(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_implied_included_ref(),
            Succession::SuccessionFlow(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for SuccessionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            SuccessionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            SuccessionRefMut::Itself(x) => x.element_id_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            SuccessionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SuccessionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            SuccessionRefMut::Itself(x) => x.declared_name_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for SuccessionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owned_relationship_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owning_relationship_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SuccessionRefMut::Itself(x) => x.element_id_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SuccessionRefMut::Itself(x) => x.alias_ids_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionRefMut::Itself(x) => x.declared_short_name_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionRefMut::Itself(x) => x.declared_name_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_implied_included_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for SuccessionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionRef::Itself(x) => x.owned_relationship_ref(),
            SuccessionRef::SuccessionFlow(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            SuccessionRef::Itself(x) => x.owning_relationship_ref(),
            SuccessionRef::SuccessionFlow(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            SuccessionRef::Itself(x) => x.element_id_ref(),
            SuccessionRef::SuccessionFlow(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            SuccessionRef::Itself(x) => x.alias_ids_ref(),
            SuccessionRef::SuccessionFlow(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionRef::Itself(x) => x.declared_short_name_ref(),
            SuccessionRef::SuccessionFlow(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            SuccessionRef::Itself(x) => x.declared_name_ref(),
            SuccessionRef::SuccessionFlow(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_implied_included_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipStruct for Succession {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.target(),
            Succession::SuccessionFlow(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.source(),
            Succession::SuccessionFlow(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.owning_related_element(),
            Succession::SuccessionFlow(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.owned_related_element(),
            Succession::SuccessionFlow(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Succession::Itself(x) => x.is_implied(),
            Succession::SuccessionFlow(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Succession {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.target_ref_mut(),
            Succession::SuccessionFlow(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.source_ref_mut(),
            Succession::SuccessionFlow(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.owning_related_element_ref_mut(),
            Succession::SuccessionFlow(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.owned_related_element_ref_mut(),
            Succession::SuccessionFlow(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Succession::Itself(x) => x.is_implied_ref_mut(),
            Succession::SuccessionFlow(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Succession {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.target_ref(),
            Succession::SuccessionFlow(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.source_ref(),
            Succession::SuccessionFlow(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.owning_related_element_ref(),
            Succession::SuccessionFlow(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Succession::Itself(x) => x.owned_related_element_ref(),
            Succession::SuccessionFlow(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Succession::Itself(x) => x.is_implied_ref(),
            Succession::SuccessionFlow(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for SuccessionRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.target_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.source_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_implied_ref_mut(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for SuccessionRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.target_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.source_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owning_related_element_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRefMut::Itself(x) => x.owned_related_element_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SuccessionRefMut::Itself(x) => x.is_implied_ref(),
            SuccessionRefMut::SuccessionFlow(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for SuccessionRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRef::Itself(x) => x.target_ref(),
            SuccessionRef::SuccessionFlow(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRef::Itself(x) => x.source_ref(),
            SuccessionRef::SuccessionFlow(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRef::Itself(x) => x.owning_related_element_ref(),
            SuccessionRef::SuccessionFlow(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            SuccessionRef::Itself(x) => x.owned_related_element_ref(),
            SuccessionRef::SuccessionFlow(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            SuccessionRef::Itself(x) => x.is_implied_ref(),
            SuccessionRef::SuccessionFlow(x) => x.is_implied_ref(),
        }
    }
}
impl SuccessionUpcast for Succession {
    fn into_succession(self) -> Succession {
        self
    }
}
impl<'a> SuccessionUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_succession_ref_mut(self) -> SuccessionRefMut<'a> {
        self
    }
}
impl<'a> SuccessionUpcastRef<'a> for SuccessionRef<'a> {
    fn as_succession_ref(self) -> SuccessionRef<'a> {
        self
    }
}
impl ConnectorUpcast for Succession {
    fn into_connector(self) -> Connector {
        Connector::Succession(self).into_connector()
    }
}
impl<'a> ConnectorUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_connector_ref_mut(self) -> ConnectorRefMut<'a> {
        ConnectorRefMut::Succession(self).as_connector_ref_mut()
    }
}
impl<'a> ConnectorUpcastRef<'a> for SuccessionRef<'a> {
    fn as_connector_ref(self) -> ConnectorRef<'a> {
        ConnectorRef::Succession(self).as_connector_ref()
    }
}
impl FeatureUpcast for Succession {
    fn into_feature(self) -> Feature {
        Connector::Succession(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ConnectorRefMut::Succession(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for SuccessionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ConnectorRef::Succession(self).as_feature_ref()
    }
}
impl TypeUpcast for Succession {
    fn into_type_(self) -> Type {
        Connector::Succession(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ConnectorRefMut::Succession(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for SuccessionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ConnectorRef::Succession(self).as_type_ref()
    }
}
impl NamespaceUpcast for Succession {
    fn into_namespace(self) -> Namespace {
        Connector::Succession(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ConnectorRefMut::Succession(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for SuccessionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ConnectorRef::Succession(self).as_namespace_ref()
    }
}
impl ElementUpcast for Succession {
    fn into_element(self) -> Element {
        Connector::Succession(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ConnectorRefMut::Succession(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for SuccessionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ConnectorRef::Succession(self).as_element_ref()
    }
}
impl RelationshipUpcast for Succession {
    fn into_relationship(self) -> Relationship {
        Connector::Succession(self).into_relationship()
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for SuccessionRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        ConnectorRefMut::Succession(self).as_relationship_ref_mut()
    }
}
impl<'a> RelationshipUpcastRef<'a> for SuccessionRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        ConnectorRef::Succession(self).as_relationship_ref()
    }
}
pub trait SuccessionDowncast {
    fn try_into_succession_flow(self) -> Result<SuccessionFlow, String>;
}
pub trait SuccessionDowncastRefMut<'a> {
    fn try_as_succession_flow_ref_mut(self) -> Result<SuccessionFlowRefMut<'a>, String>;
}
pub trait SuccessionDowncastRef<'a> {
    fn try_as_succession_flow_ref(self) -> Result<SuccessionFlowRef<'a>, String>;
}
impl SuccessionDowncast for Succession {
    fn try_into_succession_flow(self) -> Result<SuccessionFlow, String> {
        match self {
            Succession::SuccessionFlow(e) => Ok(e),
            _ => Err("Not a SuccessionFlow".into()),
        }
    }
}
impl<'a> SuccessionDowncastRefMut<'a> for SuccessionRefMut<'a> {
    fn try_as_succession_flow_ref_mut(self) -> Result<SuccessionFlowRefMut<'a>, String> {
        match self {
            SuccessionRefMut::SuccessionFlow(e) => Ok(e),
            _ => Err("Not a SuccessionFlow".into()),
        }
    }
}
impl<'a> SuccessionDowncastRef<'a> for SuccessionRef<'a> {
    fn try_as_succession_flow_ref(self) -> Result<SuccessionFlowRef<'a>, String> {
        match self {
            SuccessionRef::SuccessionFlow(e) => Ok(e),
            _ => Err("Not a SuccessionFlow".into()),
        }
    }
}
pub trait SuccessionMethodsDescendants
where
    Self: DescendantOf<Succession>,
    Self::Via: SuccessionMethods,
    Self: Sized,
{}
pub trait SuccessionMethods: Sized {}
impl<T: SuccessionMethodsDescendants> SuccessionMethods for T
where
    T::Via: SuccessionMethods,
{}
impl DescendantOf<Connector> for Succession {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl ConnectorMethodsDescendants for Succession {}
impl DescendantOf<Feature> for Succession {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl FeatureMethodsDescendants for Succession {}
impl DescendantOf<Type> for Succession {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl TypeMethodsDescendants for Succession {}
impl DescendantOf<Namespace> for Succession {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl NamespaceMethodsDescendants for Succession {}
impl DescendantOf<Element> for Succession {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl ElementMethodsDescendants for Succession {}
impl DescendantOf<Relationship> for Succession {
    type Via = Connector;
    fn into_via(self) -> Self::Via {
        self.into_connector()
    }
}
impl RelationshipMethodsDescendants for Succession {}
pub trait SuccessionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<SuccessionRefMut<'a>>,
    Self::Via: SuccessionRefMutMethods,
    Self: Sized,
{}
pub trait SuccessionRefMutMethods: Sized {}
impl<'a, T: SuccessionRefMutMethodsDescendants<'a>> SuccessionRefMutMethods for T
where
    T::Via: SuccessionRefMutMethods,
{}
impl<'a> DescendantOf<ConnectorRefMut<'a>> for SuccessionRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> ConnectorRefMutMethodsDescendants<'a> for SuccessionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for SuccessionRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for SuccessionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for SuccessionRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for SuccessionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for SuccessionRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for SuccessionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for SuccessionRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for SuccessionRefMut<'a> {}
impl<'a> DescendantOf<RelationshipRefMut<'a>> for SuccessionRefMut<'a> {
    type Via = ConnectorRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref_mut()
    }
}
impl<'a> RelationshipRefMutMethodsDescendants<'a> for SuccessionRefMut<'a> {}
pub trait SuccessionRefMethodsDescendants<'a>
where
    Self: DescendantOf<SuccessionRef<'a>>,
    Self::Via: SuccessionRefMethods,
    Self: Sized,
{}
pub trait SuccessionRefMethods: Sized {}
impl<'a, T: SuccessionRefMethodsDescendants<'a>> SuccessionRefMethods for T
where
    T::Via: SuccessionRefMethods,
{}
impl<'a> DescendantOf<ConnectorRef<'a>> for SuccessionRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> ConnectorRefMethodsDescendants<'a> for SuccessionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for SuccessionRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for SuccessionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for SuccessionRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for SuccessionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for SuccessionRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for SuccessionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for SuccessionRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for SuccessionRef<'a> {}
impl<'a> DescendantOf<RelationshipRef<'a>> for SuccessionRef<'a> {
    type Via = ConnectorRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_connector_ref()
    }
}
impl<'a> RelationshipRefMethodsDescendants<'a> for SuccessionRef<'a> {}

