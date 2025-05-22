#![allow(unused)]
use super::utils::DescendantOf;
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
use super::payload_feature::{PayloadFeature, PayloadFeatureRefMut, PayloadFeatureRef};
use super::metadata_feature::{
    MetadataFeature, MetadataFeatureRefMut, MetadataFeatureRef,
};
use super::multiplicity::{Multiplicity, MultiplicityRefMut, MultiplicityRef};
use super::connector::{Connector, ConnectorRefMut, ConnectorRef};
use super::step::{Step, StepRefMut, StepRef};
use super::flow_end::{FlowEnd, FlowEndRefMut, FlowEndRef};
pub struct FeatureInner {
    pub(super) sup_type_: TypeInner,
    pub(super) is_unique: bool,
    pub(super) is_ordered: bool,
    pub(super) is_composite: bool,
    pub(super) is_end: bool,
    pub(super) is_derived: bool,
    pub(super) is_portion: bool,
    pub(super) is_variable: bool,
    pub(super) is_constant: bool,
    pub(super) direction: Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >,
}
pub trait FeatureStruct
where
    Self: FeatureStructRefMut,
    Self: FeatureStructRef,
    Self: TypeStruct,
{
    fn is_unique(self) -> bool;
    fn is_ordered(self) -> bool;
    fn is_composite(self) -> bool;
    fn is_end(self) -> bool;
    fn is_derived(self) -> bool;
    fn is_portion(self) -> bool;
    fn is_variable(self) -> bool;
    fn is_constant(self) -> bool;
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >;
}
pub trait FeatureStructRefMut
where
    Self: FeatureStructRef,
    Self: TypeStructRefMut,
{
    fn is_unique_ref_mut(&mut self) -> &mut bool;
    fn is_ordered_ref_mut(&mut self) -> &mut bool;
    fn is_composite_ref_mut(&mut self) -> &mut bool;
    fn is_end_ref_mut(&mut self) -> &mut bool;
    fn is_derived_ref_mut(&mut self) -> &mut bool;
    fn is_portion_ref_mut(&mut self) -> &mut bool;
    fn is_variable_ref_mut(&mut self) -> &mut bool;
    fn is_constant_ref_mut(&mut self) -> &mut bool;
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >;
}
pub trait FeatureStructRef
where
    Self: TypeStructRef,
{
    fn is_unique_ref(&self) -> &bool;
    fn is_ordered_ref(&self) -> &bool;
    fn is_composite_ref(&self) -> &bool;
    fn is_end_ref(&self) -> &bool;
    fn is_derived_ref(&self) -> &bool;
    fn is_portion_ref(&self) -> &bool;
    fn is_variable_ref(&self) -> &bool;
    fn is_constant_ref(&self) -> &bool;
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >;
}
pub trait FeatureUpcast: FeatureStruct {
    fn into_feature(self) -> Feature;
}
pub trait FeatureUpcastRefMut<'a>: FeatureStructRefMut {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a>;
}
pub trait FeatureUpcastRef<'a>: FeatureStructRef {
    fn as_feature_ref(self) -> FeatureRef<'a>;
}
impl FeatureStruct for FeatureInner {
    fn is_unique(self) -> bool {
        self.is_unique
    }
    fn is_ordered(self) -> bool {
        self.is_ordered
    }
    fn is_composite(self) -> bool {
        self.is_composite
    }
    fn is_end(self) -> bool {
        self.is_end
    }
    fn is_derived(self) -> bool {
        self.is_derived
    }
    fn is_portion(self) -> bool {
        self.is_portion
    }
    fn is_variable(self) -> bool {
        self.is_variable
    }
    fn is_constant(self) -> bool {
        self.is_constant
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.direction
    }
}
impl FeatureStructRefMut for FeatureInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        &mut self.is_unique
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        &mut self.is_ordered
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        &mut self.is_composite
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        &mut self.is_end
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        &mut self.is_derived
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        &mut self.is_portion
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        &mut self.is_variable
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        &mut self.is_constant
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        &mut self.direction
    }
}
impl FeatureStructRef for FeatureInner {
    fn is_unique_ref(&self) -> &bool {
        &self.is_unique
    }
    fn is_ordered_ref(&self) -> &bool {
        &self.is_ordered
    }
    fn is_composite_ref(&self) -> &bool {
        &self.is_composite
    }
    fn is_end_ref(&self) -> &bool {
        &self.is_end
    }
    fn is_derived_ref(&self) -> &bool {
        &self.is_derived
    }
    fn is_portion_ref(&self) -> &bool {
        &self.is_portion
    }
    fn is_variable_ref(&self) -> &bool {
        &self.is_variable
    }
    fn is_constant_ref(&self) -> &bool {
        &self.is_constant
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        &self.direction
    }
}
impl TypeStruct for FeatureInner {
    fn is_abstract(self) -> bool {
        self.sup_type_.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_type_.is_sufficient()
    }
}
impl TypeStructRefMut for FeatureInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_type_.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_type_.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for FeatureInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_type_.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_type_.is_sufficient_ref()
    }
}
impl NamespaceStruct for FeatureInner {}
impl NamespaceStructRefMut for FeatureInner {}
impl NamespaceStructRef for FeatureInner {}
impl ElementStruct for FeatureInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_type_.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_type_.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_type_.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_type_.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_type_.is_implied_included()
    }
}
impl ElementStructRefMut for FeatureInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_type_.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_type_.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_type_.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_type_.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_type_.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_type_.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for FeatureInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_type_.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_type_.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_type_.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_type_.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_type_.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_type_.is_implied_included_ref()
    }
}
pub enum Feature {
    Itself(FeatureInner),
    PayloadFeature(PayloadFeature),
    MetadataFeature(MetadataFeature),
    Multiplicity(Multiplicity),
    Connector(Connector),
    Step(Step),
    FlowEnd(FlowEnd),
}
pub enum FeatureRefMut<'a> {
    Itself(&'a mut FeatureInner),
    PayloadFeature(PayloadFeatureRefMut<'a>),
    MetadataFeature(MetadataFeatureRefMut<'a>),
    Multiplicity(MultiplicityRefMut<'a>),
    Connector(ConnectorRefMut<'a>),
    Step(StepRefMut<'a>),
    FlowEnd(FlowEndRefMut<'a>),
}
pub enum FeatureRef<'a> {
    Itself(&'a FeatureInner),
    PayloadFeature(PayloadFeatureRef<'a>),
    MetadataFeature(MetadataFeatureRef<'a>),
    Multiplicity(MultiplicityRef<'a>),
    Connector(ConnectorRef<'a>),
    Step(StepRef<'a>),
    FlowEnd(FlowEndRef<'a>),
}
impl Feature {
    pub fn as_ref(&self) -> FeatureRef {
        match self {
            Feature::Itself(inner) => FeatureRef::Itself(&inner),
            Feature::PayloadFeature(inner) => FeatureRef::PayloadFeature(inner.as_ref()),
            Feature::MetadataFeature(inner) => {
                FeatureRef::MetadataFeature(inner.as_ref())
            }
            Feature::Multiplicity(inner) => FeatureRef::Multiplicity(inner.as_ref()),
            Feature::Connector(inner) => FeatureRef::Connector(inner.as_ref()),
            Feature::Step(inner) => FeatureRef::Step(inner.as_ref()),
            Feature::FlowEnd(inner) => FeatureRef::FlowEnd(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureRefMut {
        match self {
            Feature::Itself(inner) => FeatureRefMut::Itself(inner),
            Feature::PayloadFeature(inner) => {
                FeatureRefMut::PayloadFeature(inner.as_ref_mut())
            }
            Feature::MetadataFeature(inner) => {
                FeatureRefMut::MetadataFeature(inner.as_ref_mut())
            }
            Feature::Multiplicity(inner) => {
                FeatureRefMut::Multiplicity(inner.as_ref_mut())
            }
            Feature::Connector(inner) => FeatureRefMut::Connector(inner.as_ref_mut()),
            Feature::Step(inner) => FeatureRefMut::Step(inner.as_ref_mut()),
            Feature::FlowEnd(inner) => FeatureRefMut::FlowEnd(inner.as_ref_mut()),
        }
    }
}
impl<'a> FeatureRefMut<'a> {
    pub fn as_ref(self) -> FeatureRef<'a> {
        match self {
            FeatureRefMut::Itself(inner) => FeatureRef::Itself(inner),
            FeatureRefMut::PayloadFeature(inner) => {
                FeatureRef::PayloadFeature(inner.as_ref())
            }
            FeatureRefMut::MetadataFeature(inner) => {
                FeatureRef::MetadataFeature(inner.as_ref())
            }
            FeatureRefMut::Multiplicity(inner) => {
                FeatureRef::Multiplicity(inner.as_ref())
            }
            FeatureRefMut::Connector(inner) => FeatureRef::Connector(inner.as_ref()),
            FeatureRefMut::Step(inner) => FeatureRef::Step(inner.as_ref()),
            FeatureRefMut::FlowEnd(inner) => FeatureRef::FlowEnd(inner.as_ref()),
        }
    }
}
impl FeatureStruct for Feature {
    fn is_unique(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_unique(),
            Feature::PayloadFeature(x) => x.is_unique(),
            Feature::MetadataFeature(x) => x.is_unique(),
            Feature::Multiplicity(x) => x.is_unique(),
            Feature::Connector(x) => x.is_unique(),
            Feature::Step(x) => x.is_unique(),
            Feature::FlowEnd(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_ordered(),
            Feature::PayloadFeature(x) => x.is_ordered(),
            Feature::MetadataFeature(x) => x.is_ordered(),
            Feature::Multiplicity(x) => x.is_ordered(),
            Feature::Connector(x) => x.is_ordered(),
            Feature::Step(x) => x.is_ordered(),
            Feature::FlowEnd(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_composite(),
            Feature::PayloadFeature(x) => x.is_composite(),
            Feature::MetadataFeature(x) => x.is_composite(),
            Feature::Multiplicity(x) => x.is_composite(),
            Feature::Connector(x) => x.is_composite(),
            Feature::Step(x) => x.is_composite(),
            Feature::FlowEnd(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_end(),
            Feature::PayloadFeature(x) => x.is_end(),
            Feature::MetadataFeature(x) => x.is_end(),
            Feature::Multiplicity(x) => x.is_end(),
            Feature::Connector(x) => x.is_end(),
            Feature::Step(x) => x.is_end(),
            Feature::FlowEnd(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_derived(),
            Feature::PayloadFeature(x) => x.is_derived(),
            Feature::MetadataFeature(x) => x.is_derived(),
            Feature::Multiplicity(x) => x.is_derived(),
            Feature::Connector(x) => x.is_derived(),
            Feature::Step(x) => x.is_derived(),
            Feature::FlowEnd(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_portion(),
            Feature::PayloadFeature(x) => x.is_portion(),
            Feature::MetadataFeature(x) => x.is_portion(),
            Feature::Multiplicity(x) => x.is_portion(),
            Feature::Connector(x) => x.is_portion(),
            Feature::Step(x) => x.is_portion(),
            Feature::FlowEnd(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_variable(),
            Feature::PayloadFeature(x) => x.is_variable(),
            Feature::MetadataFeature(x) => x.is_variable(),
            Feature::Multiplicity(x) => x.is_variable(),
            Feature::Connector(x) => x.is_variable(),
            Feature::Step(x) => x.is_variable(),
            Feature::FlowEnd(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_constant(),
            Feature::PayloadFeature(x) => x.is_constant(),
            Feature::MetadataFeature(x) => x.is_constant(),
            Feature::Multiplicity(x) => x.is_constant(),
            Feature::Connector(x) => x.is_constant(),
            Feature::Step(x) => x.is_constant(),
            Feature::FlowEnd(x) => x.is_constant(),
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
            Feature::Itself(x) => x.direction(),
            Feature::PayloadFeature(x) => x.direction(),
            Feature::MetadataFeature(x) => x.direction(),
            Feature::Multiplicity(x) => x.direction(),
            Feature::Connector(x) => x.direction(),
            Feature::Step(x) => x.direction(),
            Feature::FlowEnd(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Feature {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_unique_ref_mut(),
            Feature::PayloadFeature(x) => x.is_unique_ref_mut(),
            Feature::MetadataFeature(x) => x.is_unique_ref_mut(),
            Feature::Multiplicity(x) => x.is_unique_ref_mut(),
            Feature::Connector(x) => x.is_unique_ref_mut(),
            Feature::Step(x) => x.is_unique_ref_mut(),
            Feature::FlowEnd(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_ordered_ref_mut(),
            Feature::PayloadFeature(x) => x.is_ordered_ref_mut(),
            Feature::MetadataFeature(x) => x.is_ordered_ref_mut(),
            Feature::Multiplicity(x) => x.is_ordered_ref_mut(),
            Feature::Connector(x) => x.is_ordered_ref_mut(),
            Feature::Step(x) => x.is_ordered_ref_mut(),
            Feature::FlowEnd(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_composite_ref_mut(),
            Feature::PayloadFeature(x) => x.is_composite_ref_mut(),
            Feature::MetadataFeature(x) => x.is_composite_ref_mut(),
            Feature::Multiplicity(x) => x.is_composite_ref_mut(),
            Feature::Connector(x) => x.is_composite_ref_mut(),
            Feature::Step(x) => x.is_composite_ref_mut(),
            Feature::FlowEnd(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_end_ref_mut(),
            Feature::PayloadFeature(x) => x.is_end_ref_mut(),
            Feature::MetadataFeature(x) => x.is_end_ref_mut(),
            Feature::Multiplicity(x) => x.is_end_ref_mut(),
            Feature::Connector(x) => x.is_end_ref_mut(),
            Feature::Step(x) => x.is_end_ref_mut(),
            Feature::FlowEnd(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_derived_ref_mut(),
            Feature::PayloadFeature(x) => x.is_derived_ref_mut(),
            Feature::MetadataFeature(x) => x.is_derived_ref_mut(),
            Feature::Multiplicity(x) => x.is_derived_ref_mut(),
            Feature::Connector(x) => x.is_derived_ref_mut(),
            Feature::Step(x) => x.is_derived_ref_mut(),
            Feature::FlowEnd(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_portion_ref_mut(),
            Feature::PayloadFeature(x) => x.is_portion_ref_mut(),
            Feature::MetadataFeature(x) => x.is_portion_ref_mut(),
            Feature::Multiplicity(x) => x.is_portion_ref_mut(),
            Feature::Connector(x) => x.is_portion_ref_mut(),
            Feature::Step(x) => x.is_portion_ref_mut(),
            Feature::FlowEnd(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_variable_ref_mut(),
            Feature::PayloadFeature(x) => x.is_variable_ref_mut(),
            Feature::MetadataFeature(x) => x.is_variable_ref_mut(),
            Feature::Multiplicity(x) => x.is_variable_ref_mut(),
            Feature::Connector(x) => x.is_variable_ref_mut(),
            Feature::Step(x) => x.is_variable_ref_mut(),
            Feature::FlowEnd(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_constant_ref_mut(),
            Feature::PayloadFeature(x) => x.is_constant_ref_mut(),
            Feature::MetadataFeature(x) => x.is_constant_ref_mut(),
            Feature::Multiplicity(x) => x.is_constant_ref_mut(),
            Feature::Connector(x) => x.is_constant_ref_mut(),
            Feature::Step(x) => x.is_constant_ref_mut(),
            Feature::FlowEnd(x) => x.is_constant_ref_mut(),
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
            Feature::Itself(x) => x.direction_ref_mut(),
            Feature::PayloadFeature(x) => x.direction_ref_mut(),
            Feature::MetadataFeature(x) => x.direction_ref_mut(),
            Feature::Multiplicity(x) => x.direction_ref_mut(),
            Feature::Connector(x) => x.direction_ref_mut(),
            Feature::Step(x) => x.direction_ref_mut(),
            Feature::FlowEnd(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Feature {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_unique_ref(),
            Feature::PayloadFeature(x) => x.is_unique_ref(),
            Feature::MetadataFeature(x) => x.is_unique_ref(),
            Feature::Multiplicity(x) => x.is_unique_ref(),
            Feature::Connector(x) => x.is_unique_ref(),
            Feature::Step(x) => x.is_unique_ref(),
            Feature::FlowEnd(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_ordered_ref(),
            Feature::PayloadFeature(x) => x.is_ordered_ref(),
            Feature::MetadataFeature(x) => x.is_ordered_ref(),
            Feature::Multiplicity(x) => x.is_ordered_ref(),
            Feature::Connector(x) => x.is_ordered_ref(),
            Feature::Step(x) => x.is_ordered_ref(),
            Feature::FlowEnd(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_composite_ref(),
            Feature::PayloadFeature(x) => x.is_composite_ref(),
            Feature::MetadataFeature(x) => x.is_composite_ref(),
            Feature::Multiplicity(x) => x.is_composite_ref(),
            Feature::Connector(x) => x.is_composite_ref(),
            Feature::Step(x) => x.is_composite_ref(),
            Feature::FlowEnd(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_end_ref(),
            Feature::PayloadFeature(x) => x.is_end_ref(),
            Feature::MetadataFeature(x) => x.is_end_ref(),
            Feature::Multiplicity(x) => x.is_end_ref(),
            Feature::Connector(x) => x.is_end_ref(),
            Feature::Step(x) => x.is_end_ref(),
            Feature::FlowEnd(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_derived_ref(),
            Feature::PayloadFeature(x) => x.is_derived_ref(),
            Feature::MetadataFeature(x) => x.is_derived_ref(),
            Feature::Multiplicity(x) => x.is_derived_ref(),
            Feature::Connector(x) => x.is_derived_ref(),
            Feature::Step(x) => x.is_derived_ref(),
            Feature::FlowEnd(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_portion_ref(),
            Feature::PayloadFeature(x) => x.is_portion_ref(),
            Feature::MetadataFeature(x) => x.is_portion_ref(),
            Feature::Multiplicity(x) => x.is_portion_ref(),
            Feature::Connector(x) => x.is_portion_ref(),
            Feature::Step(x) => x.is_portion_ref(),
            Feature::FlowEnd(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_variable_ref(),
            Feature::PayloadFeature(x) => x.is_variable_ref(),
            Feature::MetadataFeature(x) => x.is_variable_ref(),
            Feature::Multiplicity(x) => x.is_variable_ref(),
            Feature::Connector(x) => x.is_variable_ref(),
            Feature::Step(x) => x.is_variable_ref(),
            Feature::FlowEnd(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_constant_ref(),
            Feature::PayloadFeature(x) => x.is_constant_ref(),
            Feature::MetadataFeature(x) => x.is_constant_ref(),
            Feature::Multiplicity(x) => x.is_constant_ref(),
            Feature::Connector(x) => x.is_constant_ref(),
            Feature::Step(x) => x.is_constant_ref(),
            Feature::FlowEnd(x) => x.is_constant_ref(),
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
            Feature::Itself(x) => x.direction_ref(),
            Feature::PayloadFeature(x) => x.direction_ref(),
            Feature::MetadataFeature(x) => x.direction_ref(),
            Feature::Multiplicity(x) => x.direction_ref(),
            Feature::Connector(x) => x.direction_ref(),
            Feature::Step(x) => x.direction_ref(),
            Feature::FlowEnd(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for FeatureRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_unique_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_unique_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_unique_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_unique_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_unique_ref_mut(),
            FeatureRefMut::Step(x) => x.is_unique_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_ordered_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_ordered_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_ordered_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_ordered_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_ordered_ref_mut(),
            FeatureRefMut::Step(x) => x.is_ordered_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_composite_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_composite_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_composite_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_composite_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_composite_ref_mut(),
            FeatureRefMut::Step(x) => x.is_composite_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_end_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_end_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_end_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_end_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_end_ref_mut(),
            FeatureRefMut::Step(x) => x.is_end_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_derived_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_derived_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_derived_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_derived_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_derived_ref_mut(),
            FeatureRefMut::Step(x) => x.is_derived_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_portion_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_portion_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_portion_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_portion_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_portion_ref_mut(),
            FeatureRefMut::Step(x) => x.is_portion_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_variable_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_variable_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_variable_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_variable_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_variable_ref_mut(),
            FeatureRefMut::Step(x) => x.is_variable_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_constant_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_constant_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_constant_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_constant_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_constant_ref_mut(),
            FeatureRefMut::Step(x) => x.is_constant_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_constant_ref_mut(),
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
            FeatureRefMut::Itself(x) => x.direction_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.direction_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.direction_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.direction_ref_mut(),
            FeatureRefMut::Connector(x) => x.direction_ref_mut(),
            FeatureRefMut::Step(x) => x.direction_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for FeatureRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_unique_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_unique_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_unique_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_unique_ref(),
            FeatureRefMut::Connector(x) => x.is_unique_ref(),
            FeatureRefMut::Step(x) => x.is_unique_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_ordered_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_ordered_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_ordered_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_ordered_ref(),
            FeatureRefMut::Connector(x) => x.is_ordered_ref(),
            FeatureRefMut::Step(x) => x.is_ordered_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_composite_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_composite_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_composite_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_composite_ref(),
            FeatureRefMut::Connector(x) => x.is_composite_ref(),
            FeatureRefMut::Step(x) => x.is_composite_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_end_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_end_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_end_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_end_ref(),
            FeatureRefMut::Connector(x) => x.is_end_ref(),
            FeatureRefMut::Step(x) => x.is_end_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_derived_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_derived_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_derived_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_derived_ref(),
            FeatureRefMut::Connector(x) => x.is_derived_ref(),
            FeatureRefMut::Step(x) => x.is_derived_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_portion_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_portion_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_portion_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_portion_ref(),
            FeatureRefMut::Connector(x) => x.is_portion_ref(),
            FeatureRefMut::Step(x) => x.is_portion_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_variable_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_variable_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_variable_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_variable_ref(),
            FeatureRefMut::Connector(x) => x.is_variable_ref(),
            FeatureRefMut::Step(x) => x.is_variable_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_constant_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_constant_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_constant_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_constant_ref(),
            FeatureRefMut::Connector(x) => x.is_constant_ref(),
            FeatureRefMut::Step(x) => x.is_constant_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_constant_ref(),
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
            FeatureRefMut::Itself(x) => x.direction_ref(),
            FeatureRefMut::PayloadFeature(x) => x.direction_ref(),
            FeatureRefMut::MetadataFeature(x) => x.direction_ref(),
            FeatureRefMut::Multiplicity(x) => x.direction_ref(),
            FeatureRefMut::Connector(x) => x.direction_ref(),
            FeatureRefMut::Step(x) => x.direction_ref(),
            FeatureRefMut::FlowEnd(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for FeatureRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_unique_ref(),
            FeatureRef::PayloadFeature(x) => x.is_unique_ref(),
            FeatureRef::MetadataFeature(x) => x.is_unique_ref(),
            FeatureRef::Multiplicity(x) => x.is_unique_ref(),
            FeatureRef::Connector(x) => x.is_unique_ref(),
            FeatureRef::Step(x) => x.is_unique_ref(),
            FeatureRef::FlowEnd(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_ordered_ref(),
            FeatureRef::PayloadFeature(x) => x.is_ordered_ref(),
            FeatureRef::MetadataFeature(x) => x.is_ordered_ref(),
            FeatureRef::Multiplicity(x) => x.is_ordered_ref(),
            FeatureRef::Connector(x) => x.is_ordered_ref(),
            FeatureRef::Step(x) => x.is_ordered_ref(),
            FeatureRef::FlowEnd(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_composite_ref(),
            FeatureRef::PayloadFeature(x) => x.is_composite_ref(),
            FeatureRef::MetadataFeature(x) => x.is_composite_ref(),
            FeatureRef::Multiplicity(x) => x.is_composite_ref(),
            FeatureRef::Connector(x) => x.is_composite_ref(),
            FeatureRef::Step(x) => x.is_composite_ref(),
            FeatureRef::FlowEnd(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_end_ref(),
            FeatureRef::PayloadFeature(x) => x.is_end_ref(),
            FeatureRef::MetadataFeature(x) => x.is_end_ref(),
            FeatureRef::Multiplicity(x) => x.is_end_ref(),
            FeatureRef::Connector(x) => x.is_end_ref(),
            FeatureRef::Step(x) => x.is_end_ref(),
            FeatureRef::FlowEnd(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_derived_ref(),
            FeatureRef::PayloadFeature(x) => x.is_derived_ref(),
            FeatureRef::MetadataFeature(x) => x.is_derived_ref(),
            FeatureRef::Multiplicity(x) => x.is_derived_ref(),
            FeatureRef::Connector(x) => x.is_derived_ref(),
            FeatureRef::Step(x) => x.is_derived_ref(),
            FeatureRef::FlowEnd(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_portion_ref(),
            FeatureRef::PayloadFeature(x) => x.is_portion_ref(),
            FeatureRef::MetadataFeature(x) => x.is_portion_ref(),
            FeatureRef::Multiplicity(x) => x.is_portion_ref(),
            FeatureRef::Connector(x) => x.is_portion_ref(),
            FeatureRef::Step(x) => x.is_portion_ref(),
            FeatureRef::FlowEnd(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_variable_ref(),
            FeatureRef::PayloadFeature(x) => x.is_variable_ref(),
            FeatureRef::MetadataFeature(x) => x.is_variable_ref(),
            FeatureRef::Multiplicity(x) => x.is_variable_ref(),
            FeatureRef::Connector(x) => x.is_variable_ref(),
            FeatureRef::Step(x) => x.is_variable_ref(),
            FeatureRef::FlowEnd(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_constant_ref(),
            FeatureRef::PayloadFeature(x) => x.is_constant_ref(),
            FeatureRef::MetadataFeature(x) => x.is_constant_ref(),
            FeatureRef::Multiplicity(x) => x.is_constant_ref(),
            FeatureRef::Connector(x) => x.is_constant_ref(),
            FeatureRef::Step(x) => x.is_constant_ref(),
            FeatureRef::FlowEnd(x) => x.is_constant_ref(),
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
            FeatureRef::Itself(x) => x.direction_ref(),
            FeatureRef::PayloadFeature(x) => x.direction_ref(),
            FeatureRef::MetadataFeature(x) => x.direction_ref(),
            FeatureRef::Multiplicity(x) => x.direction_ref(),
            FeatureRef::Connector(x) => x.direction_ref(),
            FeatureRef::Step(x) => x.direction_ref(),
            FeatureRef::FlowEnd(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Feature {
    fn is_abstract(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_abstract(),
            Feature::PayloadFeature(x) => x.is_abstract(),
            Feature::MetadataFeature(x) => x.is_abstract(),
            Feature::Multiplicity(x) => x.is_abstract(),
            Feature::Connector(x) => x.is_abstract(),
            Feature::Step(x) => x.is_abstract(),
            Feature::FlowEnd(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_sufficient(),
            Feature::PayloadFeature(x) => x.is_sufficient(),
            Feature::MetadataFeature(x) => x.is_sufficient(),
            Feature::Multiplicity(x) => x.is_sufficient(),
            Feature::Connector(x) => x.is_sufficient(),
            Feature::Step(x) => x.is_sufficient(),
            Feature::FlowEnd(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Feature {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_abstract_ref_mut(),
            Feature::PayloadFeature(x) => x.is_abstract_ref_mut(),
            Feature::MetadataFeature(x) => x.is_abstract_ref_mut(),
            Feature::Multiplicity(x) => x.is_abstract_ref_mut(),
            Feature::Connector(x) => x.is_abstract_ref_mut(),
            Feature::Step(x) => x.is_abstract_ref_mut(),
            Feature::FlowEnd(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_sufficient_ref_mut(),
            Feature::PayloadFeature(x) => x.is_sufficient_ref_mut(),
            Feature::MetadataFeature(x) => x.is_sufficient_ref_mut(),
            Feature::Multiplicity(x) => x.is_sufficient_ref_mut(),
            Feature::Connector(x) => x.is_sufficient_ref_mut(),
            Feature::Step(x) => x.is_sufficient_ref_mut(),
            Feature::FlowEnd(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Feature {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_abstract_ref(),
            Feature::PayloadFeature(x) => x.is_abstract_ref(),
            Feature::MetadataFeature(x) => x.is_abstract_ref(),
            Feature::Multiplicity(x) => x.is_abstract_ref(),
            Feature::Connector(x) => x.is_abstract_ref(),
            Feature::Step(x) => x.is_abstract_ref(),
            Feature::FlowEnd(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_sufficient_ref(),
            Feature::PayloadFeature(x) => x.is_sufficient_ref(),
            Feature::MetadataFeature(x) => x.is_sufficient_ref(),
            Feature::Multiplicity(x) => x.is_sufficient_ref(),
            Feature::Connector(x) => x.is_sufficient_ref(),
            Feature::Step(x) => x.is_sufficient_ref(),
            Feature::FlowEnd(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for FeatureRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_abstract_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_abstract_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_abstract_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_abstract_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_abstract_ref_mut(),
            FeatureRefMut::Step(x) => x.is_abstract_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_sufficient_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_sufficient_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_sufficient_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_sufficient_ref_mut(),
            FeatureRefMut::Step(x) => x.is_sufficient_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for FeatureRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_abstract_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_abstract_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_abstract_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_abstract_ref(),
            FeatureRefMut::Connector(x) => x.is_abstract_ref(),
            FeatureRefMut::Step(x) => x.is_abstract_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_sufficient_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_sufficient_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_sufficient_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_sufficient_ref(),
            FeatureRefMut::Connector(x) => x.is_sufficient_ref(),
            FeatureRefMut::Step(x) => x.is_sufficient_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for FeatureRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_abstract_ref(),
            FeatureRef::PayloadFeature(x) => x.is_abstract_ref(),
            FeatureRef::MetadataFeature(x) => x.is_abstract_ref(),
            FeatureRef::Multiplicity(x) => x.is_abstract_ref(),
            FeatureRef::Connector(x) => x.is_abstract_ref(),
            FeatureRef::Step(x) => x.is_abstract_ref(),
            FeatureRef::FlowEnd(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_sufficient_ref(),
            FeatureRef::PayloadFeature(x) => x.is_sufficient_ref(),
            FeatureRef::MetadataFeature(x) => x.is_sufficient_ref(),
            FeatureRef::Multiplicity(x) => x.is_sufficient_ref(),
            FeatureRef::Connector(x) => x.is_sufficient_ref(),
            FeatureRef::Step(x) => x.is_sufficient_ref(),
            FeatureRef::FlowEnd(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Feature {}
impl NamespaceStructRefMut for Feature {}
impl NamespaceStructRef for Feature {}
impl<'a> NamespaceStructRefMut for FeatureRefMut<'a> {}
impl<'a> NamespaceStructRef for FeatureRefMut<'a> {}
impl<'a> NamespaceStructRef for FeatureRef<'a> {}
impl ElementStruct for Feature {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Feature::Itself(x) => x.owned_relationship(),
            Feature::PayloadFeature(x) => x.owned_relationship(),
            Feature::MetadataFeature(x) => x.owned_relationship(),
            Feature::Multiplicity(x) => x.owned_relationship(),
            Feature::Connector(x) => x.owned_relationship(),
            Feature::Step(x) => x.owned_relationship(),
            Feature::FlowEnd(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Feature::Itself(x) => x.owning_relationship(),
            Feature::PayloadFeature(x) => x.owning_relationship(),
            Feature::MetadataFeature(x) => x.owning_relationship(),
            Feature::Multiplicity(x) => x.owning_relationship(),
            Feature::Connector(x) => x.owning_relationship(),
            Feature::Step(x) => x.owning_relationship(),
            Feature::FlowEnd(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Feature::Itself(x) => x.element_id(),
            Feature::PayloadFeature(x) => x.element_id(),
            Feature::MetadataFeature(x) => x.element_id(),
            Feature::Multiplicity(x) => x.element_id(),
            Feature::Connector(x) => x.element_id(),
            Feature::Step(x) => x.element_id(),
            Feature::FlowEnd(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Feature::Itself(x) => x.alias_ids(),
            Feature::PayloadFeature(x) => x.alias_ids(),
            Feature::MetadataFeature(x) => x.alias_ids(),
            Feature::Multiplicity(x) => x.alias_ids(),
            Feature::Connector(x) => x.alias_ids(),
            Feature::Step(x) => x.alias_ids(),
            Feature::FlowEnd(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Feature::Itself(x) => x.declared_short_name(),
            Feature::PayloadFeature(x) => x.declared_short_name(),
            Feature::MetadataFeature(x) => x.declared_short_name(),
            Feature::Multiplicity(x) => x.declared_short_name(),
            Feature::Connector(x) => x.declared_short_name(),
            Feature::Step(x) => x.declared_short_name(),
            Feature::FlowEnd(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Feature::Itself(x) => x.declared_name(),
            Feature::PayloadFeature(x) => x.declared_name(),
            Feature::MetadataFeature(x) => x.declared_name(),
            Feature::Multiplicity(x) => x.declared_name(),
            Feature::Connector(x) => x.declared_name(),
            Feature::Step(x) => x.declared_name(),
            Feature::FlowEnd(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Feature::Itself(x) => x.is_implied_included(),
            Feature::PayloadFeature(x) => x.is_implied_included(),
            Feature::MetadataFeature(x) => x.is_implied_included(),
            Feature::Multiplicity(x) => x.is_implied_included(),
            Feature::Connector(x) => x.is_implied_included(),
            Feature::Step(x) => x.is_implied_included(),
            Feature::FlowEnd(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Feature {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Feature::Itself(x) => x.owned_relationship_ref_mut(),
            Feature::PayloadFeature(x) => x.owned_relationship_ref_mut(),
            Feature::MetadataFeature(x) => x.owned_relationship_ref_mut(),
            Feature::Multiplicity(x) => x.owned_relationship_ref_mut(),
            Feature::Connector(x) => x.owned_relationship_ref_mut(),
            Feature::Step(x) => x.owned_relationship_ref_mut(),
            Feature::FlowEnd(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Feature::Itself(x) => x.owning_relationship_ref_mut(),
            Feature::PayloadFeature(x) => x.owning_relationship_ref_mut(),
            Feature::MetadataFeature(x) => x.owning_relationship_ref_mut(),
            Feature::Multiplicity(x) => x.owning_relationship_ref_mut(),
            Feature::Connector(x) => x.owning_relationship_ref_mut(),
            Feature::Step(x) => x.owning_relationship_ref_mut(),
            Feature::FlowEnd(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Feature::Itself(x) => x.element_id_ref_mut(),
            Feature::PayloadFeature(x) => x.element_id_ref_mut(),
            Feature::MetadataFeature(x) => x.element_id_ref_mut(),
            Feature::Multiplicity(x) => x.element_id_ref_mut(),
            Feature::Connector(x) => x.element_id_ref_mut(),
            Feature::Step(x) => x.element_id_ref_mut(),
            Feature::FlowEnd(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Feature::Itself(x) => x.alias_ids_ref_mut(),
            Feature::PayloadFeature(x) => x.alias_ids_ref_mut(),
            Feature::MetadataFeature(x) => x.alias_ids_ref_mut(),
            Feature::Multiplicity(x) => x.alias_ids_ref_mut(),
            Feature::Connector(x) => x.alias_ids_ref_mut(),
            Feature::Step(x) => x.alias_ids_ref_mut(),
            Feature::FlowEnd(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Feature::Itself(x) => x.declared_short_name_ref_mut(),
            Feature::PayloadFeature(x) => x.declared_short_name_ref_mut(),
            Feature::MetadataFeature(x) => x.declared_short_name_ref_mut(),
            Feature::Multiplicity(x) => x.declared_short_name_ref_mut(),
            Feature::Connector(x) => x.declared_short_name_ref_mut(),
            Feature::Step(x) => x.declared_short_name_ref_mut(),
            Feature::FlowEnd(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Feature::Itself(x) => x.declared_name_ref_mut(),
            Feature::PayloadFeature(x) => x.declared_name_ref_mut(),
            Feature::MetadataFeature(x) => x.declared_name_ref_mut(),
            Feature::Multiplicity(x) => x.declared_name_ref_mut(),
            Feature::Connector(x) => x.declared_name_ref_mut(),
            Feature::Step(x) => x.declared_name_ref_mut(),
            Feature::FlowEnd(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Feature::Itself(x) => x.is_implied_included_ref_mut(),
            Feature::PayloadFeature(x) => x.is_implied_included_ref_mut(),
            Feature::MetadataFeature(x) => x.is_implied_included_ref_mut(),
            Feature::Multiplicity(x) => x.is_implied_included_ref_mut(),
            Feature::Connector(x) => x.is_implied_included_ref_mut(),
            Feature::Step(x) => x.is_implied_included_ref_mut(),
            Feature::FlowEnd(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Feature {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Feature::Itself(x) => x.owned_relationship_ref(),
            Feature::PayloadFeature(x) => x.owned_relationship_ref(),
            Feature::MetadataFeature(x) => x.owned_relationship_ref(),
            Feature::Multiplicity(x) => x.owned_relationship_ref(),
            Feature::Connector(x) => x.owned_relationship_ref(),
            Feature::Step(x) => x.owned_relationship_ref(),
            Feature::FlowEnd(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Feature::Itself(x) => x.owning_relationship_ref(),
            Feature::PayloadFeature(x) => x.owning_relationship_ref(),
            Feature::MetadataFeature(x) => x.owning_relationship_ref(),
            Feature::Multiplicity(x) => x.owning_relationship_ref(),
            Feature::Connector(x) => x.owning_relationship_ref(),
            Feature::Step(x) => x.owning_relationship_ref(),
            Feature::FlowEnd(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Feature::Itself(x) => x.element_id_ref(),
            Feature::PayloadFeature(x) => x.element_id_ref(),
            Feature::MetadataFeature(x) => x.element_id_ref(),
            Feature::Multiplicity(x) => x.element_id_ref(),
            Feature::Connector(x) => x.element_id_ref(),
            Feature::Step(x) => x.element_id_ref(),
            Feature::FlowEnd(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Feature::Itself(x) => x.alias_ids_ref(),
            Feature::PayloadFeature(x) => x.alias_ids_ref(),
            Feature::MetadataFeature(x) => x.alias_ids_ref(),
            Feature::Multiplicity(x) => x.alias_ids_ref(),
            Feature::Connector(x) => x.alias_ids_ref(),
            Feature::Step(x) => x.alias_ids_ref(),
            Feature::FlowEnd(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Feature::Itself(x) => x.declared_short_name_ref(),
            Feature::PayloadFeature(x) => x.declared_short_name_ref(),
            Feature::MetadataFeature(x) => x.declared_short_name_ref(),
            Feature::Multiplicity(x) => x.declared_short_name_ref(),
            Feature::Connector(x) => x.declared_short_name_ref(),
            Feature::Step(x) => x.declared_short_name_ref(),
            Feature::FlowEnd(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Feature::Itself(x) => x.declared_name_ref(),
            Feature::PayloadFeature(x) => x.declared_name_ref(),
            Feature::MetadataFeature(x) => x.declared_name_ref(),
            Feature::Multiplicity(x) => x.declared_name_ref(),
            Feature::Connector(x) => x.declared_name_ref(),
            Feature::Step(x) => x.declared_name_ref(),
            Feature::FlowEnd(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Feature::Itself(x) => x.is_implied_included_ref(),
            Feature::PayloadFeature(x) => x.is_implied_included_ref(),
            Feature::MetadataFeature(x) => x.is_implied_included_ref(),
            Feature::Multiplicity(x) => x.is_implied_included_ref(),
            Feature::Connector(x) => x.is_implied_included_ref(),
            Feature::Step(x) => x.is_implied_included_ref(),
            Feature::FlowEnd(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.owned_relationship_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.owned_relationship_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.owned_relationship_ref_mut(),
            FeatureRefMut::Connector(x) => x.owned_relationship_ref_mut(),
            FeatureRefMut::Step(x) => x.owned_relationship_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.owning_relationship_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.owning_relationship_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.owning_relationship_ref_mut(),
            FeatureRefMut::Connector(x) => x.owning_relationship_ref_mut(),
            FeatureRefMut::Step(x) => x.owning_relationship_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureRefMut::Itself(x) => x.element_id_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.element_id_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.element_id_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.element_id_ref_mut(),
            FeatureRefMut::Connector(x) => x.element_id_ref_mut(),
            FeatureRefMut::Step(x) => x.element_id_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureRefMut::Itself(x) => x.alias_ids_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.alias_ids_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.alias_ids_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.alias_ids_ref_mut(),
            FeatureRefMut::Connector(x) => x.alias_ids_ref_mut(),
            FeatureRefMut::Step(x) => x.alias_ids_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.declared_short_name_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.declared_short_name_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.declared_short_name_ref_mut(),
            FeatureRefMut::Connector(x) => x.declared_short_name_ref_mut(),
            FeatureRefMut::Step(x) => x.declared_short_name_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureRefMut::Itself(x) => x.declared_name_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.declared_name_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.declared_name_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.declared_name_ref_mut(),
            FeatureRefMut::Connector(x) => x.declared_name_ref_mut(),
            FeatureRefMut::Step(x) => x.declared_name_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            FeatureRefMut::PayloadFeature(x) => x.is_implied_included_ref_mut(),
            FeatureRefMut::MetadataFeature(x) => x.is_implied_included_ref_mut(),
            FeatureRefMut::Multiplicity(x) => x.is_implied_included_ref_mut(),
            FeatureRefMut::Connector(x) => x.is_implied_included_ref_mut(),
            FeatureRefMut::Step(x) => x.is_implied_included_ref_mut(),
            FeatureRefMut::FlowEnd(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for FeatureRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureRefMut::Itself(x) => x.owned_relationship_ref(),
            FeatureRefMut::PayloadFeature(x) => x.owned_relationship_ref(),
            FeatureRefMut::MetadataFeature(x) => x.owned_relationship_ref(),
            FeatureRefMut::Multiplicity(x) => x.owned_relationship_ref(),
            FeatureRefMut::Connector(x) => x.owned_relationship_ref(),
            FeatureRefMut::Step(x) => x.owned_relationship_ref(),
            FeatureRefMut::FlowEnd(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureRefMut::Itself(x) => x.owning_relationship_ref(),
            FeatureRefMut::PayloadFeature(x) => x.owning_relationship_ref(),
            FeatureRefMut::MetadataFeature(x) => x.owning_relationship_ref(),
            FeatureRefMut::Multiplicity(x) => x.owning_relationship_ref(),
            FeatureRefMut::Connector(x) => x.owning_relationship_ref(),
            FeatureRefMut::Step(x) => x.owning_relationship_ref(),
            FeatureRefMut::FlowEnd(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureRefMut::Itself(x) => x.element_id_ref(),
            FeatureRefMut::PayloadFeature(x) => x.element_id_ref(),
            FeatureRefMut::MetadataFeature(x) => x.element_id_ref(),
            FeatureRefMut::Multiplicity(x) => x.element_id_ref(),
            FeatureRefMut::Connector(x) => x.element_id_ref(),
            FeatureRefMut::Step(x) => x.element_id_ref(),
            FeatureRefMut::FlowEnd(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureRefMut::Itself(x) => x.alias_ids_ref(),
            FeatureRefMut::PayloadFeature(x) => x.alias_ids_ref(),
            FeatureRefMut::MetadataFeature(x) => x.alias_ids_ref(),
            FeatureRefMut::Multiplicity(x) => x.alias_ids_ref(),
            FeatureRefMut::Connector(x) => x.alias_ids_ref(),
            FeatureRefMut::Step(x) => x.alias_ids_ref(),
            FeatureRefMut::FlowEnd(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureRefMut::Itself(x) => x.declared_short_name_ref(),
            FeatureRefMut::PayloadFeature(x) => x.declared_short_name_ref(),
            FeatureRefMut::MetadataFeature(x) => x.declared_short_name_ref(),
            FeatureRefMut::Multiplicity(x) => x.declared_short_name_ref(),
            FeatureRefMut::Connector(x) => x.declared_short_name_ref(),
            FeatureRefMut::Step(x) => x.declared_short_name_ref(),
            FeatureRefMut::FlowEnd(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureRefMut::Itself(x) => x.declared_name_ref(),
            FeatureRefMut::PayloadFeature(x) => x.declared_name_ref(),
            FeatureRefMut::MetadataFeature(x) => x.declared_name_ref(),
            FeatureRefMut::Multiplicity(x) => x.declared_name_ref(),
            FeatureRefMut::Connector(x) => x.declared_name_ref(),
            FeatureRefMut::Step(x) => x.declared_name_ref(),
            FeatureRefMut::FlowEnd(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureRefMut::Itself(x) => x.is_implied_included_ref(),
            FeatureRefMut::PayloadFeature(x) => x.is_implied_included_ref(),
            FeatureRefMut::MetadataFeature(x) => x.is_implied_included_ref(),
            FeatureRefMut::Multiplicity(x) => x.is_implied_included_ref(),
            FeatureRefMut::Connector(x) => x.is_implied_included_ref(),
            FeatureRefMut::Step(x) => x.is_implied_included_ref(),
            FeatureRefMut::FlowEnd(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureRef::Itself(x) => x.owned_relationship_ref(),
            FeatureRef::PayloadFeature(x) => x.owned_relationship_ref(),
            FeatureRef::MetadataFeature(x) => x.owned_relationship_ref(),
            FeatureRef::Multiplicity(x) => x.owned_relationship_ref(),
            FeatureRef::Connector(x) => x.owned_relationship_ref(),
            FeatureRef::Step(x) => x.owned_relationship_ref(),
            FeatureRef::FlowEnd(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureRef::Itself(x) => x.owning_relationship_ref(),
            FeatureRef::PayloadFeature(x) => x.owning_relationship_ref(),
            FeatureRef::MetadataFeature(x) => x.owning_relationship_ref(),
            FeatureRef::Multiplicity(x) => x.owning_relationship_ref(),
            FeatureRef::Connector(x) => x.owning_relationship_ref(),
            FeatureRef::Step(x) => x.owning_relationship_ref(),
            FeatureRef::FlowEnd(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureRef::Itself(x) => x.element_id_ref(),
            FeatureRef::PayloadFeature(x) => x.element_id_ref(),
            FeatureRef::MetadataFeature(x) => x.element_id_ref(),
            FeatureRef::Multiplicity(x) => x.element_id_ref(),
            FeatureRef::Connector(x) => x.element_id_ref(),
            FeatureRef::Step(x) => x.element_id_ref(),
            FeatureRef::FlowEnd(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureRef::Itself(x) => x.alias_ids_ref(),
            FeatureRef::PayloadFeature(x) => x.alias_ids_ref(),
            FeatureRef::MetadataFeature(x) => x.alias_ids_ref(),
            FeatureRef::Multiplicity(x) => x.alias_ids_ref(),
            FeatureRef::Connector(x) => x.alias_ids_ref(),
            FeatureRef::Step(x) => x.alias_ids_ref(),
            FeatureRef::FlowEnd(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureRef::Itself(x) => x.declared_short_name_ref(),
            FeatureRef::PayloadFeature(x) => x.declared_short_name_ref(),
            FeatureRef::MetadataFeature(x) => x.declared_short_name_ref(),
            FeatureRef::Multiplicity(x) => x.declared_short_name_ref(),
            FeatureRef::Connector(x) => x.declared_short_name_ref(),
            FeatureRef::Step(x) => x.declared_short_name_ref(),
            FeatureRef::FlowEnd(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureRef::Itself(x) => x.declared_name_ref(),
            FeatureRef::PayloadFeature(x) => x.declared_name_ref(),
            FeatureRef::MetadataFeature(x) => x.declared_name_ref(),
            FeatureRef::Multiplicity(x) => x.declared_name_ref(),
            FeatureRef::Connector(x) => x.declared_name_ref(),
            FeatureRef::Step(x) => x.declared_name_ref(),
            FeatureRef::FlowEnd(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureRef::Itself(x) => x.is_implied_included_ref(),
            FeatureRef::PayloadFeature(x) => x.is_implied_included_ref(),
            FeatureRef::MetadataFeature(x) => x.is_implied_included_ref(),
            FeatureRef::Multiplicity(x) => x.is_implied_included_ref(),
            FeatureRef::Connector(x) => x.is_implied_included_ref(),
            FeatureRef::Step(x) => x.is_implied_included_ref(),
            FeatureRef::FlowEnd(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureUpcast for Feature {
    fn into_feature(self) -> Feature {
        self
    }
}
impl<'a> FeatureUpcastRefMut<'a> for FeatureRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        self
    }
}
impl<'a> FeatureUpcastRef<'a> for FeatureRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        self
    }
}
impl TypeUpcast for Feature {
    fn into_type_(self) -> Type {
        Type::Feature(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for FeatureRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        TypeRefMut::Feature(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for FeatureRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        TypeRef::Feature(self).as_type_ref()
    }
}
impl NamespaceUpcast for Feature {
    fn into_namespace(self) -> Namespace {
        Type::Feature(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for FeatureRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        TypeRefMut::Feature(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for FeatureRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        TypeRef::Feature(self).as_namespace_ref()
    }
}
impl ElementUpcast for Feature {
    fn into_element(self) -> Element {
        Type::Feature(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        TypeRefMut::Feature(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        TypeRef::Feature(self).as_element_ref()
    }
}
pub trait FeatureDowncast {
    fn try_into_payload_feature(self) -> Result<PayloadFeature, String>;
    fn try_into_metadata_feature(self) -> Result<MetadataFeature, String>;
    fn try_into_multiplicity(self) -> Result<Multiplicity, String>;
    fn try_into_connector(self) -> Result<Connector, String>;
    fn try_into_step(self) -> Result<Step, String>;
    fn try_into_flow_end(self) -> Result<FlowEnd, String>;
}
pub trait FeatureDowncastRefMut<'a> {
    fn try_as_payload_feature_ref_mut(self) -> Result<PayloadFeatureRefMut<'a>, String>;
    fn try_as_metadata_feature_ref_mut(
        self,
    ) -> Result<MetadataFeatureRefMut<'a>, String>;
    fn try_as_multiplicity_ref_mut(self) -> Result<MultiplicityRefMut<'a>, String>;
    fn try_as_connector_ref_mut(self) -> Result<ConnectorRefMut<'a>, String>;
    fn try_as_step_ref_mut(self) -> Result<StepRefMut<'a>, String>;
    fn try_as_flow_end_ref_mut(self) -> Result<FlowEndRefMut<'a>, String>;
}
pub trait FeatureDowncastRef<'a> {
    fn try_as_payload_feature_ref(self) -> Result<PayloadFeatureRef<'a>, String>;
    fn try_as_metadata_feature_ref(self) -> Result<MetadataFeatureRef<'a>, String>;
    fn try_as_multiplicity_ref(self) -> Result<MultiplicityRef<'a>, String>;
    fn try_as_connector_ref(self) -> Result<ConnectorRef<'a>, String>;
    fn try_as_step_ref(self) -> Result<StepRef<'a>, String>;
    fn try_as_flow_end_ref(self) -> Result<FlowEndRef<'a>, String>;
}
impl FeatureDowncast for Feature {
    fn try_into_payload_feature(self) -> Result<PayloadFeature, String> {
        match self {
            Feature::PayloadFeature(e) => Ok(e),
            _ => Err("Not a PayloadFeature".into()),
        }
    }
    fn try_into_metadata_feature(self) -> Result<MetadataFeature, String> {
        match self {
            Feature::MetadataFeature(e) => Ok(e),
            _ => Err("Not a MetadataFeature".into()),
        }
    }
    fn try_into_multiplicity(self) -> Result<Multiplicity, String> {
        match self {
            Feature::Multiplicity(e) => Ok(e),
            _ => Err("Not a Multiplicity".into()),
        }
    }
    fn try_into_connector(self) -> Result<Connector, String> {
        match self {
            Feature::Connector(e) => Ok(e),
            _ => Err("Not a Connector".into()),
        }
    }
    fn try_into_step(self) -> Result<Step, String> {
        match self {
            Feature::Step(e) => Ok(e),
            _ => Err("Not a Step".into()),
        }
    }
    fn try_into_flow_end(self) -> Result<FlowEnd, String> {
        match self {
            Feature::FlowEnd(e) => Ok(e),
            _ => Err("Not a FlowEnd".into()),
        }
    }
}
impl<'a> FeatureDowncastRefMut<'a> for FeatureRefMut<'a> {
    fn try_as_payload_feature_ref_mut(self) -> Result<PayloadFeatureRefMut<'a>, String> {
        match self {
            FeatureRefMut::PayloadFeature(e) => Ok(e),
            _ => Err("Not a PayloadFeature".into()),
        }
    }
    fn try_as_metadata_feature_ref_mut(
        self,
    ) -> Result<MetadataFeatureRefMut<'a>, String> {
        match self {
            FeatureRefMut::MetadataFeature(e) => Ok(e),
            _ => Err("Not a MetadataFeature".into()),
        }
    }
    fn try_as_multiplicity_ref_mut(self) -> Result<MultiplicityRefMut<'a>, String> {
        match self {
            FeatureRefMut::Multiplicity(e) => Ok(e),
            _ => Err("Not a Multiplicity".into()),
        }
    }
    fn try_as_connector_ref_mut(self) -> Result<ConnectorRefMut<'a>, String> {
        match self {
            FeatureRefMut::Connector(e) => Ok(e),
            _ => Err("Not a Connector".into()),
        }
    }
    fn try_as_step_ref_mut(self) -> Result<StepRefMut<'a>, String> {
        match self {
            FeatureRefMut::Step(e) => Ok(e),
            _ => Err("Not a Step".into()),
        }
    }
    fn try_as_flow_end_ref_mut(self) -> Result<FlowEndRefMut<'a>, String> {
        match self {
            FeatureRefMut::FlowEnd(e) => Ok(e),
            _ => Err("Not a FlowEnd".into()),
        }
    }
}
impl<'a> FeatureDowncastRef<'a> for FeatureRef<'a> {
    fn try_as_payload_feature_ref(self) -> Result<PayloadFeatureRef<'a>, String> {
        match self {
            FeatureRef::PayloadFeature(e) => Ok(e),
            _ => Err("Not a PayloadFeature".into()),
        }
    }
    fn try_as_metadata_feature_ref(self) -> Result<MetadataFeatureRef<'a>, String> {
        match self {
            FeatureRef::MetadataFeature(e) => Ok(e),
            _ => Err("Not a MetadataFeature".into()),
        }
    }
    fn try_as_multiplicity_ref(self) -> Result<MultiplicityRef<'a>, String> {
        match self {
            FeatureRef::Multiplicity(e) => Ok(e),
            _ => Err("Not a Multiplicity".into()),
        }
    }
    fn try_as_connector_ref(self) -> Result<ConnectorRef<'a>, String> {
        match self {
            FeatureRef::Connector(e) => Ok(e),
            _ => Err("Not a Connector".into()),
        }
    }
    fn try_as_step_ref(self) -> Result<StepRef<'a>, String> {
        match self {
            FeatureRef::Step(e) => Ok(e),
            _ => Err("Not a Step".into()),
        }
    }
    fn try_as_flow_end_ref(self) -> Result<FlowEndRef<'a>, String> {
        match self {
            FeatureRef::FlowEnd(e) => Ok(e),
            _ => Err("Not a FlowEnd".into()),
        }
    }
}
pub trait FeatureMethodsDescendants
where
    Self: DescendantOf<Feature>,
    Self::Via: FeatureMethods,
    Self: Sized,
{}
pub trait FeatureMethods: Sized {}
impl<T: FeatureMethodsDescendants> FeatureMethods for T
where
    T::Via: FeatureMethods,
{}
impl DescendantOf<Type> for Feature {
    type Via = Type;
    fn into_via(self) -> Self::Via {
        self.into_type_()
    }
}
impl TypeMethodsDescendants for Feature {}
impl DescendantOf<Namespace> for Feature {
    type Via = Type;
    fn into_via(self) -> Self::Via {
        self.into_type_()
    }
}
impl NamespaceMethodsDescendants for Feature {}
impl DescendantOf<Element> for Feature {
    type Via = Type;
    fn into_via(self) -> Self::Via {
        self.into_type_()
    }
}
impl ElementMethodsDescendants for Feature {}
pub trait FeatureRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureRefMut<'a>>,
    Self::Via: FeatureRefMutMethods,
    Self: Sized,
{}
pub trait FeatureRefMutMethods: Sized {}
impl<'a, T: FeatureRefMutMethodsDescendants<'a>> FeatureRefMutMethods for T
where
    T::Via: FeatureRefMutMethods,
{}
impl<'a> DescendantOf<TypeRefMut<'a>> for FeatureRefMut<'a> {
    type Via = TypeRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for FeatureRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for FeatureRefMut<'a> {
    type Via = TypeRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for FeatureRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureRefMut<'a> {
    type Via = TypeRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureRefMut<'a> {}
pub trait FeatureRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureRef<'a>>,
    Self::Via: FeatureRefMethods,
    Self: Sized,
{
    fn direction_for_impl(
        self,
        type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.into_via().direction_for(type_)
    }
    fn naming_feature_impl(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        self.into_via().naming_feature()
    }
    fn redefines_impl(
        self,
        redefined_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool {
        self.into_via().redefines(redefined_feature)
    }
    fn redefines_from_library_impl(self, library_feature_name: String) -> bool {
        self.into_via().redefines_from_library(library_feature_name)
    }
    fn subsets_chain_impl(
        self,
        first: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
        second: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool {
        self.into_via().subsets_chain(first, second)
    }
    fn typing_features_impl(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        self.into_via().typing_features()
    }
    fn as_cartesian_product_impl(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        self.into_via().as_cartesian_product()
    }
    fn is_cartesian_product_impl(self) -> bool {
        self.into_via().is_cartesian_product()
    }
    fn is_owned_cross_feature_impl(self) -> bool {
        self.into_via().is_owned_cross_feature()
    }
    fn owned_cross_feature_impl(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        self.into_via().owned_cross_feature()
    }
    fn all_redefined_features_impl(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        self.into_via().all_redefined_features()
    }
    fn is_featured_within_impl(
        self,
        type_: Option<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
    ) -> bool {
        self.into_via().is_featured_within(type_)
    }
    fn can_access_impl(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool {
        self.into_via().can_access(feature)
    }
    fn is_featuring_type_impl(
        self,
        type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> bool {
        self.into_via().is_featuring_type(type_)
    }
}
pub trait FeatureRefMethods: Sized {
    fn direction_for(
        self,
        type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    >;
    fn naming_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>;
    fn redefines(
        self,
        redefined_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool;
    fn redefines_from_library(self, library_feature_name: String) -> bool;
    fn subsets_chain(
        self,
        first: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
        second: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool;
    fn typing_features(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>;
    fn as_cartesian_product(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>;
    fn is_cartesian_product(self) -> bool;
    fn is_owned_cross_feature(self) -> bool;
    fn owned_cross_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>;
    fn all_redefined_features(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>;
    fn is_featured_within(
        self,
        type_: Option<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
    ) -> bool;
    fn can_access(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool;
    fn is_featuring_type(
        self,
        type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> bool;
}
impl<'a, T: FeatureRefMethodsDescendants<'a>> FeatureRefMethods for T
where
    T::Via: FeatureRefMethods,
{
    fn direction_for(
        self,
        type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        FeatureRefMethodsDescendants::direction_for_impl(self, type_)
    }
    fn naming_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        FeatureRefMethodsDescendants::naming_feature_impl(self)
    }
    fn redefines(
        self,
        redefined_feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool {
        FeatureRefMethodsDescendants::redefines_impl(self, redefined_feature)
    }
    fn redefines_from_library(self, library_feature_name: String) -> bool {
        FeatureRefMethodsDescendants::redefines_from_library_impl(
            self,
            library_feature_name,
        )
    }
    fn subsets_chain(
        self,
        first: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
        second: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool {
        FeatureRefMethodsDescendants::subsets_chain_impl(self, first, second)
    }
    fn typing_features(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        FeatureRefMethodsDescendants::typing_features_impl(self)
    }
    fn as_cartesian_product(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        FeatureRefMethodsDescendants::as_cartesian_product_impl(self)
    }
    fn is_cartesian_product(self) -> bool {
        FeatureRefMethodsDescendants::is_cartesian_product_impl(self)
    }
    fn is_owned_cross_feature(self) -> bool {
        FeatureRefMethodsDescendants::is_owned_cross_feature_impl(self)
    }
    fn owned_cross_feature(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        FeatureRefMethodsDescendants::owned_cross_feature_impl(self)
    }
    fn all_redefined_features(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>> {
        FeatureRefMethodsDescendants::all_redefined_features_impl(self)
    }
    fn is_featured_within(
        self,
        type_: Option<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>,
    ) -> bool {
        FeatureRefMethodsDescendants::is_featured_within_impl(self, type_)
    }
    fn can_access(
        self,
        feature: std::rc::Rc<std::cell::RefCell<super::feature::Feature>>,
    ) -> bool {
        FeatureRefMethodsDescendants::can_access_impl(self, feature)
    }
    fn is_featuring_type(
        self,
        type_: std::rc::Rc<std::cell::RefCell<super::type_::Type>>,
    ) -> bool {
        FeatureRefMethodsDescendants::is_featuring_type_impl(self, type_)
    }
}
impl<'a> DescendantOf<TypeRef<'a>> for FeatureRef<'a> {
    type Via = TypeRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for FeatureRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for FeatureRef<'a> {
    type Via = TypeRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for FeatureRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureRef<'a> {
    type Via = TypeRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_type_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureRef<'a> {}

