#![allow(unused)]
use super::utils::DescendantOf;
use super::literal_expression::{
    LiteralExpression, LiteralExpressionRefMut, LiteralExpressionRef,
    LiteralExpressionStruct, LiteralExpressionStructRefMut, LiteralExpressionStructRef,
    LiteralExpressionInner, LiteralExpressionUpcast, LiteralExpressionUpcastRefMut,
    LiteralExpressionUpcastRef, LiteralExpressionMethodsDescendants,
    LiteralExpressionRefMutMethodsDescendants, LiteralExpressionRefMethodsDescendants,
};
use super::expression::{
    Expression, ExpressionRefMut, ExpressionRef, ExpressionStruct,
    ExpressionStructRefMut, ExpressionStructRef, ExpressionInner, ExpressionUpcast,
    ExpressionUpcastRefMut, ExpressionUpcastRef, ExpressionMethodsDescendants,
    ExpressionRefMutMethodsDescendants, ExpressionRefMethodsDescendants,
};
use super::step::{
    Step, StepRefMut, StepRef, StepStruct, StepStructRefMut, StepStructRef, StepInner,
    StepUpcast, StepUpcastRefMut, StepUpcastRef, StepMethodsDescendants,
    StepRefMutMethodsDescendants, StepRefMethodsDescendants,
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
pub struct LiteralStringInner {
    pub(super) sup_literal_expression: LiteralExpressionInner,
    pub(super) value: String,
}
pub trait LiteralStringStruct
where
    Self: LiteralStringStructRefMut,
    Self: LiteralStringStructRef,
    Self: LiteralExpressionStruct,
{
    fn value(self) -> String;
}
pub trait LiteralStringStructRefMut
where
    Self: LiteralStringStructRef,
    Self: LiteralExpressionStructRefMut,
{
    fn value_ref_mut(&mut self) -> &mut String;
}
pub trait LiteralStringStructRef
where
    Self: LiteralExpressionStructRef,
{
    fn value_ref(&self) -> &String;
}
pub trait LiteralStringUpcast: LiteralStringStruct {
    fn into_literal_string(self) -> LiteralString;
}
pub trait LiteralStringUpcastRefMut<'a>: LiteralStringStructRefMut {
    fn as_literal_string_ref_mut(self) -> LiteralStringRefMut<'a>;
}
pub trait LiteralStringUpcastRef<'a>: LiteralStringStructRef {
    fn as_literal_string_ref(self) -> LiteralStringRef<'a>;
}
impl LiteralStringStruct for LiteralStringInner {
    fn value(self) -> String {
        self.value
    }
}
impl LiteralStringStructRefMut for LiteralStringInner {
    fn value_ref_mut(&mut self) -> &mut String {
        &mut self.value
    }
}
impl LiteralStringStructRef for LiteralStringInner {
    fn value_ref(&self) -> &String {
        &self.value
    }
}
impl LiteralExpressionStruct for LiteralStringInner {}
impl LiteralExpressionStructRefMut for LiteralStringInner {}
impl LiteralExpressionStructRef for LiteralStringInner {}
impl ExpressionStruct for LiteralStringInner {}
impl ExpressionStructRefMut for LiteralStringInner {}
impl ExpressionStructRef for LiteralStringInner {}
impl StepStruct for LiteralStringInner {}
impl StepStructRefMut for LiteralStringInner {}
impl StepStructRef for LiteralStringInner {}
impl FeatureStruct for LiteralStringInner {
    fn is_unique(self) -> bool {
        self.sup_literal_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_literal_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_literal_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_literal_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_literal_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_literal_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_literal_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_literal_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_literal_expression.direction()
    }
}
impl FeatureStructRefMut for LiteralStringInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_literal_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for LiteralStringInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_literal_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_literal_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_literal_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_literal_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_literal_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_literal_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_literal_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_literal_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_literal_expression.direction_ref()
    }
}
impl TypeStruct for LiteralStringInner {
    fn is_abstract(self) -> bool {
        self.sup_literal_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_literal_expression.is_sufficient()
    }
}
impl TypeStructRefMut for LiteralStringInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for LiteralStringInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_literal_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_literal_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for LiteralStringInner {}
impl NamespaceStructRefMut for LiteralStringInner {}
impl NamespaceStructRef for LiteralStringInner {}
impl ElementStruct for LiteralStringInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_literal_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_literal_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_literal_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_literal_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_literal_expression.is_implied_included()
    }
}
impl ElementStructRefMut for LiteralStringInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_literal_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_literal_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_literal_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_literal_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_literal_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for LiteralStringInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_literal_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_literal_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_literal_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_literal_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_literal_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_literal_expression.is_implied_included_ref()
    }
}
pub enum LiteralString {
    Itself(LiteralStringInner),
}
pub enum LiteralStringRefMut<'a> {
    Itself(&'a mut LiteralStringInner),
}
pub enum LiteralStringRef<'a> {
    Itself(&'a LiteralStringInner),
}
impl LiteralString {
    pub fn as_ref(&self) -> LiteralStringRef {
        match self {
            LiteralString::Itself(inner) => LiteralStringRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> LiteralStringRefMut {
        match self {
            LiteralString::Itself(inner) => LiteralStringRefMut::Itself(inner),
        }
    }
}
impl<'a> LiteralStringRefMut<'a> {
    pub fn as_ref(self) -> LiteralStringRef<'a> {
        match self {
            LiteralStringRefMut::Itself(inner) => LiteralStringRef::Itself(inner),
        }
    }
}
impl LiteralStringStruct for LiteralString {
    fn value(self) -> String {
        match self {
            LiteralString::Itself(x) => x.value(),
        }
    }
}
impl LiteralStringStructRefMut for LiteralString {
    fn value_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralString::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl LiteralStringStructRef for LiteralString {
    fn value_ref(&self) -> &String {
        match self {
            LiteralString::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralStringStructRefMut for LiteralStringRefMut<'a> {
    fn value_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralStringRefMut::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl<'a> LiteralStringStructRef for LiteralStringRefMut<'a> {
    fn value_ref(&self) -> &String {
        match self {
            LiteralStringRefMut::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralStringStructRef for LiteralStringRef<'a> {
    fn value_ref(&self) -> &String {
        match self {
            LiteralStringRef::Itself(x) => x.value_ref(),
        }
    }
}
impl LiteralExpressionStruct for LiteralString {}
impl LiteralExpressionStructRefMut for LiteralString {}
impl LiteralExpressionStructRef for LiteralString {}
impl<'a> LiteralExpressionStructRefMut for LiteralStringRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralStringRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralStringRef<'a> {}
impl ExpressionStruct for LiteralString {}
impl ExpressionStructRefMut for LiteralString {}
impl ExpressionStructRef for LiteralString {}
impl<'a> ExpressionStructRefMut for LiteralStringRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralStringRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralStringRef<'a> {}
impl StepStruct for LiteralString {}
impl StepStructRefMut for LiteralString {}
impl StepStructRef for LiteralString {}
impl<'a> StepStructRefMut for LiteralStringRefMut<'a> {}
impl<'a> StepStructRef for LiteralStringRefMut<'a> {}
impl<'a> StepStructRef for LiteralStringRef<'a> {}
impl FeatureStruct for LiteralString {
    fn is_unique(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_constant(),
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
            LiteralString::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for LiteralString {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralString::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for LiteralString {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_constant_ref(),
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
            LiteralString::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for LiteralStringRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralStringRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralStringRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_constant_ref(),
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
            LiteralStringRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralStringRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_constant_ref(),
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
            LiteralStringRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for LiteralString {
    fn is_abstract(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for LiteralString {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for LiteralString {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for LiteralStringRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for LiteralStringRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for LiteralStringRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for LiteralString {}
impl NamespaceStructRefMut for LiteralString {}
impl NamespaceStructRef for LiteralString {}
impl<'a> NamespaceStructRefMut for LiteralStringRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralStringRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralStringRef<'a> {}
impl ElementStruct for LiteralString {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralString::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralString::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LiteralString::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LiteralString::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LiteralString::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LiteralString::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LiteralString::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LiteralString {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralString::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralString::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralString::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralString::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralString::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralString::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralString::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LiteralString {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralString::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralString::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralString::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralString::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralString::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralString::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralString::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LiteralStringRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralStringRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralStringRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralStringRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralStringRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralStringRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralStringRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for LiteralStringRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralStringRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralStringRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralStringRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralStringRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralStringRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralStringRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralStringRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LiteralStringRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralStringRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralStringRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralStringRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralStringRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralStringRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralStringRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralStringRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl LiteralStringUpcast for LiteralString {
    fn into_literal_string(self) -> LiteralString {
        self
    }
}
impl<'a> LiteralStringUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_literal_string_ref_mut(self) -> LiteralStringRefMut<'a> {
        self
    }
}
impl<'a> LiteralStringUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_literal_string_ref(self) -> LiteralStringRef<'a> {
        self
    }
}
impl LiteralExpressionUpcast for LiteralString {
    fn into_literal_expression(self) -> LiteralExpression {
        LiteralExpression::LiteralString(self).into_literal_expression()
    }
}
impl<'a> LiteralExpressionUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_literal_expression_ref()
    }
}
impl ExpressionUpcast for LiteralString {
    fn into_expression(self) -> Expression {
        LiteralExpression::LiteralString(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_expression_ref()
    }
}
impl StepUpcast for LiteralString {
    fn into_step(self) -> Step {
        LiteralExpression::LiteralString(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_step_ref()
    }
}
impl FeatureUpcast for LiteralString {
    fn into_feature(self) -> Feature {
        LiteralExpression::LiteralString(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_feature_ref()
    }
}
impl TypeUpcast for LiteralString {
    fn into_type_(self) -> Type {
        LiteralExpression::LiteralString(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_type_ref()
    }
}
impl NamespaceUpcast for LiteralString {
    fn into_namespace(self) -> Namespace {
        LiteralExpression::LiteralString(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_namespace_ref()
    }
}
impl ElementUpcast for LiteralString {
    fn into_element(self) -> Element {
        LiteralExpression::LiteralString(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LiteralStringRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        LiteralExpressionRefMut::LiteralString(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LiteralStringRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        LiteralExpressionRef::LiteralString(self).as_element_ref()
    }
}
pub trait LiteralStringDowncast {}
pub trait LiteralStringDowncastRefMut<'a> {}
pub trait LiteralStringDowncastRef<'a> {}
impl LiteralStringDowncast for LiteralString {}
impl<'a> LiteralStringDowncastRefMut<'a> for LiteralStringRefMut<'a> {}
impl<'a> LiteralStringDowncastRef<'a> for LiteralStringRef<'a> {}
pub trait LiteralStringMethodsDescendants
where
    Self: DescendantOf<LiteralString>,
    Self::Via: LiteralStringMethods,
    Self: Sized,
{}
pub trait LiteralStringMethods: Sized {}
impl<T: LiteralStringMethodsDescendants> LiteralStringMethods for T
where
    T::Via: LiteralStringMethods,
{}
impl DescendantOf<LiteralExpression> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl LiteralExpressionMethodsDescendants for LiteralString {}
impl DescendantOf<Expression> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ExpressionMethodsDescendants for LiteralString {}
impl DescendantOf<Step> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl StepMethodsDescendants for LiteralString {}
impl DescendantOf<Feature> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl FeatureMethodsDescendants for LiteralString {}
impl DescendantOf<Type> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl TypeMethodsDescendants for LiteralString {}
impl DescendantOf<Namespace> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl NamespaceMethodsDescendants for LiteralString {}
impl DescendantOf<Element> for LiteralString {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ElementMethodsDescendants for LiteralString {}
pub trait LiteralStringRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralStringRefMut<'a>>,
    Self::Via: LiteralStringRefMutMethods,
    Self: Sized,
{}
pub trait LiteralStringRefMutMethods: Sized {}
impl<'a, T: LiteralStringRefMutMethodsDescendants<'a>> LiteralStringRefMutMethods for T
where
    T::Via: LiteralStringRefMutMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LiteralStringRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LiteralStringRefMut<'a> {}
pub trait LiteralStringRefMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralStringRef<'a>>,
    Self::Via: LiteralStringRefMethods,
    Self: Sized,
{}
pub trait LiteralStringRefMethods: Sized {}
impl<'a, T: LiteralStringRefMethodsDescendants<'a>> LiteralStringRefMethods for T
where
    T::Via: LiteralStringRefMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> LiteralExpressionRefMethodsDescendants<'a> for LiteralStringRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for LiteralStringRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for LiteralStringRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for LiteralStringRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for LiteralStringRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LiteralStringRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LiteralStringRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LiteralStringRef<'a> {}

