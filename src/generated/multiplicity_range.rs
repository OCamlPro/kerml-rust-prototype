#![allow(unused)]
use super::utils::DescendantOf;
use super::multiplicity::{
    Multiplicity, MultiplicityRefMut, MultiplicityRef, MultiplicityStruct,
    MultiplicityStructRefMut, MultiplicityStructRef, MultiplicityInner,
    MultiplicityUpcast, MultiplicityUpcastRefMut, MultiplicityUpcastRef,
    MultiplicityMethodsDescendants, MultiplicityRefMutMethodsDescendants,
    MultiplicityRefMethodsDescendants,
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
pub struct MultiplicityRangeInner {
    pub(super) sup_multiplicity: MultiplicityInner,
}
pub trait MultiplicityRangeStruct
where
    Self: MultiplicityRangeStructRefMut,
    Self: MultiplicityRangeStructRef,
    Self: MultiplicityStruct,
{}
pub trait MultiplicityRangeStructRefMut
where
    Self: MultiplicityRangeStructRef,
    Self: MultiplicityStructRefMut,
{}
pub trait MultiplicityRangeStructRef
where
    Self: MultiplicityStructRef,
{}
pub trait MultiplicityRangeUpcast: MultiplicityRangeStruct {
    fn into_multiplicity_range(self) -> MultiplicityRange;
}
pub trait MultiplicityRangeUpcastRefMut<'a>: MultiplicityRangeStructRefMut {
    fn as_multiplicity_range_ref_mut(self) -> MultiplicityRangeRefMut<'a>;
}
pub trait MultiplicityRangeUpcastRef<'a>: MultiplicityRangeStructRef {
    fn as_multiplicity_range_ref(self) -> MultiplicityRangeRef<'a>;
}
impl MultiplicityRangeStruct for MultiplicityRangeInner {}
impl MultiplicityRangeStructRefMut for MultiplicityRangeInner {}
impl MultiplicityRangeStructRef for MultiplicityRangeInner {}
impl MultiplicityStruct for MultiplicityRangeInner {}
impl MultiplicityStructRefMut for MultiplicityRangeInner {}
impl MultiplicityStructRef for MultiplicityRangeInner {}
impl FeatureStruct for MultiplicityRangeInner {
    fn is_unique(self) -> bool {
        self.sup_multiplicity.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_multiplicity.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_multiplicity.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_multiplicity.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_multiplicity.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_multiplicity.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_multiplicity.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_multiplicity.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_multiplicity.direction()
    }
}
impl FeatureStructRefMut for MultiplicityRangeInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_multiplicity.direction_ref_mut()
    }
}
impl FeatureStructRef for MultiplicityRangeInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_multiplicity.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_multiplicity.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_multiplicity.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_multiplicity.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_multiplicity.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_multiplicity.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_multiplicity.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_multiplicity.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_multiplicity.direction_ref()
    }
}
impl TypeStruct for MultiplicityRangeInner {
    fn is_abstract(self) -> bool {
        self.sup_multiplicity.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_multiplicity.is_sufficient()
    }
}
impl TypeStructRefMut for MultiplicityRangeInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for MultiplicityRangeInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_multiplicity.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_multiplicity.is_sufficient_ref()
    }
}
impl NamespaceStruct for MultiplicityRangeInner {}
impl NamespaceStructRefMut for MultiplicityRangeInner {}
impl NamespaceStructRef for MultiplicityRangeInner {}
impl ElementStruct for MultiplicityRangeInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_multiplicity.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_multiplicity.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_multiplicity.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_multiplicity.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_multiplicity.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_multiplicity.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_multiplicity.is_implied_included()
    }
}
impl ElementStructRefMut for MultiplicityRangeInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_multiplicity.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_multiplicity.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_multiplicity.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_multiplicity.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_multiplicity.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_multiplicity.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_multiplicity.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for MultiplicityRangeInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_multiplicity.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_multiplicity.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_multiplicity.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_multiplicity.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_multiplicity.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_multiplicity.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_multiplicity.is_implied_included_ref()
    }
}
pub enum MultiplicityRange {
    Itself(MultiplicityRangeInner),
}
pub enum MultiplicityRangeRefMut<'a> {
    Itself(&'a mut MultiplicityRangeInner),
}
pub enum MultiplicityRangeRef<'a> {
    Itself(&'a MultiplicityRangeInner),
}
impl MultiplicityRange {
    pub fn as_ref(&self) -> MultiplicityRangeRef {
        match self {
            MultiplicityRange::Itself(inner) => MultiplicityRangeRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> MultiplicityRangeRefMut {
        match self {
            MultiplicityRange::Itself(inner) => MultiplicityRangeRefMut::Itself(inner),
        }
    }
}
impl<'a> MultiplicityRangeRefMut<'a> {
    pub fn as_ref(self) -> MultiplicityRangeRef<'a> {
        match self {
            MultiplicityRangeRefMut::Itself(inner) => MultiplicityRangeRef::Itself(inner),
        }
    }
}
impl MultiplicityRangeStruct for MultiplicityRange {}
impl MultiplicityRangeStructRefMut for MultiplicityRange {}
impl MultiplicityRangeStructRef for MultiplicityRange {}
impl<'a> MultiplicityRangeStructRefMut for MultiplicityRangeRefMut<'a> {}
impl<'a> MultiplicityRangeStructRef for MultiplicityRangeRefMut<'a> {}
impl<'a> MultiplicityRangeStructRef for MultiplicityRangeRef<'a> {}
impl MultiplicityStruct for MultiplicityRange {}
impl MultiplicityStructRefMut for MultiplicityRange {}
impl MultiplicityStructRef for MultiplicityRange {}
impl<'a> MultiplicityStructRefMut for MultiplicityRangeRefMut<'a> {}
impl<'a> MultiplicityStructRef for MultiplicityRangeRefMut<'a> {}
impl<'a> MultiplicityStructRef for MultiplicityRangeRef<'a> {}
impl FeatureStruct for MultiplicityRange {
    fn is_unique(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_constant(),
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
            MultiplicityRange::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for MultiplicityRange {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_constant_ref_mut(),
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
            MultiplicityRange::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for MultiplicityRange {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_constant_ref(),
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
            MultiplicityRange::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for MultiplicityRangeRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            MultiplicityRangeRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for MultiplicityRangeRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_constant_ref(),
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
            MultiplicityRangeRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for MultiplicityRangeRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_constant_ref(),
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
            MultiplicityRangeRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for MultiplicityRange {
    fn is_abstract(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for MultiplicityRange {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for MultiplicityRange {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for MultiplicityRangeRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for MultiplicityRangeRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for MultiplicityRangeRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for MultiplicityRange {}
impl NamespaceStructRefMut for MultiplicityRange {}
impl NamespaceStructRef for MultiplicityRange {}
impl<'a> NamespaceStructRefMut for MultiplicityRangeRefMut<'a> {}
impl<'a> NamespaceStructRef for MultiplicityRangeRefMut<'a> {}
impl<'a> NamespaceStructRef for MultiplicityRangeRef<'a> {}
impl ElementStruct for MultiplicityRange {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRange::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRange::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            MultiplicityRange::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            MultiplicityRange::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            MultiplicityRange::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            MultiplicityRange::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for MultiplicityRange {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRange::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MultiplicityRange::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MultiplicityRange::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MultiplicityRange::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MultiplicityRange::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MultiplicityRange::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for MultiplicityRange {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRange::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRange::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MultiplicityRange::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MultiplicityRange::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRange::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRange::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MultiplicityRange::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for MultiplicityRangeRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for MultiplicityRangeRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for MultiplicityRangeRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRangeRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            MultiplicityRangeRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            MultiplicityRangeRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            MultiplicityRangeRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRangeRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            MultiplicityRangeRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            MultiplicityRangeRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl MultiplicityRangeUpcast for MultiplicityRange {
    fn into_multiplicity_range(self) -> MultiplicityRange {
        self
    }
}
impl<'a> MultiplicityRangeUpcastRefMut<'a> for MultiplicityRangeRefMut<'a> {
    fn as_multiplicity_range_ref_mut(self) -> MultiplicityRangeRefMut<'a> {
        self
    }
}
impl<'a> MultiplicityRangeUpcastRef<'a> for MultiplicityRangeRef<'a> {
    fn as_multiplicity_range_ref(self) -> MultiplicityRangeRef<'a> {
        self
    }
}
impl MultiplicityUpcast for MultiplicityRange {
    fn into_multiplicity(self) -> Multiplicity {
        Multiplicity::MultiplicityRange(self).into_multiplicity()
    }
}
impl<'a> MultiplicityUpcastRefMut<'a> for MultiplicityRangeRefMut<'a> {
    fn as_multiplicity_ref_mut(self) -> MultiplicityRefMut<'a> {
        MultiplicityRefMut::MultiplicityRange(self).as_multiplicity_ref_mut()
    }
}
impl<'a> MultiplicityUpcastRef<'a> for MultiplicityRangeRef<'a> {
    fn as_multiplicity_ref(self) -> MultiplicityRef<'a> {
        MultiplicityRef::MultiplicityRange(self).as_multiplicity_ref()
    }
}
impl FeatureUpcast for MultiplicityRange {
    fn into_feature(self) -> Feature {
        Multiplicity::MultiplicityRange(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for MultiplicityRangeRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        MultiplicityRefMut::MultiplicityRange(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for MultiplicityRangeRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        MultiplicityRef::MultiplicityRange(self).as_feature_ref()
    }
}
impl TypeUpcast for MultiplicityRange {
    fn into_type_(self) -> Type {
        Multiplicity::MultiplicityRange(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for MultiplicityRangeRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        MultiplicityRefMut::MultiplicityRange(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for MultiplicityRangeRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        MultiplicityRef::MultiplicityRange(self).as_type_ref()
    }
}
impl NamespaceUpcast for MultiplicityRange {
    fn into_namespace(self) -> Namespace {
        Multiplicity::MultiplicityRange(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for MultiplicityRangeRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        MultiplicityRefMut::MultiplicityRange(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for MultiplicityRangeRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        MultiplicityRef::MultiplicityRange(self).as_namespace_ref()
    }
}
impl ElementUpcast for MultiplicityRange {
    fn into_element(self) -> Element {
        Multiplicity::MultiplicityRange(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for MultiplicityRangeRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        MultiplicityRefMut::MultiplicityRange(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for MultiplicityRangeRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        MultiplicityRef::MultiplicityRange(self).as_element_ref()
    }
}
pub trait MultiplicityRangeDowncast {}
pub trait MultiplicityRangeDowncastRefMut<'a> {}
pub trait MultiplicityRangeDowncastRef<'a> {}
impl MultiplicityRangeDowncast for MultiplicityRange {}
impl<'a> MultiplicityRangeDowncastRefMut<'a> for MultiplicityRangeRefMut<'a> {}
impl<'a> MultiplicityRangeDowncastRef<'a> for MultiplicityRangeRef<'a> {}
pub trait MultiplicityRangeMethodsDescendants
where
    Self: DescendantOf<MultiplicityRange>,
    Self::Via: MultiplicityRangeMethods,
    Self: Sized,
{}
pub trait MultiplicityRangeMethods: Sized {}
impl<T: MultiplicityRangeMethodsDescendants> MultiplicityRangeMethods for T
where
    T::Via: MultiplicityRangeMethods,
{}
impl DescendantOf<Multiplicity> for MultiplicityRange {
    type Via = Multiplicity;
    fn into_via(self) -> Self::Via {
        self.into_multiplicity()
    }
}
impl MultiplicityMethodsDescendants for MultiplicityRange {}
impl DescendantOf<Feature> for MultiplicityRange {
    type Via = Multiplicity;
    fn into_via(self) -> Self::Via {
        self.into_multiplicity()
    }
}
impl FeatureMethodsDescendants for MultiplicityRange {}
impl DescendantOf<Type> for MultiplicityRange {
    type Via = Multiplicity;
    fn into_via(self) -> Self::Via {
        self.into_multiplicity()
    }
}
impl TypeMethodsDescendants for MultiplicityRange {}
impl DescendantOf<Namespace> for MultiplicityRange {
    type Via = Multiplicity;
    fn into_via(self) -> Self::Via {
        self.into_multiplicity()
    }
}
impl NamespaceMethodsDescendants for MultiplicityRange {}
impl DescendantOf<Element> for MultiplicityRange {
    type Via = Multiplicity;
    fn into_via(self) -> Self::Via {
        self.into_multiplicity()
    }
}
impl ElementMethodsDescendants for MultiplicityRange {}
pub trait MultiplicityRangeRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<MultiplicityRangeRefMut<'a>>,
    Self::Via: MultiplicityRangeRefMutMethods,
    Self: Sized,
{}
pub trait MultiplicityRangeRefMutMethods: Sized {}
impl<'a, T: MultiplicityRangeRefMutMethodsDescendants<'a>> MultiplicityRangeRefMutMethods
for T
where
    T::Via: MultiplicityRangeRefMutMethods,
{}
impl<'a> DescendantOf<MultiplicityRefMut<'a>> for MultiplicityRangeRefMut<'a> {
    type Via = MultiplicityRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref_mut()
    }
}
impl<'a> MultiplicityRefMutMethodsDescendants<'a> for MultiplicityRangeRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for MultiplicityRangeRefMut<'a> {
    type Via = MultiplicityRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for MultiplicityRangeRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for MultiplicityRangeRefMut<'a> {
    type Via = MultiplicityRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for MultiplicityRangeRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for MultiplicityRangeRefMut<'a> {
    type Via = MultiplicityRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for MultiplicityRangeRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for MultiplicityRangeRefMut<'a> {
    type Via = MultiplicityRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for MultiplicityRangeRefMut<'a> {}
pub trait MultiplicityRangeRefMethodsDescendants<'a>
where
    Self: DescendantOf<MultiplicityRangeRef<'a>>,
    Self::Via: MultiplicityRangeRefMethods,
    Self: Sized,
{
    fn has_bounds_impl(self, lower: i64, upper: u64) -> bool {
        self.into_via().has_bounds(lower, upper)
    }
    fn value_of_impl(
        self,
        bound: Option<std::rc::Rc<std::cell::RefCell<super::expression::Expression>>>,
    ) -> Option<u64> {
        self.into_via().value_of(bound)
    }
}
pub trait MultiplicityRangeRefMethods: Sized {
    fn has_bounds(self, lower: i64, upper: u64) -> bool;
    fn value_of(
        self,
        bound: Option<std::rc::Rc<std::cell::RefCell<super::expression::Expression>>>,
    ) -> Option<u64>;
}
impl<'a, T: MultiplicityRangeRefMethodsDescendants<'a>> MultiplicityRangeRefMethods for T
where
    T::Via: MultiplicityRangeRefMethods,
{
    fn has_bounds(self, lower: i64, upper: u64) -> bool {
        MultiplicityRangeRefMethodsDescendants::has_bounds_impl(self, lower, upper)
    }
    fn value_of(
        self,
        bound: Option<std::rc::Rc<std::cell::RefCell<super::expression::Expression>>>,
    ) -> Option<u64> {
        MultiplicityRangeRefMethodsDescendants::value_of_impl(self, bound)
    }
}
impl<'a> DescendantOf<MultiplicityRef<'a>> for MultiplicityRangeRef<'a> {
    type Via = MultiplicityRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref()
    }
}
impl<'a> MultiplicityRefMethodsDescendants<'a> for MultiplicityRangeRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for MultiplicityRangeRef<'a> {
    type Via = MultiplicityRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for MultiplicityRangeRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for MultiplicityRangeRef<'a> {
    type Via = MultiplicityRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for MultiplicityRangeRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for MultiplicityRangeRef<'a> {
    type Via = MultiplicityRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for MultiplicityRangeRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for MultiplicityRangeRef<'a> {
    type Via = MultiplicityRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_multiplicity_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for MultiplicityRangeRef<'a> {}

