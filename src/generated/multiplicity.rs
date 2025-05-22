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
use super::multiplicity_range::{
    MultiplicityRange, MultiplicityRangeRefMut, MultiplicityRangeRef,
};
pub struct MultiplicityInner {
    pub(super) sup_feature: FeatureInner,
}
pub trait MultiplicityStruct
where
    Self: MultiplicityStructRefMut,
    Self: MultiplicityStructRef,
    Self: FeatureStruct,
{}
pub trait MultiplicityStructRefMut
where
    Self: MultiplicityStructRef,
    Self: FeatureStructRefMut,
{}
pub trait MultiplicityStructRef
where
    Self: FeatureStructRef,
{}
pub trait MultiplicityUpcast: MultiplicityStruct {
    fn into_multiplicity(self) -> Multiplicity;
}
pub trait MultiplicityUpcastRefMut<'a>: MultiplicityStructRefMut {
    fn as_multiplicity_ref_mut(self) -> MultiplicityRefMut<'a>;
}
pub trait MultiplicityUpcastRef<'a>: MultiplicityStructRef {
    fn as_multiplicity_ref(self) -> MultiplicityRef<'a>;
}
impl MultiplicityStruct for MultiplicityInner {}
impl MultiplicityStructRefMut for MultiplicityInner {}
impl MultiplicityStructRef for MultiplicityInner {}
impl FeatureStruct for MultiplicityInner {
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
impl FeatureStructRefMut for MultiplicityInner {
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
impl FeatureStructRef for MultiplicityInner {
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
impl TypeStruct for MultiplicityInner {
    fn is_abstract(self) -> bool {
        self.sup_feature.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_feature.is_sufficient()
    }
}
impl TypeStructRefMut for MultiplicityInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_feature.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for MultiplicityInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_feature.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_feature.is_sufficient_ref()
    }
}
impl NamespaceStruct for MultiplicityInner {}
impl NamespaceStructRefMut for MultiplicityInner {}
impl NamespaceStructRef for MultiplicityInner {}
impl ElementStruct for MultiplicityInner {
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
impl ElementStructRefMut for MultiplicityInner {
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
impl ElementStructRef for MultiplicityInner {
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
pub enum Multiplicity {
    Itself(MultiplicityInner),
    MultiplicityRange(MultiplicityRange),
}
pub enum MultiplicityRefMut<'a> {
    Itself(&'a mut MultiplicityInner),
    MultiplicityRange(MultiplicityRangeRefMut<'a>),
}
pub enum MultiplicityRef<'a> {
    Itself(&'a MultiplicityInner),
    MultiplicityRange(MultiplicityRangeRef<'a>),
}
impl Multiplicity {
    pub fn as_ref(&self) -> MultiplicityRef {
        match self {
            Multiplicity::Itself(inner) => MultiplicityRef::Itself(&inner),
            Multiplicity::MultiplicityRange(inner) => {
                MultiplicityRef::MultiplicityRange(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> MultiplicityRefMut {
        match self {
            Multiplicity::Itself(inner) => MultiplicityRefMut::Itself(inner),
            Multiplicity::MultiplicityRange(inner) => {
                MultiplicityRefMut::MultiplicityRange(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> MultiplicityRefMut<'a> {
    pub fn as_ref(self) -> MultiplicityRef<'a> {
        match self {
            MultiplicityRefMut::Itself(inner) => MultiplicityRef::Itself(inner),
            MultiplicityRefMut::MultiplicityRange(inner) => {
                MultiplicityRef::MultiplicityRange(inner.as_ref())
            }
        }
    }
}
impl MultiplicityStruct for Multiplicity {}
impl MultiplicityStructRefMut for Multiplicity {}
impl MultiplicityStructRef for Multiplicity {}
impl<'a> MultiplicityStructRefMut for MultiplicityRefMut<'a> {}
impl<'a> MultiplicityStructRef for MultiplicityRefMut<'a> {}
impl<'a> MultiplicityStructRef for MultiplicityRef<'a> {}
impl FeatureStruct for Multiplicity {
    fn is_unique(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_unique(),
            Multiplicity::MultiplicityRange(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_ordered(),
            Multiplicity::MultiplicityRange(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_composite(),
            Multiplicity::MultiplicityRange(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_end(),
            Multiplicity::MultiplicityRange(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_derived(),
            Multiplicity::MultiplicityRange(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_portion(),
            Multiplicity::MultiplicityRange(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_variable(),
            Multiplicity::MultiplicityRange(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_constant(),
            Multiplicity::MultiplicityRange(x) => x.is_constant(),
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
            Multiplicity::Itself(x) => x.direction(),
            Multiplicity::MultiplicityRange(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Multiplicity {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_unique_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_ordered_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_composite_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_end_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_derived_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_portion_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_variable_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_constant_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_constant_ref_mut(),
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
            Multiplicity::Itself(x) => x.direction_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Multiplicity {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_unique_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_ordered_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_composite_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_end_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_derived_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_portion_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_variable_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_constant_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_constant_ref(),
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
            Multiplicity::Itself(x) => x.direction_ref(),
            Multiplicity::MultiplicityRange(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for MultiplicityRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_unique_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_ordered_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_composite_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_end_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_derived_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_portion_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_variable_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_constant_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_constant_ref_mut(),
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
            MultiplicityRefMut::Itself(x) => x.direction_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for MultiplicityRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_unique_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_ordered_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_composite_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_end_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_derived_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_portion_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_variable_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_constant_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_constant_ref(),
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
            MultiplicityRefMut::Itself(x) => x.direction_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for MultiplicityRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_unique_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_ordered_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_composite_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_end_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_derived_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_portion_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_variable_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_constant_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_constant_ref(),
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
            MultiplicityRef::Itself(x) => x.direction_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Multiplicity {
    fn is_abstract(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_abstract(),
            Multiplicity::MultiplicityRange(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_sufficient(),
            Multiplicity::MultiplicityRange(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Multiplicity {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_abstract_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_sufficient_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Multiplicity {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_abstract_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_sufficient_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for MultiplicityRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_abstract_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for MultiplicityRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_abstract_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_sufficient_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for MultiplicityRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_abstract_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_sufficient_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Multiplicity {}
impl NamespaceStructRefMut for Multiplicity {}
impl NamespaceStructRef for Multiplicity {}
impl<'a> NamespaceStructRefMut for MultiplicityRefMut<'a> {}
impl<'a> NamespaceStructRef for MultiplicityRefMut<'a> {}
impl<'a> NamespaceStructRef for MultiplicityRef<'a> {}
impl ElementStruct for Multiplicity {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Multiplicity::Itself(x) => x.owned_relationship(),
            Multiplicity::MultiplicityRange(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Multiplicity::Itself(x) => x.owning_relationship(),
            Multiplicity::MultiplicityRange(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Multiplicity::Itself(x) => x.element_id(),
            Multiplicity::MultiplicityRange(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Multiplicity::Itself(x) => x.alias_ids(),
            Multiplicity::MultiplicityRange(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Multiplicity::Itself(x) => x.declared_short_name(),
            Multiplicity::MultiplicityRange(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Multiplicity::Itself(x) => x.declared_name(),
            Multiplicity::MultiplicityRange(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Multiplicity::Itself(x) => x.is_implied_included(),
            Multiplicity::MultiplicityRange(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Multiplicity {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Multiplicity::Itself(x) => x.owned_relationship_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Multiplicity::Itself(x) => x.owning_relationship_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Multiplicity::Itself(x) => x.element_id_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Multiplicity::Itself(x) => x.alias_ids_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Multiplicity::Itself(x) => x.declared_short_name_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Multiplicity::Itself(x) => x.declared_name_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Multiplicity::Itself(x) => x.is_implied_included_ref_mut(),
            Multiplicity::MultiplicityRange(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Multiplicity {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Multiplicity::Itself(x) => x.owned_relationship_ref(),
            Multiplicity::MultiplicityRange(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Multiplicity::Itself(x) => x.owning_relationship_ref(),
            Multiplicity::MultiplicityRange(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Multiplicity::Itself(x) => x.element_id_ref(),
            Multiplicity::MultiplicityRange(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Multiplicity::Itself(x) => x.alias_ids_ref(),
            Multiplicity::MultiplicityRange(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Multiplicity::Itself(x) => x.declared_short_name_ref(),
            Multiplicity::MultiplicityRange(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Multiplicity::Itself(x) => x.declared_name_ref(),
            Multiplicity::MultiplicityRange(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Multiplicity::Itself(x) => x.is_implied_included_ref(),
            Multiplicity::MultiplicityRange(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MultiplicityRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MultiplicityRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MultiplicityRefMut::Itself(x) => x.element_id_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MultiplicityRefMut::Itself(x) => x.alias_ids_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MultiplicityRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MultiplicityRefMut::Itself(x) => x.declared_name_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MultiplicityRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRefMut::Itself(x) => x.owned_relationship_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRefMut::Itself(x) => x.owning_relationship_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MultiplicityRefMut::Itself(x) => x.element_id_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MultiplicityRefMut::Itself(x) => x.alias_ids_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRefMut::Itself(x) => x.declared_short_name_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRefMut::Itself(x) => x.declared_name_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MultiplicityRefMut::Itself(x) => x.is_implied_included_ref(),
            MultiplicityRefMut::MultiplicityRange(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MultiplicityRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRef::Itself(x) => x.owned_relationship_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRef::Itself(x) => x.owning_relationship_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MultiplicityRef::Itself(x) => x.element_id_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MultiplicityRef::Itself(x) => x.alias_ids_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRef::Itself(x) => x.declared_short_name_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRef::Itself(x) => x.declared_name_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MultiplicityRef::Itself(x) => x.is_implied_included_ref(),
            MultiplicityRef::MultiplicityRange(x) => x.is_implied_included_ref(),
        }
    }
}
impl MultiplicityUpcast for Multiplicity {
    fn into_multiplicity(self) -> Multiplicity {
        self
    }
}
impl<'a> MultiplicityUpcastRefMut<'a> for MultiplicityRefMut<'a> {
    fn as_multiplicity_ref_mut(self) -> MultiplicityRefMut<'a> {
        self
    }
}
impl<'a> MultiplicityUpcastRef<'a> for MultiplicityRef<'a> {
    fn as_multiplicity_ref(self) -> MultiplicityRef<'a> {
        self
    }
}
impl FeatureUpcast for Multiplicity {
    fn into_feature(self) -> Feature {
        Feature::Multiplicity(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for MultiplicityRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        FeatureRefMut::Multiplicity(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for MultiplicityRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        FeatureRef::Multiplicity(self).as_feature_ref()
    }
}
impl TypeUpcast for Multiplicity {
    fn into_type_(self) -> Type {
        Feature::Multiplicity(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for MultiplicityRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        FeatureRefMut::Multiplicity(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for MultiplicityRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        FeatureRef::Multiplicity(self).as_type_ref()
    }
}
impl NamespaceUpcast for Multiplicity {
    fn into_namespace(self) -> Namespace {
        Feature::Multiplicity(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for MultiplicityRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        FeatureRefMut::Multiplicity(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for MultiplicityRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        FeatureRef::Multiplicity(self).as_namespace_ref()
    }
}
impl ElementUpcast for Multiplicity {
    fn into_element(self) -> Element {
        Feature::Multiplicity(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MultiplicityRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        FeatureRefMut::Multiplicity(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MultiplicityRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        FeatureRef::Multiplicity(self).as_element_ref()
    }
}
pub trait MultiplicityDowncast {
    fn try_into_multiplicity_range(self) -> Result<MultiplicityRange, String>;
}
pub trait MultiplicityDowncastRefMut<'a> {
    fn try_as_multiplicity_range_ref_mut(
        self,
    ) -> Result<MultiplicityRangeRefMut<'a>, String>;
}
pub trait MultiplicityDowncastRef<'a> {
    fn try_as_multiplicity_range_ref(self) -> Result<MultiplicityRangeRef<'a>, String>;
}
impl MultiplicityDowncast for Multiplicity {
    fn try_into_multiplicity_range(self) -> Result<MultiplicityRange, String> {
        match self {
            Multiplicity::MultiplicityRange(e) => Ok(e),
            _ => Err("Not a MultiplicityRange".into()),
        }
    }
}
impl<'a> MultiplicityDowncastRefMut<'a> for MultiplicityRefMut<'a> {
    fn try_as_multiplicity_range_ref_mut(
        self,
    ) -> Result<MultiplicityRangeRefMut<'a>, String> {
        match self {
            MultiplicityRefMut::MultiplicityRange(e) => Ok(e),
            _ => Err("Not a MultiplicityRange".into()),
        }
    }
}
impl<'a> MultiplicityDowncastRef<'a> for MultiplicityRef<'a> {
    fn try_as_multiplicity_range_ref(self) -> Result<MultiplicityRangeRef<'a>, String> {
        match self {
            MultiplicityRef::MultiplicityRange(e) => Ok(e),
            _ => Err("Not a MultiplicityRange".into()),
        }
    }
}
pub trait MultiplicityMethodsDescendants
where
    Self: DescendantOf<Multiplicity>,
    Self::Via: MultiplicityMethods,
    Self: Sized,
{}
pub trait MultiplicityMethods: Sized {}
impl<T: MultiplicityMethodsDescendants> MultiplicityMethods for T
where
    T::Via: MultiplicityMethods,
{}
impl DescendantOf<Feature> for Multiplicity {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl FeatureMethodsDescendants for Multiplicity {}
impl DescendantOf<Type> for Multiplicity {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl TypeMethodsDescendants for Multiplicity {}
impl DescendantOf<Namespace> for Multiplicity {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl NamespaceMethodsDescendants for Multiplicity {}
impl DescendantOf<Element> for Multiplicity {
    type Via = Feature;
    fn into_via(self) -> Self::Via {
        self.into_feature()
    }
}
impl ElementMethodsDescendants for Multiplicity {}
pub trait MultiplicityRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MultiplicityRefMut<'a>>,
    Self::Via: MultiplicityRefMutMethods,
    Self: Sized,
{}
pub trait MultiplicityRefMutMethods: Sized {}
impl<'a, T: MultiplicityRefMutMethodsDescendants<'a>> MultiplicityRefMutMethods for T
where
    T::Via: MultiplicityRefMutMethods,
{}
impl<'a> DescendantOf<FeatureRefMut<'a>> for MultiplicityRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for MultiplicityRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for MultiplicityRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for MultiplicityRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for MultiplicityRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for MultiplicityRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MultiplicityRefMut<'a> {
    type Via = FeatureRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MultiplicityRefMut<'a> {}
pub trait MultiplicityRefMethodsDescendants<'a>
where
    Self: DescendantOf<MultiplicityRef<'a>>,
    Self::Via: MultiplicityRefMethods,
    Self: Sized,
{}
pub trait MultiplicityRefMethods: Sized {}
impl<'a, T: MultiplicityRefMethodsDescendants<'a>> MultiplicityRefMethods for T
where
    T::Via: MultiplicityRefMethods,
{}
impl<'a> DescendantOf<FeatureRef<'a>> for MultiplicityRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for MultiplicityRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for MultiplicityRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for MultiplicityRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for MultiplicityRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for MultiplicityRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MultiplicityRef<'a> {
    type Via = FeatureRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_feature_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MultiplicityRef<'a> {}

