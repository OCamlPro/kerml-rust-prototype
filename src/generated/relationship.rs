#![allow(unused)]
use super::utils::DescendantOf;
use super::element::{
    Element, ElementRefMut, ElementRef, ElementStruct, ElementStructRefMut,
    ElementStructRef, ElementInner, ElementUpcast, ElementUpcastRefMut, ElementUpcastRef,
    ElementMethodsDescendants, ElementRefMutMethodsDescendants,
    ElementRefMethodsDescendants,
};
use super::association::{Association, AssociationRefMut, AssociationRef};
use super::connector::{Connector, ConnectorRefMut, ConnectorRef};
use super::import::{Import, ImportRefMut, ImportRef};
use super::disjoining::{Disjoining, DisjoiningRefMut, DisjoiningRef};
use super::feature_inverting::{
    FeatureInverting, FeatureInvertingRefMut, FeatureInvertingRef,
};
use super::feature_chaining::{
    FeatureChaining, FeatureChainingRefMut, FeatureChainingRef,
};
use super::annotation::{Annotation, AnnotationRefMut, AnnotationRef};
use super::dependency::{Dependency, DependencyRefMut, DependencyRef};
use super::differencing::{Differencing, DifferencingRefMut, DifferencingRef};
use super::type_featuring::{TypeFeaturing, TypeFeaturingRefMut, TypeFeaturingRef};
use super::specialization::{Specialization, SpecializationRefMut, SpecializationRef};
use super::intersecting::{Intersecting, IntersectingRefMut, IntersectingRef};
use super::conjugation::{Conjugation, ConjugationRefMut, ConjugationRef};
use super::membership::{Membership, MembershipRefMut, MembershipRef};
use super::unioning::{Unioning, UnioningRefMut, UnioningRef};
pub struct RelationshipInner {
    pub(super) sup_element: ElementInner,
    pub(super) target: Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>,
    pub(super) source: Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>,
    pub(super) owning_related_element: Option<
        std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    >,
    pub(super) owned_related_element: Vec<
        std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    >,
    pub(super) is_implied: bool,
}
pub trait RelationshipStruct
where
    Self: RelationshipStructRefMut,
    Self: RelationshipStructRef,
    Self: ElementStruct,
{
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn is_implied(self) -> bool;
}
pub trait RelationshipStructRefMut
where
    Self: RelationshipStructRef,
    Self: ElementStructRefMut,
{
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn is_implied_ref_mut(&mut self) -> &mut bool;
}
pub trait RelationshipStructRef
where
    Self: ElementStructRef,
{
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn is_implied_ref(&self) -> &bool;
}
pub trait RelationshipUpcast: RelationshipStruct {
    fn into_relationship(self) -> Relationship;
}
pub trait RelationshipUpcastRefMut<'a>: RelationshipStructRefMut {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a>;
}
pub trait RelationshipUpcastRef<'a>: RelationshipStructRef {
    fn as_relationship_ref(self) -> RelationshipRef<'a>;
}
impl RelationshipStruct for RelationshipInner {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.target
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.source
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.owning_related_element
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.owned_related_element
    }
    fn is_implied(self) -> bool {
        self.is_implied
    }
}
impl RelationshipStructRefMut for RelationshipInner {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &mut self.target
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &mut self.source
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &mut self.owning_related_element
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &mut self.owned_related_element
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        &mut self.is_implied
    }
}
impl RelationshipStructRef for RelationshipInner {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &self.target
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &self.source
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &self.owning_related_element
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        &self.owned_related_element
    }
    fn is_implied_ref(&self) -> &bool {
        &self.is_implied
    }
}
impl ElementStruct for RelationshipInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_element.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_element.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_element.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_element.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_element.is_implied_included()
    }
}
impl ElementStructRefMut for RelationshipInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_element.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_element.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_element.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_element.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_element.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_element.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for RelationshipInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_element.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_element.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_element.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_element.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_element.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_element.is_implied_included_ref()
    }
}
pub enum Relationship {
    Itself(RelationshipInner),
    Association(Association),
    Connector(Connector),
    Import(Import),
    Disjoining(Disjoining),
    FeatureInverting(FeatureInverting),
    FeatureChaining(FeatureChaining),
    Annotation(Annotation),
    Dependency(Dependency),
    Differencing(Differencing),
    TypeFeaturing(TypeFeaturing),
    Specialization(Specialization),
    Intersecting(Intersecting),
    Conjugation(Conjugation),
    Membership(Membership),
    Unioning(Unioning),
}
pub enum RelationshipRefMut<'a> {
    Itself(&'a mut RelationshipInner),
    Association(AssociationRefMut<'a>),
    Connector(ConnectorRefMut<'a>),
    Import(ImportRefMut<'a>),
    Disjoining(DisjoiningRefMut<'a>),
    FeatureInverting(FeatureInvertingRefMut<'a>),
    FeatureChaining(FeatureChainingRefMut<'a>),
    Annotation(AnnotationRefMut<'a>),
    Dependency(DependencyRefMut<'a>),
    Differencing(DifferencingRefMut<'a>),
    TypeFeaturing(TypeFeaturingRefMut<'a>),
    Specialization(SpecializationRefMut<'a>),
    Intersecting(IntersectingRefMut<'a>),
    Conjugation(ConjugationRefMut<'a>),
    Membership(MembershipRefMut<'a>),
    Unioning(UnioningRefMut<'a>),
}
pub enum RelationshipRef<'a> {
    Itself(&'a RelationshipInner),
    Association(AssociationRef<'a>),
    Connector(ConnectorRef<'a>),
    Import(ImportRef<'a>),
    Disjoining(DisjoiningRef<'a>),
    FeatureInverting(FeatureInvertingRef<'a>),
    FeatureChaining(FeatureChainingRef<'a>),
    Annotation(AnnotationRef<'a>),
    Dependency(DependencyRef<'a>),
    Differencing(DifferencingRef<'a>),
    TypeFeaturing(TypeFeaturingRef<'a>),
    Specialization(SpecializationRef<'a>),
    Intersecting(IntersectingRef<'a>),
    Conjugation(ConjugationRef<'a>),
    Membership(MembershipRef<'a>),
    Unioning(UnioningRef<'a>),
}
impl Relationship {
    pub fn as_ref(&self) -> RelationshipRef {
        match self {
            Relationship::Itself(inner) => RelationshipRef::Itself(&inner),
            Relationship::Association(inner) => {
                RelationshipRef::Association(inner.as_ref())
            }
            Relationship::Connector(inner) => RelationshipRef::Connector(inner.as_ref()),
            Relationship::Import(inner) => RelationshipRef::Import(inner.as_ref()),
            Relationship::Disjoining(inner) => {
                RelationshipRef::Disjoining(inner.as_ref())
            }
            Relationship::FeatureInverting(inner) => {
                RelationshipRef::FeatureInverting(inner.as_ref())
            }
            Relationship::FeatureChaining(inner) => {
                RelationshipRef::FeatureChaining(inner.as_ref())
            }
            Relationship::Annotation(inner) => {
                RelationshipRef::Annotation(inner.as_ref())
            }
            Relationship::Dependency(inner) => {
                RelationshipRef::Dependency(inner.as_ref())
            }
            Relationship::Differencing(inner) => {
                RelationshipRef::Differencing(inner.as_ref())
            }
            Relationship::TypeFeaturing(inner) => {
                RelationshipRef::TypeFeaturing(inner.as_ref())
            }
            Relationship::Specialization(inner) => {
                RelationshipRef::Specialization(inner.as_ref())
            }
            Relationship::Intersecting(inner) => {
                RelationshipRef::Intersecting(inner.as_ref())
            }
            Relationship::Conjugation(inner) => {
                RelationshipRef::Conjugation(inner.as_ref())
            }
            Relationship::Membership(inner) => {
                RelationshipRef::Membership(inner.as_ref())
            }
            Relationship::Unioning(inner) => RelationshipRef::Unioning(inner.as_ref()),
        }
    }
    pub fn as_ref_mut(&mut self) -> RelationshipRefMut {
        match self {
            Relationship::Itself(inner) => RelationshipRefMut::Itself(inner),
            Relationship::Association(inner) => {
                RelationshipRefMut::Association(inner.as_ref_mut())
            }
            Relationship::Connector(inner) => {
                RelationshipRefMut::Connector(inner.as_ref_mut())
            }
            Relationship::Import(inner) => RelationshipRefMut::Import(inner.as_ref_mut()),
            Relationship::Disjoining(inner) => {
                RelationshipRefMut::Disjoining(inner.as_ref_mut())
            }
            Relationship::FeatureInverting(inner) => {
                RelationshipRefMut::FeatureInverting(inner.as_ref_mut())
            }
            Relationship::FeatureChaining(inner) => {
                RelationshipRefMut::FeatureChaining(inner.as_ref_mut())
            }
            Relationship::Annotation(inner) => {
                RelationshipRefMut::Annotation(inner.as_ref_mut())
            }
            Relationship::Dependency(inner) => {
                RelationshipRefMut::Dependency(inner.as_ref_mut())
            }
            Relationship::Differencing(inner) => {
                RelationshipRefMut::Differencing(inner.as_ref_mut())
            }
            Relationship::TypeFeaturing(inner) => {
                RelationshipRefMut::TypeFeaturing(inner.as_ref_mut())
            }
            Relationship::Specialization(inner) => {
                RelationshipRefMut::Specialization(inner.as_ref_mut())
            }
            Relationship::Intersecting(inner) => {
                RelationshipRefMut::Intersecting(inner.as_ref_mut())
            }
            Relationship::Conjugation(inner) => {
                RelationshipRefMut::Conjugation(inner.as_ref_mut())
            }
            Relationship::Membership(inner) => {
                RelationshipRefMut::Membership(inner.as_ref_mut())
            }
            Relationship::Unioning(inner) => {
                RelationshipRefMut::Unioning(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> RelationshipRefMut<'a> {
    pub fn as_ref(self) -> RelationshipRef<'a> {
        match self {
            RelationshipRefMut::Itself(inner) => RelationshipRef::Itself(inner),
            RelationshipRefMut::Association(inner) => {
                RelationshipRef::Association(inner.as_ref())
            }
            RelationshipRefMut::Connector(inner) => {
                RelationshipRef::Connector(inner.as_ref())
            }
            RelationshipRefMut::Import(inner) => RelationshipRef::Import(inner.as_ref()),
            RelationshipRefMut::Disjoining(inner) => {
                RelationshipRef::Disjoining(inner.as_ref())
            }
            RelationshipRefMut::FeatureInverting(inner) => {
                RelationshipRef::FeatureInverting(inner.as_ref())
            }
            RelationshipRefMut::FeatureChaining(inner) => {
                RelationshipRef::FeatureChaining(inner.as_ref())
            }
            RelationshipRefMut::Annotation(inner) => {
                RelationshipRef::Annotation(inner.as_ref())
            }
            RelationshipRefMut::Dependency(inner) => {
                RelationshipRef::Dependency(inner.as_ref())
            }
            RelationshipRefMut::Differencing(inner) => {
                RelationshipRef::Differencing(inner.as_ref())
            }
            RelationshipRefMut::TypeFeaturing(inner) => {
                RelationshipRef::TypeFeaturing(inner.as_ref())
            }
            RelationshipRefMut::Specialization(inner) => {
                RelationshipRef::Specialization(inner.as_ref())
            }
            RelationshipRefMut::Intersecting(inner) => {
                RelationshipRef::Intersecting(inner.as_ref())
            }
            RelationshipRefMut::Conjugation(inner) => {
                RelationshipRef::Conjugation(inner.as_ref())
            }
            RelationshipRefMut::Membership(inner) => {
                RelationshipRef::Membership(inner.as_ref())
            }
            RelationshipRefMut::Unioning(inner) => {
                RelationshipRef::Unioning(inner.as_ref())
            }
        }
    }
}
impl RelationshipStruct for Relationship {
    fn target(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.target(),
            Relationship::Association(x) => x.target(),
            Relationship::Connector(x) => x.target(),
            Relationship::Import(x) => x.target(),
            Relationship::Disjoining(x) => x.target(),
            Relationship::FeatureInverting(x) => x.target(),
            Relationship::FeatureChaining(x) => x.target(),
            Relationship::Annotation(x) => x.target(),
            Relationship::Dependency(x) => x.target(),
            Relationship::Differencing(x) => x.target(),
            Relationship::TypeFeaturing(x) => x.target(),
            Relationship::Specialization(x) => x.target(),
            Relationship::Intersecting(x) => x.target(),
            Relationship::Conjugation(x) => x.target(),
            Relationship::Membership(x) => x.target(),
            Relationship::Unioning(x) => x.target(),
        }
    }
    fn source(self) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.source(),
            Relationship::Association(x) => x.source(),
            Relationship::Connector(x) => x.source(),
            Relationship::Import(x) => x.source(),
            Relationship::Disjoining(x) => x.source(),
            Relationship::FeatureInverting(x) => x.source(),
            Relationship::FeatureChaining(x) => x.source(),
            Relationship::Annotation(x) => x.source(),
            Relationship::Dependency(x) => x.source(),
            Relationship::Differencing(x) => x.source(),
            Relationship::TypeFeaturing(x) => x.source(),
            Relationship::Specialization(x) => x.source(),
            Relationship::Intersecting(x) => x.source(),
            Relationship::Conjugation(x) => x.source(),
            Relationship::Membership(x) => x.source(),
            Relationship::Unioning(x) => x.source(),
        }
    }
    fn owning_related_element(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.owning_related_element(),
            Relationship::Association(x) => x.owning_related_element(),
            Relationship::Connector(x) => x.owning_related_element(),
            Relationship::Import(x) => x.owning_related_element(),
            Relationship::Disjoining(x) => x.owning_related_element(),
            Relationship::FeatureInverting(x) => x.owning_related_element(),
            Relationship::FeatureChaining(x) => x.owning_related_element(),
            Relationship::Annotation(x) => x.owning_related_element(),
            Relationship::Dependency(x) => x.owning_related_element(),
            Relationship::Differencing(x) => x.owning_related_element(),
            Relationship::TypeFeaturing(x) => x.owning_related_element(),
            Relationship::Specialization(x) => x.owning_related_element(),
            Relationship::Intersecting(x) => x.owning_related_element(),
            Relationship::Conjugation(x) => x.owning_related_element(),
            Relationship::Membership(x) => x.owning_related_element(),
            Relationship::Unioning(x) => x.owning_related_element(),
        }
    }
    fn owned_related_element(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.owned_related_element(),
            Relationship::Association(x) => x.owned_related_element(),
            Relationship::Connector(x) => x.owned_related_element(),
            Relationship::Import(x) => x.owned_related_element(),
            Relationship::Disjoining(x) => x.owned_related_element(),
            Relationship::FeatureInverting(x) => x.owned_related_element(),
            Relationship::FeatureChaining(x) => x.owned_related_element(),
            Relationship::Annotation(x) => x.owned_related_element(),
            Relationship::Dependency(x) => x.owned_related_element(),
            Relationship::Differencing(x) => x.owned_related_element(),
            Relationship::TypeFeaturing(x) => x.owned_related_element(),
            Relationship::Specialization(x) => x.owned_related_element(),
            Relationship::Intersecting(x) => x.owned_related_element(),
            Relationship::Conjugation(x) => x.owned_related_element(),
            Relationship::Membership(x) => x.owned_related_element(),
            Relationship::Unioning(x) => x.owned_related_element(),
        }
    }
    fn is_implied(self) -> bool {
        match self {
            Relationship::Itself(x) => x.is_implied(),
            Relationship::Association(x) => x.is_implied(),
            Relationship::Connector(x) => x.is_implied(),
            Relationship::Import(x) => x.is_implied(),
            Relationship::Disjoining(x) => x.is_implied(),
            Relationship::FeatureInverting(x) => x.is_implied(),
            Relationship::FeatureChaining(x) => x.is_implied(),
            Relationship::Annotation(x) => x.is_implied(),
            Relationship::Dependency(x) => x.is_implied(),
            Relationship::Differencing(x) => x.is_implied(),
            Relationship::TypeFeaturing(x) => x.is_implied(),
            Relationship::Specialization(x) => x.is_implied(),
            Relationship::Intersecting(x) => x.is_implied(),
            Relationship::Conjugation(x) => x.is_implied(),
            Relationship::Membership(x) => x.is_implied(),
            Relationship::Unioning(x) => x.is_implied(),
        }
    }
}
impl RelationshipStructRefMut for Relationship {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.target_ref_mut(),
            Relationship::Association(x) => x.target_ref_mut(),
            Relationship::Connector(x) => x.target_ref_mut(),
            Relationship::Import(x) => x.target_ref_mut(),
            Relationship::Disjoining(x) => x.target_ref_mut(),
            Relationship::FeatureInverting(x) => x.target_ref_mut(),
            Relationship::FeatureChaining(x) => x.target_ref_mut(),
            Relationship::Annotation(x) => x.target_ref_mut(),
            Relationship::Dependency(x) => x.target_ref_mut(),
            Relationship::Differencing(x) => x.target_ref_mut(),
            Relationship::TypeFeaturing(x) => x.target_ref_mut(),
            Relationship::Specialization(x) => x.target_ref_mut(),
            Relationship::Intersecting(x) => x.target_ref_mut(),
            Relationship::Conjugation(x) => x.target_ref_mut(),
            Relationship::Membership(x) => x.target_ref_mut(),
            Relationship::Unioning(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.source_ref_mut(),
            Relationship::Association(x) => x.source_ref_mut(),
            Relationship::Connector(x) => x.source_ref_mut(),
            Relationship::Import(x) => x.source_ref_mut(),
            Relationship::Disjoining(x) => x.source_ref_mut(),
            Relationship::FeatureInverting(x) => x.source_ref_mut(),
            Relationship::FeatureChaining(x) => x.source_ref_mut(),
            Relationship::Annotation(x) => x.source_ref_mut(),
            Relationship::Dependency(x) => x.source_ref_mut(),
            Relationship::Differencing(x) => x.source_ref_mut(),
            Relationship::TypeFeaturing(x) => x.source_ref_mut(),
            Relationship::Specialization(x) => x.source_ref_mut(),
            Relationship::Intersecting(x) => x.source_ref_mut(),
            Relationship::Conjugation(x) => x.source_ref_mut(),
            Relationship::Membership(x) => x.source_ref_mut(),
            Relationship::Unioning(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.owning_related_element_ref_mut(),
            Relationship::Association(x) => x.owning_related_element_ref_mut(),
            Relationship::Connector(x) => x.owning_related_element_ref_mut(),
            Relationship::Import(x) => x.owning_related_element_ref_mut(),
            Relationship::Disjoining(x) => x.owning_related_element_ref_mut(),
            Relationship::FeatureInverting(x) => x.owning_related_element_ref_mut(),
            Relationship::FeatureChaining(x) => x.owning_related_element_ref_mut(),
            Relationship::Annotation(x) => x.owning_related_element_ref_mut(),
            Relationship::Dependency(x) => x.owning_related_element_ref_mut(),
            Relationship::Differencing(x) => x.owning_related_element_ref_mut(),
            Relationship::TypeFeaturing(x) => x.owning_related_element_ref_mut(),
            Relationship::Specialization(x) => x.owning_related_element_ref_mut(),
            Relationship::Intersecting(x) => x.owning_related_element_ref_mut(),
            Relationship::Conjugation(x) => x.owning_related_element_ref_mut(),
            Relationship::Membership(x) => x.owning_related_element_ref_mut(),
            Relationship::Unioning(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.owned_related_element_ref_mut(),
            Relationship::Association(x) => x.owned_related_element_ref_mut(),
            Relationship::Connector(x) => x.owned_related_element_ref_mut(),
            Relationship::Import(x) => x.owned_related_element_ref_mut(),
            Relationship::Disjoining(x) => x.owned_related_element_ref_mut(),
            Relationship::FeatureInverting(x) => x.owned_related_element_ref_mut(),
            Relationship::FeatureChaining(x) => x.owned_related_element_ref_mut(),
            Relationship::Annotation(x) => x.owned_related_element_ref_mut(),
            Relationship::Dependency(x) => x.owned_related_element_ref_mut(),
            Relationship::Differencing(x) => x.owned_related_element_ref_mut(),
            Relationship::TypeFeaturing(x) => x.owned_related_element_ref_mut(),
            Relationship::Specialization(x) => x.owned_related_element_ref_mut(),
            Relationship::Intersecting(x) => x.owned_related_element_ref_mut(),
            Relationship::Conjugation(x) => x.owned_related_element_ref_mut(),
            Relationship::Membership(x) => x.owned_related_element_ref_mut(),
            Relationship::Unioning(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            Relationship::Itself(x) => x.is_implied_ref_mut(),
            Relationship::Association(x) => x.is_implied_ref_mut(),
            Relationship::Connector(x) => x.is_implied_ref_mut(),
            Relationship::Import(x) => x.is_implied_ref_mut(),
            Relationship::Disjoining(x) => x.is_implied_ref_mut(),
            Relationship::FeatureInverting(x) => x.is_implied_ref_mut(),
            Relationship::FeatureChaining(x) => x.is_implied_ref_mut(),
            Relationship::Annotation(x) => x.is_implied_ref_mut(),
            Relationship::Dependency(x) => x.is_implied_ref_mut(),
            Relationship::Differencing(x) => x.is_implied_ref_mut(),
            Relationship::TypeFeaturing(x) => x.is_implied_ref_mut(),
            Relationship::Specialization(x) => x.is_implied_ref_mut(),
            Relationship::Intersecting(x) => x.is_implied_ref_mut(),
            Relationship::Conjugation(x) => x.is_implied_ref_mut(),
            Relationship::Membership(x) => x.is_implied_ref_mut(),
            Relationship::Unioning(x) => x.is_implied_ref_mut(),
        }
    }
}
impl RelationshipStructRef for Relationship {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.target_ref(),
            Relationship::Association(x) => x.target_ref(),
            Relationship::Connector(x) => x.target_ref(),
            Relationship::Import(x) => x.target_ref(),
            Relationship::Disjoining(x) => x.target_ref(),
            Relationship::FeatureInverting(x) => x.target_ref(),
            Relationship::FeatureChaining(x) => x.target_ref(),
            Relationship::Annotation(x) => x.target_ref(),
            Relationship::Dependency(x) => x.target_ref(),
            Relationship::Differencing(x) => x.target_ref(),
            Relationship::TypeFeaturing(x) => x.target_ref(),
            Relationship::Specialization(x) => x.target_ref(),
            Relationship::Intersecting(x) => x.target_ref(),
            Relationship::Conjugation(x) => x.target_ref(),
            Relationship::Membership(x) => x.target_ref(),
            Relationship::Unioning(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.source_ref(),
            Relationship::Association(x) => x.source_ref(),
            Relationship::Connector(x) => x.source_ref(),
            Relationship::Import(x) => x.source_ref(),
            Relationship::Disjoining(x) => x.source_ref(),
            Relationship::FeatureInverting(x) => x.source_ref(),
            Relationship::FeatureChaining(x) => x.source_ref(),
            Relationship::Annotation(x) => x.source_ref(),
            Relationship::Dependency(x) => x.source_ref(),
            Relationship::Differencing(x) => x.source_ref(),
            Relationship::TypeFeaturing(x) => x.source_ref(),
            Relationship::Specialization(x) => x.source_ref(),
            Relationship::Intersecting(x) => x.source_ref(),
            Relationship::Conjugation(x) => x.source_ref(),
            Relationship::Membership(x) => x.source_ref(),
            Relationship::Unioning(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.owning_related_element_ref(),
            Relationship::Association(x) => x.owning_related_element_ref(),
            Relationship::Connector(x) => x.owning_related_element_ref(),
            Relationship::Import(x) => x.owning_related_element_ref(),
            Relationship::Disjoining(x) => x.owning_related_element_ref(),
            Relationship::FeatureInverting(x) => x.owning_related_element_ref(),
            Relationship::FeatureChaining(x) => x.owning_related_element_ref(),
            Relationship::Annotation(x) => x.owning_related_element_ref(),
            Relationship::Dependency(x) => x.owning_related_element_ref(),
            Relationship::Differencing(x) => x.owning_related_element_ref(),
            Relationship::TypeFeaturing(x) => x.owning_related_element_ref(),
            Relationship::Specialization(x) => x.owning_related_element_ref(),
            Relationship::Intersecting(x) => x.owning_related_element_ref(),
            Relationship::Conjugation(x) => x.owning_related_element_ref(),
            Relationship::Membership(x) => x.owning_related_element_ref(),
            Relationship::Unioning(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            Relationship::Itself(x) => x.owned_related_element_ref(),
            Relationship::Association(x) => x.owned_related_element_ref(),
            Relationship::Connector(x) => x.owned_related_element_ref(),
            Relationship::Import(x) => x.owned_related_element_ref(),
            Relationship::Disjoining(x) => x.owned_related_element_ref(),
            Relationship::FeatureInverting(x) => x.owned_related_element_ref(),
            Relationship::FeatureChaining(x) => x.owned_related_element_ref(),
            Relationship::Annotation(x) => x.owned_related_element_ref(),
            Relationship::Dependency(x) => x.owned_related_element_ref(),
            Relationship::Differencing(x) => x.owned_related_element_ref(),
            Relationship::TypeFeaturing(x) => x.owned_related_element_ref(),
            Relationship::Specialization(x) => x.owned_related_element_ref(),
            Relationship::Intersecting(x) => x.owned_related_element_ref(),
            Relationship::Conjugation(x) => x.owned_related_element_ref(),
            Relationship::Membership(x) => x.owned_related_element_ref(),
            Relationship::Unioning(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            Relationship::Itself(x) => x.is_implied_ref(),
            Relationship::Association(x) => x.is_implied_ref(),
            Relationship::Connector(x) => x.is_implied_ref(),
            Relationship::Import(x) => x.is_implied_ref(),
            Relationship::Disjoining(x) => x.is_implied_ref(),
            Relationship::FeatureInverting(x) => x.is_implied_ref(),
            Relationship::FeatureChaining(x) => x.is_implied_ref(),
            Relationship::Annotation(x) => x.is_implied_ref(),
            Relationship::Dependency(x) => x.is_implied_ref(),
            Relationship::Differencing(x) => x.is_implied_ref(),
            Relationship::TypeFeaturing(x) => x.is_implied_ref(),
            Relationship::Specialization(x) => x.is_implied_ref(),
            Relationship::Intersecting(x) => x.is_implied_ref(),
            Relationship::Conjugation(x) => x.is_implied_ref(),
            Relationship::Membership(x) => x.is_implied_ref(),
            Relationship::Unioning(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRefMut for RelationshipRefMut<'a> {
    fn target_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.target_ref_mut(),
            RelationshipRefMut::Association(x) => x.target_ref_mut(),
            RelationshipRefMut::Connector(x) => x.target_ref_mut(),
            RelationshipRefMut::Import(x) => x.target_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.target_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.target_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.target_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.target_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.target_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.target_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.target_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.target_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.target_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.target_ref_mut(),
            RelationshipRefMut::Membership(x) => x.target_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.target_ref_mut(),
        }
    }
    fn source_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.source_ref_mut(),
            RelationshipRefMut::Association(x) => x.source_ref_mut(),
            RelationshipRefMut::Connector(x) => x.source_ref_mut(),
            RelationshipRefMut::Import(x) => x.source_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.source_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.source_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.source_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.source_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.source_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.source_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.source_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.source_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.source_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.source_ref_mut(),
            RelationshipRefMut::Membership(x) => x.source_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.source_ref_mut(),
        }
    }
    fn owning_related_element_ref_mut(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Association(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Connector(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Import(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Membership(x) => x.owning_related_element_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.owning_related_element_ref_mut(),
        }
    }
    fn owned_related_element_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Association(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Connector(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Import(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Membership(x) => x.owned_related_element_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.owned_related_element_ref_mut(),
        }
    }
    fn is_implied_ref_mut(&mut self) -> &mut bool {
        match self {
            RelationshipRefMut::Itself(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Association(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Connector(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Import(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Membership(x) => x.is_implied_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.is_implied_ref_mut(),
        }
    }
}
impl<'a> RelationshipStructRef for RelationshipRefMut<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.target_ref(),
            RelationshipRefMut::Association(x) => x.target_ref(),
            RelationshipRefMut::Connector(x) => x.target_ref(),
            RelationshipRefMut::Import(x) => x.target_ref(),
            RelationshipRefMut::Disjoining(x) => x.target_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.target_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.target_ref(),
            RelationshipRefMut::Annotation(x) => x.target_ref(),
            RelationshipRefMut::Dependency(x) => x.target_ref(),
            RelationshipRefMut::Differencing(x) => x.target_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.target_ref(),
            RelationshipRefMut::Specialization(x) => x.target_ref(),
            RelationshipRefMut::Intersecting(x) => x.target_ref(),
            RelationshipRefMut::Conjugation(x) => x.target_ref(),
            RelationshipRefMut::Membership(x) => x.target_ref(),
            RelationshipRefMut::Unioning(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.source_ref(),
            RelationshipRefMut::Association(x) => x.source_ref(),
            RelationshipRefMut::Connector(x) => x.source_ref(),
            RelationshipRefMut::Import(x) => x.source_ref(),
            RelationshipRefMut::Disjoining(x) => x.source_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.source_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.source_ref(),
            RelationshipRefMut::Annotation(x) => x.source_ref(),
            RelationshipRefMut::Dependency(x) => x.source_ref(),
            RelationshipRefMut::Differencing(x) => x.source_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.source_ref(),
            RelationshipRefMut::Specialization(x) => x.source_ref(),
            RelationshipRefMut::Intersecting(x) => x.source_ref(),
            RelationshipRefMut::Conjugation(x) => x.source_ref(),
            RelationshipRefMut::Membership(x) => x.source_ref(),
            RelationshipRefMut::Unioning(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Association(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Connector(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Import(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Disjoining(x) => x.owning_related_element_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.owning_related_element_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Annotation(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Dependency(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Differencing(x) => x.owning_related_element_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Specialization(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Intersecting(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Conjugation(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Membership(x) => x.owning_related_element_ref(),
            RelationshipRefMut::Unioning(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Association(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Connector(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Import(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Disjoining(x) => x.owned_related_element_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.owned_related_element_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Annotation(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Dependency(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Differencing(x) => x.owned_related_element_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Specialization(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Intersecting(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Conjugation(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Membership(x) => x.owned_related_element_ref(),
            RelationshipRefMut::Unioning(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            RelationshipRefMut::Itself(x) => x.is_implied_ref(),
            RelationshipRefMut::Association(x) => x.is_implied_ref(),
            RelationshipRefMut::Connector(x) => x.is_implied_ref(),
            RelationshipRefMut::Import(x) => x.is_implied_ref(),
            RelationshipRefMut::Disjoining(x) => x.is_implied_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.is_implied_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.is_implied_ref(),
            RelationshipRefMut::Annotation(x) => x.is_implied_ref(),
            RelationshipRefMut::Dependency(x) => x.is_implied_ref(),
            RelationshipRefMut::Differencing(x) => x.is_implied_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.is_implied_ref(),
            RelationshipRefMut::Specialization(x) => x.is_implied_ref(),
            RelationshipRefMut::Intersecting(x) => x.is_implied_ref(),
            RelationshipRefMut::Conjugation(x) => x.is_implied_ref(),
            RelationshipRefMut::Membership(x) => x.is_implied_ref(),
            RelationshipRefMut::Unioning(x) => x.is_implied_ref(),
        }
    }
}
impl<'a> RelationshipStructRef for RelationshipRef<'a> {
    fn target_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRef::Itself(x) => x.target_ref(),
            RelationshipRef::Association(x) => x.target_ref(),
            RelationshipRef::Connector(x) => x.target_ref(),
            RelationshipRef::Import(x) => x.target_ref(),
            RelationshipRef::Disjoining(x) => x.target_ref(),
            RelationshipRef::FeatureInverting(x) => x.target_ref(),
            RelationshipRef::FeatureChaining(x) => x.target_ref(),
            RelationshipRef::Annotation(x) => x.target_ref(),
            RelationshipRef::Dependency(x) => x.target_ref(),
            RelationshipRef::Differencing(x) => x.target_ref(),
            RelationshipRef::TypeFeaturing(x) => x.target_ref(),
            RelationshipRef::Specialization(x) => x.target_ref(),
            RelationshipRef::Intersecting(x) => x.target_ref(),
            RelationshipRef::Conjugation(x) => x.target_ref(),
            RelationshipRef::Membership(x) => x.target_ref(),
            RelationshipRef::Unioning(x) => x.target_ref(),
        }
    }
    fn source_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRef::Itself(x) => x.source_ref(),
            RelationshipRef::Association(x) => x.source_ref(),
            RelationshipRef::Connector(x) => x.source_ref(),
            RelationshipRef::Import(x) => x.source_ref(),
            RelationshipRef::Disjoining(x) => x.source_ref(),
            RelationshipRef::FeatureInverting(x) => x.source_ref(),
            RelationshipRef::FeatureChaining(x) => x.source_ref(),
            RelationshipRef::Annotation(x) => x.source_ref(),
            RelationshipRef::Dependency(x) => x.source_ref(),
            RelationshipRef::Differencing(x) => x.source_ref(),
            RelationshipRef::TypeFeaturing(x) => x.source_ref(),
            RelationshipRef::Specialization(x) => x.source_ref(),
            RelationshipRef::Intersecting(x) => x.source_ref(),
            RelationshipRef::Conjugation(x) => x.source_ref(),
            RelationshipRef::Membership(x) => x.source_ref(),
            RelationshipRef::Unioning(x) => x.source_ref(),
        }
    }
    fn owning_related_element_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRef::Itself(x) => x.owning_related_element_ref(),
            RelationshipRef::Association(x) => x.owning_related_element_ref(),
            RelationshipRef::Connector(x) => x.owning_related_element_ref(),
            RelationshipRef::Import(x) => x.owning_related_element_ref(),
            RelationshipRef::Disjoining(x) => x.owning_related_element_ref(),
            RelationshipRef::FeatureInverting(x) => x.owning_related_element_ref(),
            RelationshipRef::FeatureChaining(x) => x.owning_related_element_ref(),
            RelationshipRef::Annotation(x) => x.owning_related_element_ref(),
            RelationshipRef::Dependency(x) => x.owning_related_element_ref(),
            RelationshipRef::Differencing(x) => x.owning_related_element_ref(),
            RelationshipRef::TypeFeaturing(x) => x.owning_related_element_ref(),
            RelationshipRef::Specialization(x) => x.owning_related_element_ref(),
            RelationshipRef::Intersecting(x) => x.owning_related_element_ref(),
            RelationshipRef::Conjugation(x) => x.owning_related_element_ref(),
            RelationshipRef::Membership(x) => x.owning_related_element_ref(),
            RelationshipRef::Unioning(x) => x.owning_related_element_ref(),
        }
    }
    fn owned_related_element_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        match self {
            RelationshipRef::Itself(x) => x.owned_related_element_ref(),
            RelationshipRef::Association(x) => x.owned_related_element_ref(),
            RelationshipRef::Connector(x) => x.owned_related_element_ref(),
            RelationshipRef::Import(x) => x.owned_related_element_ref(),
            RelationshipRef::Disjoining(x) => x.owned_related_element_ref(),
            RelationshipRef::FeatureInverting(x) => x.owned_related_element_ref(),
            RelationshipRef::FeatureChaining(x) => x.owned_related_element_ref(),
            RelationshipRef::Annotation(x) => x.owned_related_element_ref(),
            RelationshipRef::Dependency(x) => x.owned_related_element_ref(),
            RelationshipRef::Differencing(x) => x.owned_related_element_ref(),
            RelationshipRef::TypeFeaturing(x) => x.owned_related_element_ref(),
            RelationshipRef::Specialization(x) => x.owned_related_element_ref(),
            RelationshipRef::Intersecting(x) => x.owned_related_element_ref(),
            RelationshipRef::Conjugation(x) => x.owned_related_element_ref(),
            RelationshipRef::Membership(x) => x.owned_related_element_ref(),
            RelationshipRef::Unioning(x) => x.owned_related_element_ref(),
        }
    }
    fn is_implied_ref(&self) -> &bool {
        match self {
            RelationshipRef::Itself(x) => x.is_implied_ref(),
            RelationshipRef::Association(x) => x.is_implied_ref(),
            RelationshipRef::Connector(x) => x.is_implied_ref(),
            RelationshipRef::Import(x) => x.is_implied_ref(),
            RelationshipRef::Disjoining(x) => x.is_implied_ref(),
            RelationshipRef::FeatureInverting(x) => x.is_implied_ref(),
            RelationshipRef::FeatureChaining(x) => x.is_implied_ref(),
            RelationshipRef::Annotation(x) => x.is_implied_ref(),
            RelationshipRef::Dependency(x) => x.is_implied_ref(),
            RelationshipRef::Differencing(x) => x.is_implied_ref(),
            RelationshipRef::TypeFeaturing(x) => x.is_implied_ref(),
            RelationshipRef::Specialization(x) => x.is_implied_ref(),
            RelationshipRef::Intersecting(x) => x.is_implied_ref(),
            RelationshipRef::Conjugation(x) => x.is_implied_ref(),
            RelationshipRef::Membership(x) => x.is_implied_ref(),
            RelationshipRef::Unioning(x) => x.is_implied_ref(),
        }
    }
}
impl ElementStruct for Relationship {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Relationship::Itself(x) => x.owned_relationship(),
            Relationship::Association(x) => x.owned_relationship(),
            Relationship::Connector(x) => x.owned_relationship(),
            Relationship::Import(x) => x.owned_relationship(),
            Relationship::Disjoining(x) => x.owned_relationship(),
            Relationship::FeatureInverting(x) => x.owned_relationship(),
            Relationship::FeatureChaining(x) => x.owned_relationship(),
            Relationship::Annotation(x) => x.owned_relationship(),
            Relationship::Dependency(x) => x.owned_relationship(),
            Relationship::Differencing(x) => x.owned_relationship(),
            Relationship::TypeFeaturing(x) => x.owned_relationship(),
            Relationship::Specialization(x) => x.owned_relationship(),
            Relationship::Intersecting(x) => x.owned_relationship(),
            Relationship::Conjugation(x) => x.owned_relationship(),
            Relationship::Membership(x) => x.owned_relationship(),
            Relationship::Unioning(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Relationship::Itself(x) => x.owning_relationship(),
            Relationship::Association(x) => x.owning_relationship(),
            Relationship::Connector(x) => x.owning_relationship(),
            Relationship::Import(x) => x.owning_relationship(),
            Relationship::Disjoining(x) => x.owning_relationship(),
            Relationship::FeatureInverting(x) => x.owning_relationship(),
            Relationship::FeatureChaining(x) => x.owning_relationship(),
            Relationship::Annotation(x) => x.owning_relationship(),
            Relationship::Dependency(x) => x.owning_relationship(),
            Relationship::Differencing(x) => x.owning_relationship(),
            Relationship::TypeFeaturing(x) => x.owning_relationship(),
            Relationship::Specialization(x) => x.owning_relationship(),
            Relationship::Intersecting(x) => x.owning_relationship(),
            Relationship::Conjugation(x) => x.owning_relationship(),
            Relationship::Membership(x) => x.owning_relationship(),
            Relationship::Unioning(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Relationship::Itself(x) => x.element_id(),
            Relationship::Association(x) => x.element_id(),
            Relationship::Connector(x) => x.element_id(),
            Relationship::Import(x) => x.element_id(),
            Relationship::Disjoining(x) => x.element_id(),
            Relationship::FeatureInverting(x) => x.element_id(),
            Relationship::FeatureChaining(x) => x.element_id(),
            Relationship::Annotation(x) => x.element_id(),
            Relationship::Dependency(x) => x.element_id(),
            Relationship::Differencing(x) => x.element_id(),
            Relationship::TypeFeaturing(x) => x.element_id(),
            Relationship::Specialization(x) => x.element_id(),
            Relationship::Intersecting(x) => x.element_id(),
            Relationship::Conjugation(x) => x.element_id(),
            Relationship::Membership(x) => x.element_id(),
            Relationship::Unioning(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Relationship::Itself(x) => x.alias_ids(),
            Relationship::Association(x) => x.alias_ids(),
            Relationship::Connector(x) => x.alias_ids(),
            Relationship::Import(x) => x.alias_ids(),
            Relationship::Disjoining(x) => x.alias_ids(),
            Relationship::FeatureInverting(x) => x.alias_ids(),
            Relationship::FeatureChaining(x) => x.alias_ids(),
            Relationship::Annotation(x) => x.alias_ids(),
            Relationship::Dependency(x) => x.alias_ids(),
            Relationship::Differencing(x) => x.alias_ids(),
            Relationship::TypeFeaturing(x) => x.alias_ids(),
            Relationship::Specialization(x) => x.alias_ids(),
            Relationship::Intersecting(x) => x.alias_ids(),
            Relationship::Conjugation(x) => x.alias_ids(),
            Relationship::Membership(x) => x.alias_ids(),
            Relationship::Unioning(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Relationship::Itself(x) => x.declared_short_name(),
            Relationship::Association(x) => x.declared_short_name(),
            Relationship::Connector(x) => x.declared_short_name(),
            Relationship::Import(x) => x.declared_short_name(),
            Relationship::Disjoining(x) => x.declared_short_name(),
            Relationship::FeatureInverting(x) => x.declared_short_name(),
            Relationship::FeatureChaining(x) => x.declared_short_name(),
            Relationship::Annotation(x) => x.declared_short_name(),
            Relationship::Dependency(x) => x.declared_short_name(),
            Relationship::Differencing(x) => x.declared_short_name(),
            Relationship::TypeFeaturing(x) => x.declared_short_name(),
            Relationship::Specialization(x) => x.declared_short_name(),
            Relationship::Intersecting(x) => x.declared_short_name(),
            Relationship::Conjugation(x) => x.declared_short_name(),
            Relationship::Membership(x) => x.declared_short_name(),
            Relationship::Unioning(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Relationship::Itself(x) => x.declared_name(),
            Relationship::Association(x) => x.declared_name(),
            Relationship::Connector(x) => x.declared_name(),
            Relationship::Import(x) => x.declared_name(),
            Relationship::Disjoining(x) => x.declared_name(),
            Relationship::FeatureInverting(x) => x.declared_name(),
            Relationship::FeatureChaining(x) => x.declared_name(),
            Relationship::Annotation(x) => x.declared_name(),
            Relationship::Dependency(x) => x.declared_name(),
            Relationship::Differencing(x) => x.declared_name(),
            Relationship::TypeFeaturing(x) => x.declared_name(),
            Relationship::Specialization(x) => x.declared_name(),
            Relationship::Intersecting(x) => x.declared_name(),
            Relationship::Conjugation(x) => x.declared_name(),
            Relationship::Membership(x) => x.declared_name(),
            Relationship::Unioning(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Relationship::Itself(x) => x.is_implied_included(),
            Relationship::Association(x) => x.is_implied_included(),
            Relationship::Connector(x) => x.is_implied_included(),
            Relationship::Import(x) => x.is_implied_included(),
            Relationship::Disjoining(x) => x.is_implied_included(),
            Relationship::FeatureInverting(x) => x.is_implied_included(),
            Relationship::FeatureChaining(x) => x.is_implied_included(),
            Relationship::Annotation(x) => x.is_implied_included(),
            Relationship::Dependency(x) => x.is_implied_included(),
            Relationship::Differencing(x) => x.is_implied_included(),
            Relationship::TypeFeaturing(x) => x.is_implied_included(),
            Relationship::Specialization(x) => x.is_implied_included(),
            Relationship::Intersecting(x) => x.is_implied_included(),
            Relationship::Conjugation(x) => x.is_implied_included(),
            Relationship::Membership(x) => x.is_implied_included(),
            Relationship::Unioning(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Relationship {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Relationship::Itself(x) => x.owned_relationship_ref_mut(),
            Relationship::Association(x) => x.owned_relationship_ref_mut(),
            Relationship::Connector(x) => x.owned_relationship_ref_mut(),
            Relationship::Import(x) => x.owned_relationship_ref_mut(),
            Relationship::Disjoining(x) => x.owned_relationship_ref_mut(),
            Relationship::FeatureInverting(x) => x.owned_relationship_ref_mut(),
            Relationship::FeatureChaining(x) => x.owned_relationship_ref_mut(),
            Relationship::Annotation(x) => x.owned_relationship_ref_mut(),
            Relationship::Dependency(x) => x.owned_relationship_ref_mut(),
            Relationship::Differencing(x) => x.owned_relationship_ref_mut(),
            Relationship::TypeFeaturing(x) => x.owned_relationship_ref_mut(),
            Relationship::Specialization(x) => x.owned_relationship_ref_mut(),
            Relationship::Intersecting(x) => x.owned_relationship_ref_mut(),
            Relationship::Conjugation(x) => x.owned_relationship_ref_mut(),
            Relationship::Membership(x) => x.owned_relationship_ref_mut(),
            Relationship::Unioning(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Relationship::Itself(x) => x.owning_relationship_ref_mut(),
            Relationship::Association(x) => x.owning_relationship_ref_mut(),
            Relationship::Connector(x) => x.owning_relationship_ref_mut(),
            Relationship::Import(x) => x.owning_relationship_ref_mut(),
            Relationship::Disjoining(x) => x.owning_relationship_ref_mut(),
            Relationship::FeatureInverting(x) => x.owning_relationship_ref_mut(),
            Relationship::FeatureChaining(x) => x.owning_relationship_ref_mut(),
            Relationship::Annotation(x) => x.owning_relationship_ref_mut(),
            Relationship::Dependency(x) => x.owning_relationship_ref_mut(),
            Relationship::Differencing(x) => x.owning_relationship_ref_mut(),
            Relationship::TypeFeaturing(x) => x.owning_relationship_ref_mut(),
            Relationship::Specialization(x) => x.owning_relationship_ref_mut(),
            Relationship::Intersecting(x) => x.owning_relationship_ref_mut(),
            Relationship::Conjugation(x) => x.owning_relationship_ref_mut(),
            Relationship::Membership(x) => x.owning_relationship_ref_mut(),
            Relationship::Unioning(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Relationship::Itself(x) => x.element_id_ref_mut(),
            Relationship::Association(x) => x.element_id_ref_mut(),
            Relationship::Connector(x) => x.element_id_ref_mut(),
            Relationship::Import(x) => x.element_id_ref_mut(),
            Relationship::Disjoining(x) => x.element_id_ref_mut(),
            Relationship::FeatureInverting(x) => x.element_id_ref_mut(),
            Relationship::FeatureChaining(x) => x.element_id_ref_mut(),
            Relationship::Annotation(x) => x.element_id_ref_mut(),
            Relationship::Dependency(x) => x.element_id_ref_mut(),
            Relationship::Differencing(x) => x.element_id_ref_mut(),
            Relationship::TypeFeaturing(x) => x.element_id_ref_mut(),
            Relationship::Specialization(x) => x.element_id_ref_mut(),
            Relationship::Intersecting(x) => x.element_id_ref_mut(),
            Relationship::Conjugation(x) => x.element_id_ref_mut(),
            Relationship::Membership(x) => x.element_id_ref_mut(),
            Relationship::Unioning(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Relationship::Itself(x) => x.alias_ids_ref_mut(),
            Relationship::Association(x) => x.alias_ids_ref_mut(),
            Relationship::Connector(x) => x.alias_ids_ref_mut(),
            Relationship::Import(x) => x.alias_ids_ref_mut(),
            Relationship::Disjoining(x) => x.alias_ids_ref_mut(),
            Relationship::FeatureInverting(x) => x.alias_ids_ref_mut(),
            Relationship::FeatureChaining(x) => x.alias_ids_ref_mut(),
            Relationship::Annotation(x) => x.alias_ids_ref_mut(),
            Relationship::Dependency(x) => x.alias_ids_ref_mut(),
            Relationship::Differencing(x) => x.alias_ids_ref_mut(),
            Relationship::TypeFeaturing(x) => x.alias_ids_ref_mut(),
            Relationship::Specialization(x) => x.alias_ids_ref_mut(),
            Relationship::Intersecting(x) => x.alias_ids_ref_mut(),
            Relationship::Conjugation(x) => x.alias_ids_ref_mut(),
            Relationship::Membership(x) => x.alias_ids_ref_mut(),
            Relationship::Unioning(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Relationship::Itself(x) => x.declared_short_name_ref_mut(),
            Relationship::Association(x) => x.declared_short_name_ref_mut(),
            Relationship::Connector(x) => x.declared_short_name_ref_mut(),
            Relationship::Import(x) => x.declared_short_name_ref_mut(),
            Relationship::Disjoining(x) => x.declared_short_name_ref_mut(),
            Relationship::FeatureInverting(x) => x.declared_short_name_ref_mut(),
            Relationship::FeatureChaining(x) => x.declared_short_name_ref_mut(),
            Relationship::Annotation(x) => x.declared_short_name_ref_mut(),
            Relationship::Dependency(x) => x.declared_short_name_ref_mut(),
            Relationship::Differencing(x) => x.declared_short_name_ref_mut(),
            Relationship::TypeFeaturing(x) => x.declared_short_name_ref_mut(),
            Relationship::Specialization(x) => x.declared_short_name_ref_mut(),
            Relationship::Intersecting(x) => x.declared_short_name_ref_mut(),
            Relationship::Conjugation(x) => x.declared_short_name_ref_mut(),
            Relationship::Membership(x) => x.declared_short_name_ref_mut(),
            Relationship::Unioning(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Relationship::Itself(x) => x.declared_name_ref_mut(),
            Relationship::Association(x) => x.declared_name_ref_mut(),
            Relationship::Connector(x) => x.declared_name_ref_mut(),
            Relationship::Import(x) => x.declared_name_ref_mut(),
            Relationship::Disjoining(x) => x.declared_name_ref_mut(),
            Relationship::FeatureInverting(x) => x.declared_name_ref_mut(),
            Relationship::FeatureChaining(x) => x.declared_name_ref_mut(),
            Relationship::Annotation(x) => x.declared_name_ref_mut(),
            Relationship::Dependency(x) => x.declared_name_ref_mut(),
            Relationship::Differencing(x) => x.declared_name_ref_mut(),
            Relationship::TypeFeaturing(x) => x.declared_name_ref_mut(),
            Relationship::Specialization(x) => x.declared_name_ref_mut(),
            Relationship::Intersecting(x) => x.declared_name_ref_mut(),
            Relationship::Conjugation(x) => x.declared_name_ref_mut(),
            Relationship::Membership(x) => x.declared_name_ref_mut(),
            Relationship::Unioning(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Relationship::Itself(x) => x.is_implied_included_ref_mut(),
            Relationship::Association(x) => x.is_implied_included_ref_mut(),
            Relationship::Connector(x) => x.is_implied_included_ref_mut(),
            Relationship::Import(x) => x.is_implied_included_ref_mut(),
            Relationship::Disjoining(x) => x.is_implied_included_ref_mut(),
            Relationship::FeatureInverting(x) => x.is_implied_included_ref_mut(),
            Relationship::FeatureChaining(x) => x.is_implied_included_ref_mut(),
            Relationship::Annotation(x) => x.is_implied_included_ref_mut(),
            Relationship::Dependency(x) => x.is_implied_included_ref_mut(),
            Relationship::Differencing(x) => x.is_implied_included_ref_mut(),
            Relationship::TypeFeaturing(x) => x.is_implied_included_ref_mut(),
            Relationship::Specialization(x) => x.is_implied_included_ref_mut(),
            Relationship::Intersecting(x) => x.is_implied_included_ref_mut(),
            Relationship::Conjugation(x) => x.is_implied_included_ref_mut(),
            Relationship::Membership(x) => x.is_implied_included_ref_mut(),
            Relationship::Unioning(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Relationship {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Relationship::Itself(x) => x.owned_relationship_ref(),
            Relationship::Association(x) => x.owned_relationship_ref(),
            Relationship::Connector(x) => x.owned_relationship_ref(),
            Relationship::Import(x) => x.owned_relationship_ref(),
            Relationship::Disjoining(x) => x.owned_relationship_ref(),
            Relationship::FeatureInverting(x) => x.owned_relationship_ref(),
            Relationship::FeatureChaining(x) => x.owned_relationship_ref(),
            Relationship::Annotation(x) => x.owned_relationship_ref(),
            Relationship::Dependency(x) => x.owned_relationship_ref(),
            Relationship::Differencing(x) => x.owned_relationship_ref(),
            Relationship::TypeFeaturing(x) => x.owned_relationship_ref(),
            Relationship::Specialization(x) => x.owned_relationship_ref(),
            Relationship::Intersecting(x) => x.owned_relationship_ref(),
            Relationship::Conjugation(x) => x.owned_relationship_ref(),
            Relationship::Membership(x) => x.owned_relationship_ref(),
            Relationship::Unioning(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Relationship::Itself(x) => x.owning_relationship_ref(),
            Relationship::Association(x) => x.owning_relationship_ref(),
            Relationship::Connector(x) => x.owning_relationship_ref(),
            Relationship::Import(x) => x.owning_relationship_ref(),
            Relationship::Disjoining(x) => x.owning_relationship_ref(),
            Relationship::FeatureInverting(x) => x.owning_relationship_ref(),
            Relationship::FeatureChaining(x) => x.owning_relationship_ref(),
            Relationship::Annotation(x) => x.owning_relationship_ref(),
            Relationship::Dependency(x) => x.owning_relationship_ref(),
            Relationship::Differencing(x) => x.owning_relationship_ref(),
            Relationship::TypeFeaturing(x) => x.owning_relationship_ref(),
            Relationship::Specialization(x) => x.owning_relationship_ref(),
            Relationship::Intersecting(x) => x.owning_relationship_ref(),
            Relationship::Conjugation(x) => x.owning_relationship_ref(),
            Relationship::Membership(x) => x.owning_relationship_ref(),
            Relationship::Unioning(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Relationship::Itself(x) => x.element_id_ref(),
            Relationship::Association(x) => x.element_id_ref(),
            Relationship::Connector(x) => x.element_id_ref(),
            Relationship::Import(x) => x.element_id_ref(),
            Relationship::Disjoining(x) => x.element_id_ref(),
            Relationship::FeatureInverting(x) => x.element_id_ref(),
            Relationship::FeatureChaining(x) => x.element_id_ref(),
            Relationship::Annotation(x) => x.element_id_ref(),
            Relationship::Dependency(x) => x.element_id_ref(),
            Relationship::Differencing(x) => x.element_id_ref(),
            Relationship::TypeFeaturing(x) => x.element_id_ref(),
            Relationship::Specialization(x) => x.element_id_ref(),
            Relationship::Intersecting(x) => x.element_id_ref(),
            Relationship::Conjugation(x) => x.element_id_ref(),
            Relationship::Membership(x) => x.element_id_ref(),
            Relationship::Unioning(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Relationship::Itself(x) => x.alias_ids_ref(),
            Relationship::Association(x) => x.alias_ids_ref(),
            Relationship::Connector(x) => x.alias_ids_ref(),
            Relationship::Import(x) => x.alias_ids_ref(),
            Relationship::Disjoining(x) => x.alias_ids_ref(),
            Relationship::FeatureInverting(x) => x.alias_ids_ref(),
            Relationship::FeatureChaining(x) => x.alias_ids_ref(),
            Relationship::Annotation(x) => x.alias_ids_ref(),
            Relationship::Dependency(x) => x.alias_ids_ref(),
            Relationship::Differencing(x) => x.alias_ids_ref(),
            Relationship::TypeFeaturing(x) => x.alias_ids_ref(),
            Relationship::Specialization(x) => x.alias_ids_ref(),
            Relationship::Intersecting(x) => x.alias_ids_ref(),
            Relationship::Conjugation(x) => x.alias_ids_ref(),
            Relationship::Membership(x) => x.alias_ids_ref(),
            Relationship::Unioning(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Relationship::Itself(x) => x.declared_short_name_ref(),
            Relationship::Association(x) => x.declared_short_name_ref(),
            Relationship::Connector(x) => x.declared_short_name_ref(),
            Relationship::Import(x) => x.declared_short_name_ref(),
            Relationship::Disjoining(x) => x.declared_short_name_ref(),
            Relationship::FeatureInverting(x) => x.declared_short_name_ref(),
            Relationship::FeatureChaining(x) => x.declared_short_name_ref(),
            Relationship::Annotation(x) => x.declared_short_name_ref(),
            Relationship::Dependency(x) => x.declared_short_name_ref(),
            Relationship::Differencing(x) => x.declared_short_name_ref(),
            Relationship::TypeFeaturing(x) => x.declared_short_name_ref(),
            Relationship::Specialization(x) => x.declared_short_name_ref(),
            Relationship::Intersecting(x) => x.declared_short_name_ref(),
            Relationship::Conjugation(x) => x.declared_short_name_ref(),
            Relationship::Membership(x) => x.declared_short_name_ref(),
            Relationship::Unioning(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Relationship::Itself(x) => x.declared_name_ref(),
            Relationship::Association(x) => x.declared_name_ref(),
            Relationship::Connector(x) => x.declared_name_ref(),
            Relationship::Import(x) => x.declared_name_ref(),
            Relationship::Disjoining(x) => x.declared_name_ref(),
            Relationship::FeatureInverting(x) => x.declared_name_ref(),
            Relationship::FeatureChaining(x) => x.declared_name_ref(),
            Relationship::Annotation(x) => x.declared_name_ref(),
            Relationship::Dependency(x) => x.declared_name_ref(),
            Relationship::Differencing(x) => x.declared_name_ref(),
            Relationship::TypeFeaturing(x) => x.declared_name_ref(),
            Relationship::Specialization(x) => x.declared_name_ref(),
            Relationship::Intersecting(x) => x.declared_name_ref(),
            Relationship::Conjugation(x) => x.declared_name_ref(),
            Relationship::Membership(x) => x.declared_name_ref(),
            Relationship::Unioning(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Relationship::Itself(x) => x.is_implied_included_ref(),
            Relationship::Association(x) => x.is_implied_included_ref(),
            Relationship::Connector(x) => x.is_implied_included_ref(),
            Relationship::Import(x) => x.is_implied_included_ref(),
            Relationship::Disjoining(x) => x.is_implied_included_ref(),
            Relationship::FeatureInverting(x) => x.is_implied_included_ref(),
            Relationship::FeatureChaining(x) => x.is_implied_included_ref(),
            Relationship::Annotation(x) => x.is_implied_included_ref(),
            Relationship::Dependency(x) => x.is_implied_included_ref(),
            Relationship::Differencing(x) => x.is_implied_included_ref(),
            Relationship::TypeFeaturing(x) => x.is_implied_included_ref(),
            Relationship::Specialization(x) => x.is_implied_included_ref(),
            Relationship::Intersecting(x) => x.is_implied_included_ref(),
            Relationship::Conjugation(x) => x.is_implied_included_ref(),
            Relationship::Membership(x) => x.is_implied_included_ref(),
            Relationship::Unioning(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for RelationshipRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Association(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Connector(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Import(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Membership(x) => x.owned_relationship_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            RelationshipRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Association(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Connector(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Import(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Membership(x) => x.owning_relationship_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            RelationshipRefMut::Itself(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Association(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Connector(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Import(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.element_id_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.element_id_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.element_id_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Membership(x) => x.element_id_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            RelationshipRefMut::Itself(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Association(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Connector(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Import(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Membership(x) => x.alias_ids_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            RelationshipRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Association(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Connector(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Import(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Membership(x) => x.declared_short_name_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            RelationshipRefMut::Itself(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Association(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Connector(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Import(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Membership(x) => x.declared_name_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            RelationshipRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Association(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Connector(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Import(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Disjoining(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::FeatureInverting(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::FeatureChaining(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Annotation(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Dependency(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Differencing(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::TypeFeaturing(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Specialization(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Intersecting(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Conjugation(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Membership(x) => x.is_implied_included_ref_mut(),
            RelationshipRefMut::Unioning(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for RelationshipRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Association(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Connector(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Import(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Disjoining(x) => x.owned_relationship_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.owned_relationship_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Annotation(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Dependency(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Differencing(x) => x.owned_relationship_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Specialization(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Intersecting(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Conjugation(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Membership(x) => x.owned_relationship_ref(),
            RelationshipRefMut::Unioning(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RelationshipRefMut::Itself(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Association(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Connector(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Import(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Disjoining(x) => x.owning_relationship_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.owning_relationship_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Annotation(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Dependency(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Differencing(x) => x.owning_relationship_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Specialization(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Intersecting(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Conjugation(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Membership(x) => x.owning_relationship_ref(),
            RelationshipRefMut::Unioning(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            RelationshipRefMut::Itself(x) => x.element_id_ref(),
            RelationshipRefMut::Association(x) => x.element_id_ref(),
            RelationshipRefMut::Connector(x) => x.element_id_ref(),
            RelationshipRefMut::Import(x) => x.element_id_ref(),
            RelationshipRefMut::Disjoining(x) => x.element_id_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.element_id_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.element_id_ref(),
            RelationshipRefMut::Annotation(x) => x.element_id_ref(),
            RelationshipRefMut::Dependency(x) => x.element_id_ref(),
            RelationshipRefMut::Differencing(x) => x.element_id_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.element_id_ref(),
            RelationshipRefMut::Specialization(x) => x.element_id_ref(),
            RelationshipRefMut::Intersecting(x) => x.element_id_ref(),
            RelationshipRefMut::Conjugation(x) => x.element_id_ref(),
            RelationshipRefMut::Membership(x) => x.element_id_ref(),
            RelationshipRefMut::Unioning(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            RelationshipRefMut::Itself(x) => x.alias_ids_ref(),
            RelationshipRefMut::Association(x) => x.alias_ids_ref(),
            RelationshipRefMut::Connector(x) => x.alias_ids_ref(),
            RelationshipRefMut::Import(x) => x.alias_ids_ref(),
            RelationshipRefMut::Disjoining(x) => x.alias_ids_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.alias_ids_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.alias_ids_ref(),
            RelationshipRefMut::Annotation(x) => x.alias_ids_ref(),
            RelationshipRefMut::Dependency(x) => x.alias_ids_ref(),
            RelationshipRefMut::Differencing(x) => x.alias_ids_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.alias_ids_ref(),
            RelationshipRefMut::Specialization(x) => x.alias_ids_ref(),
            RelationshipRefMut::Intersecting(x) => x.alias_ids_ref(),
            RelationshipRefMut::Conjugation(x) => x.alias_ids_ref(),
            RelationshipRefMut::Membership(x) => x.alias_ids_ref(),
            RelationshipRefMut::Unioning(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            RelationshipRefMut::Itself(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Association(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Connector(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Import(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Disjoining(x) => x.declared_short_name_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.declared_short_name_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Annotation(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Dependency(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Differencing(x) => x.declared_short_name_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Specialization(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Intersecting(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Conjugation(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Membership(x) => x.declared_short_name_ref(),
            RelationshipRefMut::Unioning(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            RelationshipRefMut::Itself(x) => x.declared_name_ref(),
            RelationshipRefMut::Association(x) => x.declared_name_ref(),
            RelationshipRefMut::Connector(x) => x.declared_name_ref(),
            RelationshipRefMut::Import(x) => x.declared_name_ref(),
            RelationshipRefMut::Disjoining(x) => x.declared_name_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.declared_name_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.declared_name_ref(),
            RelationshipRefMut::Annotation(x) => x.declared_name_ref(),
            RelationshipRefMut::Dependency(x) => x.declared_name_ref(),
            RelationshipRefMut::Differencing(x) => x.declared_name_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.declared_name_ref(),
            RelationshipRefMut::Specialization(x) => x.declared_name_ref(),
            RelationshipRefMut::Intersecting(x) => x.declared_name_ref(),
            RelationshipRefMut::Conjugation(x) => x.declared_name_ref(),
            RelationshipRefMut::Membership(x) => x.declared_name_ref(),
            RelationshipRefMut::Unioning(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            RelationshipRefMut::Itself(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Association(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Connector(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Import(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Disjoining(x) => x.is_implied_included_ref(),
            RelationshipRefMut::FeatureInverting(x) => x.is_implied_included_ref(),
            RelationshipRefMut::FeatureChaining(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Annotation(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Dependency(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Differencing(x) => x.is_implied_included_ref(),
            RelationshipRefMut::TypeFeaturing(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Specialization(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Intersecting(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Conjugation(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Membership(x) => x.is_implied_included_ref(),
            RelationshipRefMut::Unioning(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for RelationshipRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RelationshipRef::Itself(x) => x.owned_relationship_ref(),
            RelationshipRef::Association(x) => x.owned_relationship_ref(),
            RelationshipRef::Connector(x) => x.owned_relationship_ref(),
            RelationshipRef::Import(x) => x.owned_relationship_ref(),
            RelationshipRef::Disjoining(x) => x.owned_relationship_ref(),
            RelationshipRef::FeatureInverting(x) => x.owned_relationship_ref(),
            RelationshipRef::FeatureChaining(x) => x.owned_relationship_ref(),
            RelationshipRef::Annotation(x) => x.owned_relationship_ref(),
            RelationshipRef::Dependency(x) => x.owned_relationship_ref(),
            RelationshipRef::Differencing(x) => x.owned_relationship_ref(),
            RelationshipRef::TypeFeaturing(x) => x.owned_relationship_ref(),
            RelationshipRef::Specialization(x) => x.owned_relationship_ref(),
            RelationshipRef::Intersecting(x) => x.owned_relationship_ref(),
            RelationshipRef::Conjugation(x) => x.owned_relationship_ref(),
            RelationshipRef::Membership(x) => x.owned_relationship_ref(),
            RelationshipRef::Unioning(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            RelationshipRef::Itself(x) => x.owning_relationship_ref(),
            RelationshipRef::Association(x) => x.owning_relationship_ref(),
            RelationshipRef::Connector(x) => x.owning_relationship_ref(),
            RelationshipRef::Import(x) => x.owning_relationship_ref(),
            RelationshipRef::Disjoining(x) => x.owning_relationship_ref(),
            RelationshipRef::FeatureInverting(x) => x.owning_relationship_ref(),
            RelationshipRef::FeatureChaining(x) => x.owning_relationship_ref(),
            RelationshipRef::Annotation(x) => x.owning_relationship_ref(),
            RelationshipRef::Dependency(x) => x.owning_relationship_ref(),
            RelationshipRef::Differencing(x) => x.owning_relationship_ref(),
            RelationshipRef::TypeFeaturing(x) => x.owning_relationship_ref(),
            RelationshipRef::Specialization(x) => x.owning_relationship_ref(),
            RelationshipRef::Intersecting(x) => x.owning_relationship_ref(),
            RelationshipRef::Conjugation(x) => x.owning_relationship_ref(),
            RelationshipRef::Membership(x) => x.owning_relationship_ref(),
            RelationshipRef::Unioning(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            RelationshipRef::Itself(x) => x.element_id_ref(),
            RelationshipRef::Association(x) => x.element_id_ref(),
            RelationshipRef::Connector(x) => x.element_id_ref(),
            RelationshipRef::Import(x) => x.element_id_ref(),
            RelationshipRef::Disjoining(x) => x.element_id_ref(),
            RelationshipRef::FeatureInverting(x) => x.element_id_ref(),
            RelationshipRef::FeatureChaining(x) => x.element_id_ref(),
            RelationshipRef::Annotation(x) => x.element_id_ref(),
            RelationshipRef::Dependency(x) => x.element_id_ref(),
            RelationshipRef::Differencing(x) => x.element_id_ref(),
            RelationshipRef::TypeFeaturing(x) => x.element_id_ref(),
            RelationshipRef::Specialization(x) => x.element_id_ref(),
            RelationshipRef::Intersecting(x) => x.element_id_ref(),
            RelationshipRef::Conjugation(x) => x.element_id_ref(),
            RelationshipRef::Membership(x) => x.element_id_ref(),
            RelationshipRef::Unioning(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            RelationshipRef::Itself(x) => x.alias_ids_ref(),
            RelationshipRef::Association(x) => x.alias_ids_ref(),
            RelationshipRef::Connector(x) => x.alias_ids_ref(),
            RelationshipRef::Import(x) => x.alias_ids_ref(),
            RelationshipRef::Disjoining(x) => x.alias_ids_ref(),
            RelationshipRef::FeatureInverting(x) => x.alias_ids_ref(),
            RelationshipRef::FeatureChaining(x) => x.alias_ids_ref(),
            RelationshipRef::Annotation(x) => x.alias_ids_ref(),
            RelationshipRef::Dependency(x) => x.alias_ids_ref(),
            RelationshipRef::Differencing(x) => x.alias_ids_ref(),
            RelationshipRef::TypeFeaturing(x) => x.alias_ids_ref(),
            RelationshipRef::Specialization(x) => x.alias_ids_ref(),
            RelationshipRef::Intersecting(x) => x.alias_ids_ref(),
            RelationshipRef::Conjugation(x) => x.alias_ids_ref(),
            RelationshipRef::Membership(x) => x.alias_ids_ref(),
            RelationshipRef::Unioning(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            RelationshipRef::Itself(x) => x.declared_short_name_ref(),
            RelationshipRef::Association(x) => x.declared_short_name_ref(),
            RelationshipRef::Connector(x) => x.declared_short_name_ref(),
            RelationshipRef::Import(x) => x.declared_short_name_ref(),
            RelationshipRef::Disjoining(x) => x.declared_short_name_ref(),
            RelationshipRef::FeatureInverting(x) => x.declared_short_name_ref(),
            RelationshipRef::FeatureChaining(x) => x.declared_short_name_ref(),
            RelationshipRef::Annotation(x) => x.declared_short_name_ref(),
            RelationshipRef::Dependency(x) => x.declared_short_name_ref(),
            RelationshipRef::Differencing(x) => x.declared_short_name_ref(),
            RelationshipRef::TypeFeaturing(x) => x.declared_short_name_ref(),
            RelationshipRef::Specialization(x) => x.declared_short_name_ref(),
            RelationshipRef::Intersecting(x) => x.declared_short_name_ref(),
            RelationshipRef::Conjugation(x) => x.declared_short_name_ref(),
            RelationshipRef::Membership(x) => x.declared_short_name_ref(),
            RelationshipRef::Unioning(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            RelationshipRef::Itself(x) => x.declared_name_ref(),
            RelationshipRef::Association(x) => x.declared_name_ref(),
            RelationshipRef::Connector(x) => x.declared_name_ref(),
            RelationshipRef::Import(x) => x.declared_name_ref(),
            RelationshipRef::Disjoining(x) => x.declared_name_ref(),
            RelationshipRef::FeatureInverting(x) => x.declared_name_ref(),
            RelationshipRef::FeatureChaining(x) => x.declared_name_ref(),
            RelationshipRef::Annotation(x) => x.declared_name_ref(),
            RelationshipRef::Dependency(x) => x.declared_name_ref(),
            RelationshipRef::Differencing(x) => x.declared_name_ref(),
            RelationshipRef::TypeFeaturing(x) => x.declared_name_ref(),
            RelationshipRef::Specialization(x) => x.declared_name_ref(),
            RelationshipRef::Intersecting(x) => x.declared_name_ref(),
            RelationshipRef::Conjugation(x) => x.declared_name_ref(),
            RelationshipRef::Membership(x) => x.declared_name_ref(),
            RelationshipRef::Unioning(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            RelationshipRef::Itself(x) => x.is_implied_included_ref(),
            RelationshipRef::Association(x) => x.is_implied_included_ref(),
            RelationshipRef::Connector(x) => x.is_implied_included_ref(),
            RelationshipRef::Import(x) => x.is_implied_included_ref(),
            RelationshipRef::Disjoining(x) => x.is_implied_included_ref(),
            RelationshipRef::FeatureInverting(x) => x.is_implied_included_ref(),
            RelationshipRef::FeatureChaining(x) => x.is_implied_included_ref(),
            RelationshipRef::Annotation(x) => x.is_implied_included_ref(),
            RelationshipRef::Dependency(x) => x.is_implied_included_ref(),
            RelationshipRef::Differencing(x) => x.is_implied_included_ref(),
            RelationshipRef::TypeFeaturing(x) => x.is_implied_included_ref(),
            RelationshipRef::Specialization(x) => x.is_implied_included_ref(),
            RelationshipRef::Intersecting(x) => x.is_implied_included_ref(),
            RelationshipRef::Conjugation(x) => x.is_implied_included_ref(),
            RelationshipRef::Membership(x) => x.is_implied_included_ref(),
            RelationshipRef::Unioning(x) => x.is_implied_included_ref(),
        }
    }
}
impl RelationshipUpcast for Relationship {
    fn into_relationship(self) -> Relationship {
        self
    }
}
impl<'a> RelationshipUpcastRefMut<'a> for RelationshipRefMut<'a> {
    fn as_relationship_ref_mut(self) -> RelationshipRefMut<'a> {
        self
    }
}
impl<'a> RelationshipUpcastRef<'a> for RelationshipRef<'a> {
    fn as_relationship_ref(self) -> RelationshipRef<'a> {
        self
    }
}
impl ElementUpcast for Relationship {
    fn into_element(self) -> Element {
        Element::Relationship(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for RelationshipRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ElementRefMut::Relationship(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for RelationshipRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ElementRef::Relationship(self).as_element_ref()
    }
}
pub trait RelationshipDowncast {
    fn try_into_association(self) -> Result<Association, String>;
    fn try_into_connector(self) -> Result<Connector, String>;
    fn try_into_import(self) -> Result<Import, String>;
    fn try_into_disjoining(self) -> Result<Disjoining, String>;
    fn try_into_feature_inverting(self) -> Result<FeatureInverting, String>;
    fn try_into_feature_chaining(self) -> Result<FeatureChaining, String>;
    fn try_into_annotation(self) -> Result<Annotation, String>;
    fn try_into_dependency(self) -> Result<Dependency, String>;
    fn try_into_differencing(self) -> Result<Differencing, String>;
    fn try_into_type_featuring(self) -> Result<TypeFeaturing, String>;
    fn try_into_specialization(self) -> Result<Specialization, String>;
    fn try_into_intersecting(self) -> Result<Intersecting, String>;
    fn try_into_conjugation(self) -> Result<Conjugation, String>;
    fn try_into_membership(self) -> Result<Membership, String>;
    fn try_into_unioning(self) -> Result<Unioning, String>;
}
pub trait RelationshipDowncastRefMut<'a> {
    fn try_as_association_ref_mut(self) -> Result<AssociationRefMut<'a>, String>;
    fn try_as_connector_ref_mut(self) -> Result<ConnectorRefMut<'a>, String>;
    fn try_as_import_ref_mut(self) -> Result<ImportRefMut<'a>, String>;
    fn try_as_disjoining_ref_mut(self) -> Result<DisjoiningRefMut<'a>, String>;
    fn try_as_feature_inverting_ref_mut(
        self,
    ) -> Result<FeatureInvertingRefMut<'a>, String>;
    fn try_as_feature_chaining_ref_mut(
        self,
    ) -> Result<FeatureChainingRefMut<'a>, String>;
    fn try_as_annotation_ref_mut(self) -> Result<AnnotationRefMut<'a>, String>;
    fn try_as_dependency_ref_mut(self) -> Result<DependencyRefMut<'a>, String>;
    fn try_as_differencing_ref_mut(self) -> Result<DifferencingRefMut<'a>, String>;
    fn try_as_type_featuring_ref_mut(self) -> Result<TypeFeaturingRefMut<'a>, String>;
    fn try_as_specialization_ref_mut(self) -> Result<SpecializationRefMut<'a>, String>;
    fn try_as_intersecting_ref_mut(self) -> Result<IntersectingRefMut<'a>, String>;
    fn try_as_conjugation_ref_mut(self) -> Result<ConjugationRefMut<'a>, String>;
    fn try_as_membership_ref_mut(self) -> Result<MembershipRefMut<'a>, String>;
    fn try_as_unioning_ref_mut(self) -> Result<UnioningRefMut<'a>, String>;
}
pub trait RelationshipDowncastRef<'a> {
    fn try_as_association_ref(self) -> Result<AssociationRef<'a>, String>;
    fn try_as_connector_ref(self) -> Result<ConnectorRef<'a>, String>;
    fn try_as_import_ref(self) -> Result<ImportRef<'a>, String>;
    fn try_as_disjoining_ref(self) -> Result<DisjoiningRef<'a>, String>;
    fn try_as_feature_inverting_ref(self) -> Result<FeatureInvertingRef<'a>, String>;
    fn try_as_feature_chaining_ref(self) -> Result<FeatureChainingRef<'a>, String>;
    fn try_as_annotation_ref(self) -> Result<AnnotationRef<'a>, String>;
    fn try_as_dependency_ref(self) -> Result<DependencyRef<'a>, String>;
    fn try_as_differencing_ref(self) -> Result<DifferencingRef<'a>, String>;
    fn try_as_type_featuring_ref(self) -> Result<TypeFeaturingRef<'a>, String>;
    fn try_as_specialization_ref(self) -> Result<SpecializationRef<'a>, String>;
    fn try_as_intersecting_ref(self) -> Result<IntersectingRef<'a>, String>;
    fn try_as_conjugation_ref(self) -> Result<ConjugationRef<'a>, String>;
    fn try_as_membership_ref(self) -> Result<MembershipRef<'a>, String>;
    fn try_as_unioning_ref(self) -> Result<UnioningRef<'a>, String>;
}
impl RelationshipDowncast for Relationship {
    fn try_into_association(self) -> Result<Association, String> {
        match self {
            Relationship::Association(e) => Ok(e),
            _ => Err("Not a Association".into()),
        }
    }
    fn try_into_connector(self) -> Result<Connector, String> {
        match self {
            Relationship::Connector(e) => Ok(e),
            _ => Err("Not a Connector".into()),
        }
    }
    fn try_into_import(self) -> Result<Import, String> {
        match self {
            Relationship::Import(e) => Ok(e),
            _ => Err("Not a Import".into()),
        }
    }
    fn try_into_disjoining(self) -> Result<Disjoining, String> {
        match self {
            Relationship::Disjoining(e) => Ok(e),
            _ => Err("Not a Disjoining".into()),
        }
    }
    fn try_into_feature_inverting(self) -> Result<FeatureInverting, String> {
        match self {
            Relationship::FeatureInverting(e) => Ok(e),
            _ => Err("Not a FeatureInverting".into()),
        }
    }
    fn try_into_feature_chaining(self) -> Result<FeatureChaining, String> {
        match self {
            Relationship::FeatureChaining(e) => Ok(e),
            _ => Err("Not a FeatureChaining".into()),
        }
    }
    fn try_into_annotation(self) -> Result<Annotation, String> {
        match self {
            Relationship::Annotation(e) => Ok(e),
            _ => Err("Not a Annotation".into()),
        }
    }
    fn try_into_dependency(self) -> Result<Dependency, String> {
        match self {
            Relationship::Dependency(e) => Ok(e),
            _ => Err("Not a Dependency".into()),
        }
    }
    fn try_into_differencing(self) -> Result<Differencing, String> {
        match self {
            Relationship::Differencing(e) => Ok(e),
            _ => Err("Not a Differencing".into()),
        }
    }
    fn try_into_type_featuring(self) -> Result<TypeFeaturing, String> {
        match self {
            Relationship::TypeFeaturing(e) => Ok(e),
            _ => Err("Not a TypeFeaturing".into()),
        }
    }
    fn try_into_specialization(self) -> Result<Specialization, String> {
        match self {
            Relationship::Specialization(e) => Ok(e),
            _ => Err("Not a Specialization".into()),
        }
    }
    fn try_into_intersecting(self) -> Result<Intersecting, String> {
        match self {
            Relationship::Intersecting(e) => Ok(e),
            _ => Err("Not a Intersecting".into()),
        }
    }
    fn try_into_conjugation(self) -> Result<Conjugation, String> {
        match self {
            Relationship::Conjugation(e) => Ok(e),
            _ => Err("Not a Conjugation".into()),
        }
    }
    fn try_into_membership(self) -> Result<Membership, String> {
        match self {
            Relationship::Membership(e) => Ok(e),
            _ => Err("Not a Membership".into()),
        }
    }
    fn try_into_unioning(self) -> Result<Unioning, String> {
        match self {
            Relationship::Unioning(e) => Ok(e),
            _ => Err("Not a Unioning".into()),
        }
    }
}
impl<'a> RelationshipDowncastRefMut<'a> for RelationshipRefMut<'a> {
    fn try_as_association_ref_mut(self) -> Result<AssociationRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Association(e) => Ok(e),
            _ => Err("Not a Association".into()),
        }
    }
    fn try_as_connector_ref_mut(self) -> Result<ConnectorRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Connector(e) => Ok(e),
            _ => Err("Not a Connector".into()),
        }
    }
    fn try_as_import_ref_mut(self) -> Result<ImportRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Import(e) => Ok(e),
            _ => Err("Not a Import".into()),
        }
    }
    fn try_as_disjoining_ref_mut(self) -> Result<DisjoiningRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Disjoining(e) => Ok(e),
            _ => Err("Not a Disjoining".into()),
        }
    }
    fn try_as_feature_inverting_ref_mut(
        self,
    ) -> Result<FeatureInvertingRefMut<'a>, String> {
        match self {
            RelationshipRefMut::FeatureInverting(e) => Ok(e),
            _ => Err("Not a FeatureInverting".into()),
        }
    }
    fn try_as_feature_chaining_ref_mut(
        self,
    ) -> Result<FeatureChainingRefMut<'a>, String> {
        match self {
            RelationshipRefMut::FeatureChaining(e) => Ok(e),
            _ => Err("Not a FeatureChaining".into()),
        }
    }
    fn try_as_annotation_ref_mut(self) -> Result<AnnotationRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Annotation(e) => Ok(e),
            _ => Err("Not a Annotation".into()),
        }
    }
    fn try_as_dependency_ref_mut(self) -> Result<DependencyRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Dependency(e) => Ok(e),
            _ => Err("Not a Dependency".into()),
        }
    }
    fn try_as_differencing_ref_mut(self) -> Result<DifferencingRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Differencing(e) => Ok(e),
            _ => Err("Not a Differencing".into()),
        }
    }
    fn try_as_type_featuring_ref_mut(self) -> Result<TypeFeaturingRefMut<'a>, String> {
        match self {
            RelationshipRefMut::TypeFeaturing(e) => Ok(e),
            _ => Err("Not a TypeFeaturing".into()),
        }
    }
    fn try_as_specialization_ref_mut(self) -> Result<SpecializationRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Specialization(e) => Ok(e),
            _ => Err("Not a Specialization".into()),
        }
    }
    fn try_as_intersecting_ref_mut(self) -> Result<IntersectingRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Intersecting(e) => Ok(e),
            _ => Err("Not a Intersecting".into()),
        }
    }
    fn try_as_conjugation_ref_mut(self) -> Result<ConjugationRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Conjugation(e) => Ok(e),
            _ => Err("Not a Conjugation".into()),
        }
    }
    fn try_as_membership_ref_mut(self) -> Result<MembershipRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Membership(e) => Ok(e),
            _ => Err("Not a Membership".into()),
        }
    }
    fn try_as_unioning_ref_mut(self) -> Result<UnioningRefMut<'a>, String> {
        match self {
            RelationshipRefMut::Unioning(e) => Ok(e),
            _ => Err("Not a Unioning".into()),
        }
    }
}
impl<'a> RelationshipDowncastRef<'a> for RelationshipRef<'a> {
    fn try_as_association_ref(self) -> Result<AssociationRef<'a>, String> {
        match self {
            RelationshipRef::Association(e) => Ok(e),
            _ => Err("Not a Association".into()),
        }
    }
    fn try_as_connector_ref(self) -> Result<ConnectorRef<'a>, String> {
        match self {
            RelationshipRef::Connector(e) => Ok(e),
            _ => Err("Not a Connector".into()),
        }
    }
    fn try_as_import_ref(self) -> Result<ImportRef<'a>, String> {
        match self {
            RelationshipRef::Import(e) => Ok(e),
            _ => Err("Not a Import".into()),
        }
    }
    fn try_as_disjoining_ref(self) -> Result<DisjoiningRef<'a>, String> {
        match self {
            RelationshipRef::Disjoining(e) => Ok(e),
            _ => Err("Not a Disjoining".into()),
        }
    }
    fn try_as_feature_inverting_ref(self) -> Result<FeatureInvertingRef<'a>, String> {
        match self {
            RelationshipRef::FeatureInverting(e) => Ok(e),
            _ => Err("Not a FeatureInverting".into()),
        }
    }
    fn try_as_feature_chaining_ref(self) -> Result<FeatureChainingRef<'a>, String> {
        match self {
            RelationshipRef::FeatureChaining(e) => Ok(e),
            _ => Err("Not a FeatureChaining".into()),
        }
    }
    fn try_as_annotation_ref(self) -> Result<AnnotationRef<'a>, String> {
        match self {
            RelationshipRef::Annotation(e) => Ok(e),
            _ => Err("Not a Annotation".into()),
        }
    }
    fn try_as_dependency_ref(self) -> Result<DependencyRef<'a>, String> {
        match self {
            RelationshipRef::Dependency(e) => Ok(e),
            _ => Err("Not a Dependency".into()),
        }
    }
    fn try_as_differencing_ref(self) -> Result<DifferencingRef<'a>, String> {
        match self {
            RelationshipRef::Differencing(e) => Ok(e),
            _ => Err("Not a Differencing".into()),
        }
    }
    fn try_as_type_featuring_ref(self) -> Result<TypeFeaturingRef<'a>, String> {
        match self {
            RelationshipRef::TypeFeaturing(e) => Ok(e),
            _ => Err("Not a TypeFeaturing".into()),
        }
    }
    fn try_as_specialization_ref(self) -> Result<SpecializationRef<'a>, String> {
        match self {
            RelationshipRef::Specialization(e) => Ok(e),
            _ => Err("Not a Specialization".into()),
        }
    }
    fn try_as_intersecting_ref(self) -> Result<IntersectingRef<'a>, String> {
        match self {
            RelationshipRef::Intersecting(e) => Ok(e),
            _ => Err("Not a Intersecting".into()),
        }
    }
    fn try_as_conjugation_ref(self) -> Result<ConjugationRef<'a>, String> {
        match self {
            RelationshipRef::Conjugation(e) => Ok(e),
            _ => Err("Not a Conjugation".into()),
        }
    }
    fn try_as_membership_ref(self) -> Result<MembershipRef<'a>, String> {
        match self {
            RelationshipRef::Membership(e) => Ok(e),
            _ => Err("Not a Membership".into()),
        }
    }
    fn try_as_unioning_ref(self) -> Result<UnioningRef<'a>, String> {
        match self {
            RelationshipRef::Unioning(e) => Ok(e),
            _ => Err("Not a Unioning".into()),
        }
    }
}
pub trait RelationshipMethodsDescendants
where
    Self: DescendantOf<Relationship>,
    Self::Via: RelationshipMethods,
    Self: Sized,
{}
pub trait RelationshipMethods: Sized {}
impl<T: RelationshipMethodsDescendants> RelationshipMethods for T
where
    T::Via: RelationshipMethods,
{}
impl DescendantOf<Element> for Relationship {
    type Via = Element;
    fn into_via(self) -> Self::Via {
        self.into_element()
    }
}
impl ElementMethodsDescendants for Relationship {}
pub trait RelationshipRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<RelationshipRefMut<'a>>,
    Self::Via: RelationshipRefMutMethods,
    Self: Sized,
{}
pub trait RelationshipRefMutMethods: Sized {}
impl<'a, T: RelationshipRefMutMethodsDescendants<'a>> RelationshipRefMutMethods for T
where
    T::Via: RelationshipRefMutMethods,
{}
impl<'a> DescendantOf<ElementRefMut<'a>> for RelationshipRefMut<'a> {
    type Via = ElementRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_element_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for RelationshipRefMut<'a> {}
pub trait RelationshipRefMethodsDescendants<'a>
where
    Self: DescendantOf<RelationshipRef<'a>>,
    Self::Via: RelationshipRefMethods,
    Self: Sized,
{}
pub trait RelationshipRefMethods: Sized {}
impl<'a, T: RelationshipRefMethodsDescendants<'a>> RelationshipRefMethods for T
where
    T::Via: RelationshipRefMethods,
{}
impl<'a> DescendantOf<ElementRef<'a>> for RelationshipRef<'a> {
    type Via = ElementRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_element_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for RelationshipRef<'a> {}

