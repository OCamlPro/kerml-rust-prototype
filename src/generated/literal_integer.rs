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
pub struct LiteralIntegerInner {
    pub(super) sup_literal_expression: LiteralExpressionInner,
    pub(super) value: i64,
}
pub trait LiteralIntegerStruct
where
    Self: LiteralIntegerStructRefMut,
    Self: LiteralIntegerStructRef,
    Self: LiteralExpressionStruct,
{
    fn value(self) -> i64;
}
pub trait LiteralIntegerStructRefMut
where
    Self: LiteralIntegerStructRef,
    Self: LiteralExpressionStructRefMut,
{
    fn value_ref_mut(&mut self) -> &mut i64;
}
pub trait LiteralIntegerStructRef
where
    Self: LiteralExpressionStructRef,
{
    fn value_ref(&self) -> &i64;
}
pub trait LiteralIntegerUpcast: LiteralIntegerStruct {
    fn into_literal_integer(self) -> LiteralInteger;
}
pub trait LiteralIntegerUpcastRefMut<'a>: LiteralIntegerStructRefMut {
    fn as_literal_integer_ref_mut(self) -> LiteralIntegerRefMut<'a>;
}
pub trait LiteralIntegerUpcastRef<'a>: LiteralIntegerStructRef {
    fn as_literal_integer_ref(self) -> LiteralIntegerRef<'a>;
}
impl LiteralIntegerStruct for LiteralIntegerInner {
    fn value(self) -> i64 {
        self.value
    }
}
impl LiteralIntegerStructRefMut for LiteralIntegerInner {
    fn value_ref_mut(&mut self) -> &mut i64 {
        &mut self.value
    }
}
impl LiteralIntegerStructRef for LiteralIntegerInner {
    fn value_ref(&self) -> &i64 {
        &self.value
    }
}
impl LiteralExpressionStruct for LiteralIntegerInner {}
impl LiteralExpressionStructRefMut for LiteralIntegerInner {}
impl LiteralExpressionStructRef for LiteralIntegerInner {}
impl ExpressionStruct for LiteralIntegerInner {}
impl ExpressionStructRefMut for LiteralIntegerInner {}
impl ExpressionStructRef for LiteralIntegerInner {}
impl StepStruct for LiteralIntegerInner {}
impl StepStructRefMut for LiteralIntegerInner {}
impl StepStructRef for LiteralIntegerInner {}
impl FeatureStruct for LiteralIntegerInner {
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
impl FeatureStructRefMut for LiteralIntegerInner {
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
impl FeatureStructRef for LiteralIntegerInner {
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
impl TypeStruct for LiteralIntegerInner {
    fn is_abstract(self) -> bool {
        self.sup_literal_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_literal_expression.is_sufficient()
    }
}
impl TypeStructRefMut for LiteralIntegerInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for LiteralIntegerInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_literal_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_literal_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for LiteralIntegerInner {}
impl NamespaceStructRefMut for LiteralIntegerInner {}
impl NamespaceStructRef for LiteralIntegerInner {}
impl ElementStruct for LiteralIntegerInner {
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
impl ElementStructRefMut for LiteralIntegerInner {
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
impl ElementStructRef for LiteralIntegerInner {
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
pub enum LiteralInteger {
    Itself(LiteralIntegerInner),
}
pub enum LiteralIntegerRefMut<'a> {
    Itself(&'a mut LiteralIntegerInner),
}
pub enum LiteralIntegerRef<'a> {
    Itself(&'a LiteralIntegerInner),
}
impl LiteralInteger {
    pub fn as_ref(&self) -> LiteralIntegerRef {
        match self {
            LiteralInteger::Itself(inner) => LiteralIntegerRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> LiteralIntegerRefMut {
        match self {
            LiteralInteger::Itself(inner) => LiteralIntegerRefMut::Itself(inner),
        }
    }
}
impl<'a> LiteralIntegerRefMut<'a> {
    pub fn as_ref(self) -> LiteralIntegerRef<'a> {
        match self {
            LiteralIntegerRefMut::Itself(inner) => LiteralIntegerRef::Itself(inner),
        }
    }
}
impl LiteralIntegerStruct for LiteralInteger {
    fn value(self) -> i64 {
        match self {
            LiteralInteger::Itself(x) => x.value(),
        }
    }
}
impl LiteralIntegerStructRefMut for LiteralInteger {
    fn value_ref_mut(&mut self) -> &mut i64 {
        match self {
            LiteralInteger::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl LiteralIntegerStructRef for LiteralInteger {
    fn value_ref(&self) -> &i64 {
        match self {
            LiteralInteger::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralIntegerStructRefMut for LiteralIntegerRefMut<'a> {
    fn value_ref_mut(&mut self) -> &mut i64 {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl<'a> LiteralIntegerStructRef for LiteralIntegerRefMut<'a> {
    fn value_ref(&self) -> &i64 {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralIntegerStructRef for LiteralIntegerRef<'a> {
    fn value_ref(&self) -> &i64 {
        match self {
            LiteralIntegerRef::Itself(x) => x.value_ref(),
        }
    }
}
impl LiteralExpressionStruct for LiteralInteger {}
impl LiteralExpressionStructRefMut for LiteralInteger {}
impl LiteralExpressionStructRef for LiteralInteger {}
impl<'a> LiteralExpressionStructRefMut for LiteralIntegerRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralIntegerRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralIntegerRef<'a> {}
impl ExpressionStruct for LiteralInteger {}
impl ExpressionStructRefMut for LiteralInteger {}
impl ExpressionStructRef for LiteralInteger {}
impl<'a> ExpressionStructRefMut for LiteralIntegerRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralIntegerRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralIntegerRef<'a> {}
impl StepStruct for LiteralInteger {}
impl StepStructRefMut for LiteralInteger {}
impl StepStructRef for LiteralInteger {}
impl<'a> StepStructRefMut for LiteralIntegerRefMut<'a> {}
impl<'a> StepStructRef for LiteralIntegerRefMut<'a> {}
impl<'a> StepStructRef for LiteralIntegerRef<'a> {}
impl FeatureStruct for LiteralInteger {
    fn is_unique(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_constant(),
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
            LiteralInteger::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for LiteralInteger {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralInteger::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for LiteralInteger {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_constant_ref(),
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
            LiteralInteger::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for LiteralIntegerRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralIntegerRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralIntegerRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_constant_ref(),
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
            LiteralIntegerRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralIntegerRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_constant_ref(),
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
            LiteralIntegerRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for LiteralInteger {
    fn is_abstract(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for LiteralInteger {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for LiteralInteger {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for LiteralIntegerRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for LiteralIntegerRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for LiteralIntegerRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for LiteralInteger {}
impl NamespaceStructRefMut for LiteralInteger {}
impl NamespaceStructRef for LiteralInteger {}
impl<'a> NamespaceStructRefMut for LiteralIntegerRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralIntegerRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralIntegerRef<'a> {}
impl ElementStruct for LiteralInteger {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInteger::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInteger::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LiteralInteger::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LiteralInteger::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LiteralInteger::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LiteralInteger::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LiteralInteger::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LiteralInteger {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInteger::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralInteger::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralInteger::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralInteger::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralInteger::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralInteger::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralInteger::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LiteralInteger {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInteger::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralInteger::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralInteger::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralInteger::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInteger::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralInteger::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralInteger::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LiteralIntegerRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for LiteralIntegerRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralIntegerRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LiteralIntegerRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralIntegerRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralIntegerRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralIntegerRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralIntegerRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralIntegerRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralIntegerRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralIntegerRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl LiteralIntegerUpcast for LiteralInteger {
    fn into_literal_integer(self) -> LiteralInteger {
        self
    }
}
impl<'a> LiteralIntegerUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_literal_integer_ref_mut(self) -> LiteralIntegerRefMut<'a> {
        self
    }
}
impl<'a> LiteralIntegerUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_literal_integer_ref(self) -> LiteralIntegerRef<'a> {
        self
    }
}
impl LiteralExpressionUpcast for LiteralInteger {
    fn into_literal_expression(self) -> LiteralExpression {
        LiteralExpression::LiteralInteger(self).into_literal_expression()
    }
}
impl<'a> LiteralExpressionUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_literal_expression_ref()
    }
}
impl ExpressionUpcast for LiteralInteger {
    fn into_expression(self) -> Expression {
        LiteralExpression::LiteralInteger(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_expression_ref()
    }
}
impl StepUpcast for LiteralInteger {
    fn into_step(self) -> Step {
        LiteralExpression::LiteralInteger(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_step_ref()
    }
}
impl FeatureUpcast for LiteralInteger {
    fn into_feature(self) -> Feature {
        LiteralExpression::LiteralInteger(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_feature_ref()
    }
}
impl TypeUpcast for LiteralInteger {
    fn into_type_(self) -> Type {
        LiteralExpression::LiteralInteger(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_type_ref()
    }
}
impl NamespaceUpcast for LiteralInteger {
    fn into_namespace(self) -> Namespace {
        LiteralExpression::LiteralInteger(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_namespace_ref()
    }
}
impl ElementUpcast for LiteralInteger {
    fn into_element(self) -> Element {
        LiteralExpression::LiteralInteger(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LiteralIntegerRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        LiteralExpressionRefMut::LiteralInteger(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LiteralIntegerRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        LiteralExpressionRef::LiteralInteger(self).as_element_ref()
    }
}
pub trait LiteralIntegerDowncast {}
pub trait LiteralIntegerDowncastRefMut<'a> {}
pub trait LiteralIntegerDowncastRef<'a> {}
impl LiteralIntegerDowncast for LiteralInteger {}
impl<'a> LiteralIntegerDowncastRefMut<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> LiteralIntegerDowncastRef<'a> for LiteralIntegerRef<'a> {}
pub trait LiteralIntegerMethodsDescendants
where
    Self: DescendantOf<LiteralInteger>,
    Self::Via: LiteralIntegerMethods,
    Self: Sized,
{}
pub trait LiteralIntegerMethods: Sized {}
impl<T: LiteralIntegerMethodsDescendants> LiteralIntegerMethods for T
where
    T::Via: LiteralIntegerMethods,
{}
impl DescendantOf<LiteralExpression> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl LiteralExpressionMethodsDescendants for LiteralInteger {}
impl DescendantOf<Expression> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ExpressionMethodsDescendants for LiteralInteger {}
impl DescendantOf<Step> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl StepMethodsDescendants for LiteralInteger {}
impl DescendantOf<Feature> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl FeatureMethodsDescendants for LiteralInteger {}
impl DescendantOf<Type> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl TypeMethodsDescendants for LiteralInteger {}
impl DescendantOf<Namespace> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl NamespaceMethodsDescendants for LiteralInteger {}
impl DescendantOf<Element> for LiteralInteger {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ElementMethodsDescendants for LiteralInteger {}
pub trait LiteralIntegerRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralIntegerRefMut<'a>>,
    Self::Via: LiteralIntegerRefMutMethods,
    Self: Sized,
{}
pub trait LiteralIntegerRefMutMethods: Sized {}
impl<'a, T: LiteralIntegerRefMutMethodsDescendants<'a>> LiteralIntegerRefMutMethods for T
where
    T::Via: LiteralIntegerRefMutMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LiteralIntegerRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LiteralIntegerRefMut<'a> {}
pub trait LiteralIntegerRefMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralIntegerRef<'a>>,
    Self::Via: LiteralIntegerRefMethods,
    Self: Sized,
{}
pub trait LiteralIntegerRefMethods: Sized {}
impl<'a, T: LiteralIntegerRefMethodsDescendants<'a>> LiteralIntegerRefMethods for T
where
    T::Via: LiteralIntegerRefMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> LiteralExpressionRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LiteralIntegerRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LiteralIntegerRef<'a> {}

