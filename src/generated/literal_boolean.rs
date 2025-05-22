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
pub struct LiteralBooleanInner {
    pub(super) sup_literal_expression: LiteralExpressionInner,
    pub(super) value: bool,
}
pub trait LiteralBooleanStruct
where
    Self: LiteralBooleanStructRefMut,
    Self: LiteralBooleanStructRef,
    Self: LiteralExpressionStruct,
{
    fn value(self) -> bool;
}
pub trait LiteralBooleanStructRefMut
where
    Self: LiteralBooleanStructRef,
    Self: LiteralExpressionStructRefMut,
{
    fn value_ref_mut(&mut self) -> &mut bool;
}
pub trait LiteralBooleanStructRef
where
    Self: LiteralExpressionStructRef,
{
    fn value_ref(&self) -> &bool;
}
pub trait LiteralBooleanUpcast: LiteralBooleanStruct {
    fn into_literal_boolean(self) -> LiteralBoolean;
}
pub trait LiteralBooleanUpcastRefMut<'a>: LiteralBooleanStructRefMut {
    fn as_literal_boolean_ref_mut(self) -> LiteralBooleanRefMut<'a>;
}
pub trait LiteralBooleanUpcastRef<'a>: LiteralBooleanStructRef {
    fn as_literal_boolean_ref(self) -> LiteralBooleanRef<'a>;
}
impl LiteralBooleanStruct for LiteralBooleanInner {
    fn value(self) -> bool {
        self.value
    }
}
impl LiteralBooleanStructRefMut for LiteralBooleanInner {
    fn value_ref_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}
impl LiteralBooleanStructRef for LiteralBooleanInner {
    fn value_ref(&self) -> &bool {
        &self.value
    }
}
impl LiteralExpressionStruct for LiteralBooleanInner {}
impl LiteralExpressionStructRefMut for LiteralBooleanInner {}
impl LiteralExpressionStructRef for LiteralBooleanInner {}
impl ExpressionStruct for LiteralBooleanInner {}
impl ExpressionStructRefMut for LiteralBooleanInner {}
impl ExpressionStructRef for LiteralBooleanInner {}
impl StepStruct for LiteralBooleanInner {}
impl StepStructRefMut for LiteralBooleanInner {}
impl StepStructRef for LiteralBooleanInner {}
impl FeatureStruct for LiteralBooleanInner {
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
impl FeatureStructRefMut for LiteralBooleanInner {
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
impl FeatureStructRef for LiteralBooleanInner {
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
impl TypeStruct for LiteralBooleanInner {
    fn is_abstract(self) -> bool {
        self.sup_literal_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_literal_expression.is_sufficient()
    }
}
impl TypeStructRefMut for LiteralBooleanInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for LiteralBooleanInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_literal_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_literal_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for LiteralBooleanInner {}
impl NamespaceStructRefMut for LiteralBooleanInner {}
impl NamespaceStructRef for LiteralBooleanInner {}
impl ElementStruct for LiteralBooleanInner {
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
impl ElementStructRefMut for LiteralBooleanInner {
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
impl ElementStructRef for LiteralBooleanInner {
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
pub enum LiteralBoolean {
    Itself(LiteralBooleanInner),
}
pub enum LiteralBooleanRefMut<'a> {
    Itself(&'a mut LiteralBooleanInner),
}
pub enum LiteralBooleanRef<'a> {
    Itself(&'a LiteralBooleanInner),
}
impl LiteralBoolean {
    pub fn as_ref(&self) -> LiteralBooleanRef {
        match self {
            LiteralBoolean::Itself(inner) => LiteralBooleanRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> LiteralBooleanRefMut {
        match self {
            LiteralBoolean::Itself(inner) => LiteralBooleanRefMut::Itself(inner),
        }
    }
}
impl<'a> LiteralBooleanRefMut<'a> {
    pub fn as_ref(self) -> LiteralBooleanRef<'a> {
        match self {
            LiteralBooleanRefMut::Itself(inner) => LiteralBooleanRef::Itself(inner),
        }
    }
}
impl LiteralBooleanStruct for LiteralBoolean {
    fn value(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.value(),
        }
    }
}
impl LiteralBooleanStructRefMut for LiteralBoolean {
    fn value_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl LiteralBooleanStructRef for LiteralBoolean {
    fn value_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralBooleanStructRefMut for LiteralBooleanRefMut<'a> {
    fn value_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl<'a> LiteralBooleanStructRef for LiteralBooleanRefMut<'a> {
    fn value_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralBooleanStructRef for LiteralBooleanRef<'a> {
    fn value_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.value_ref(),
        }
    }
}
impl LiteralExpressionStruct for LiteralBoolean {}
impl LiteralExpressionStructRefMut for LiteralBoolean {}
impl LiteralExpressionStructRef for LiteralBoolean {}
impl<'a> LiteralExpressionStructRefMut for LiteralBooleanRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralBooleanRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralBooleanRef<'a> {}
impl ExpressionStruct for LiteralBoolean {}
impl ExpressionStructRefMut for LiteralBoolean {}
impl ExpressionStructRef for LiteralBoolean {}
impl<'a> ExpressionStructRefMut for LiteralBooleanRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralBooleanRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralBooleanRef<'a> {}
impl StepStruct for LiteralBoolean {}
impl StepStructRefMut for LiteralBoolean {}
impl StepStructRef for LiteralBoolean {}
impl<'a> StepStructRefMut for LiteralBooleanRefMut<'a> {}
impl<'a> StepStructRef for LiteralBooleanRefMut<'a> {}
impl<'a> StepStructRef for LiteralBooleanRef<'a> {}
impl FeatureStruct for LiteralBoolean {
    fn is_unique(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_constant(),
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
            LiteralBoolean::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for LiteralBoolean {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralBoolean::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for LiteralBoolean {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_constant_ref(),
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
            LiteralBoolean::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for LiteralBooleanRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralBooleanRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralBooleanRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_constant_ref(),
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
            LiteralBooleanRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralBooleanRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_constant_ref(),
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
            LiteralBooleanRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for LiteralBoolean {
    fn is_abstract(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for LiteralBoolean {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for LiteralBoolean {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for LiteralBooleanRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for LiteralBooleanRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for LiteralBooleanRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for LiteralBoolean {}
impl NamespaceStructRefMut for LiteralBoolean {}
impl NamespaceStructRef for LiteralBoolean {}
impl<'a> NamespaceStructRefMut for LiteralBooleanRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralBooleanRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralBooleanRef<'a> {}
impl ElementStruct for LiteralBoolean {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBoolean::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBoolean::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LiteralBoolean::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LiteralBoolean::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LiteralBoolean::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LiteralBoolean::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LiteralBoolean {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBoolean::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralBoolean::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralBoolean::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralBoolean::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralBoolean::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralBoolean::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LiteralBoolean {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBoolean::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBoolean::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralBoolean::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralBoolean::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralBoolean::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralBoolean::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralBoolean::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LiteralBooleanRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for LiteralBooleanRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralBooleanRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LiteralBooleanRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBooleanRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralBooleanRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralBooleanRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralBooleanRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralBooleanRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralBooleanRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralBooleanRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl LiteralBooleanUpcast for LiteralBoolean {
    fn into_literal_boolean(self) -> LiteralBoolean {
        self
    }
}
impl<'a> LiteralBooleanUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_literal_boolean_ref_mut(self) -> LiteralBooleanRefMut<'a> {
        self
    }
}
impl<'a> LiteralBooleanUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_literal_boolean_ref(self) -> LiteralBooleanRef<'a> {
        self
    }
}
impl LiteralExpressionUpcast for LiteralBoolean {
    fn into_literal_expression(self) -> LiteralExpression {
        LiteralExpression::LiteralBoolean(self).into_literal_expression()
    }
}
impl<'a> LiteralExpressionUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_literal_expression_ref()
    }
}
impl ExpressionUpcast for LiteralBoolean {
    fn into_expression(self) -> Expression {
        LiteralExpression::LiteralBoolean(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_expression_ref()
    }
}
impl StepUpcast for LiteralBoolean {
    fn into_step(self) -> Step {
        LiteralExpression::LiteralBoolean(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_step_ref()
    }
}
impl FeatureUpcast for LiteralBoolean {
    fn into_feature(self) -> Feature {
        LiteralExpression::LiteralBoolean(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_feature_ref()
    }
}
impl TypeUpcast for LiteralBoolean {
    fn into_type_(self) -> Type {
        LiteralExpression::LiteralBoolean(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_type_ref()
    }
}
impl NamespaceUpcast for LiteralBoolean {
    fn into_namespace(self) -> Namespace {
        LiteralExpression::LiteralBoolean(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_namespace_ref()
    }
}
impl ElementUpcast for LiteralBoolean {
    fn into_element(self) -> Element {
        LiteralExpression::LiteralBoolean(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LiteralBooleanRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        LiteralExpressionRefMut::LiteralBoolean(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LiteralBooleanRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        LiteralExpressionRef::LiteralBoolean(self).as_element_ref()
    }
}
pub trait LiteralBooleanDowncast {}
pub trait LiteralBooleanDowncastRefMut<'a> {}
pub trait LiteralBooleanDowncastRef<'a> {}
impl LiteralBooleanDowncast for LiteralBoolean {}
impl<'a> LiteralBooleanDowncastRefMut<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> LiteralBooleanDowncastRef<'a> for LiteralBooleanRef<'a> {}
pub trait LiteralBooleanMethodsDescendants
where
    Self: DescendantOf<LiteralBoolean>,
    Self::Via: LiteralBooleanMethods,
    Self: Sized,
{}
pub trait LiteralBooleanMethods: Sized {}
impl<T: LiteralBooleanMethodsDescendants> LiteralBooleanMethods for T
where
    T::Via: LiteralBooleanMethods,
{}
impl DescendantOf<LiteralExpression> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl LiteralExpressionMethodsDescendants for LiteralBoolean {}
impl DescendantOf<Expression> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ExpressionMethodsDescendants for LiteralBoolean {}
impl DescendantOf<Step> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl StepMethodsDescendants for LiteralBoolean {}
impl DescendantOf<Feature> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl FeatureMethodsDescendants for LiteralBoolean {}
impl DescendantOf<Type> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl TypeMethodsDescendants for LiteralBoolean {}
impl DescendantOf<Namespace> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl NamespaceMethodsDescendants for LiteralBoolean {}
impl DescendantOf<Element> for LiteralBoolean {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ElementMethodsDescendants for LiteralBoolean {}
pub trait LiteralBooleanRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralBooleanRefMut<'a>>,
    Self::Via: LiteralBooleanRefMutMethods,
    Self: Sized,
{}
pub trait LiteralBooleanRefMutMethods: Sized {}
impl<'a, T: LiteralBooleanRefMutMethodsDescendants<'a>> LiteralBooleanRefMutMethods for T
where
    T::Via: LiteralBooleanRefMutMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LiteralBooleanRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LiteralBooleanRefMut<'a> {}
pub trait LiteralBooleanRefMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralBooleanRef<'a>>,
    Self::Via: LiteralBooleanRefMethods,
    Self: Sized,
{}
pub trait LiteralBooleanRefMethods: Sized {}
impl<'a, T: LiteralBooleanRefMethodsDescendants<'a>> LiteralBooleanRefMethods for T
where
    T::Via: LiteralBooleanRefMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> LiteralExpressionRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LiteralBooleanRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LiteralBooleanRef<'a> {}

