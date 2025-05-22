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
pub struct LiteralRationalInner {
    pub(super) sup_literal_expression: LiteralExpressionInner,
    pub(super) value: f64,
}
pub trait LiteralRationalStruct
where
    Self: LiteralRationalStructRefMut,
    Self: LiteralRationalStructRef,
    Self: LiteralExpressionStruct,
{
    fn value(self) -> f64;
}
pub trait LiteralRationalStructRefMut
where
    Self: LiteralRationalStructRef,
    Self: LiteralExpressionStructRefMut,
{
    fn value_ref_mut(&mut self) -> &mut f64;
}
pub trait LiteralRationalStructRef
where
    Self: LiteralExpressionStructRef,
{
    fn value_ref(&self) -> &f64;
}
pub trait LiteralRationalUpcast: LiteralRationalStruct {
    fn into_literal_rational(self) -> LiteralRational;
}
pub trait LiteralRationalUpcastRefMut<'a>: LiteralRationalStructRefMut {
    fn as_literal_rational_ref_mut(self) -> LiteralRationalRefMut<'a>;
}
pub trait LiteralRationalUpcastRef<'a>: LiteralRationalStructRef {
    fn as_literal_rational_ref(self) -> LiteralRationalRef<'a>;
}
impl LiteralRationalStruct for LiteralRationalInner {
    fn value(self) -> f64 {
        self.value
    }
}
impl LiteralRationalStructRefMut for LiteralRationalInner {
    fn value_ref_mut(&mut self) -> &mut f64 {
        &mut self.value
    }
}
impl LiteralRationalStructRef for LiteralRationalInner {
    fn value_ref(&self) -> &f64 {
        &self.value
    }
}
impl LiteralExpressionStruct for LiteralRationalInner {}
impl LiteralExpressionStructRefMut for LiteralRationalInner {}
impl LiteralExpressionStructRef for LiteralRationalInner {}
impl ExpressionStruct for LiteralRationalInner {}
impl ExpressionStructRefMut for LiteralRationalInner {}
impl ExpressionStructRef for LiteralRationalInner {}
impl StepStruct for LiteralRationalInner {}
impl StepStructRefMut for LiteralRationalInner {}
impl StepStructRef for LiteralRationalInner {}
impl FeatureStruct for LiteralRationalInner {
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
impl FeatureStructRefMut for LiteralRationalInner {
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
impl FeatureStructRef for LiteralRationalInner {
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
impl TypeStruct for LiteralRationalInner {
    fn is_abstract(self) -> bool {
        self.sup_literal_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_literal_expression.is_sufficient()
    }
}
impl TypeStructRefMut for LiteralRationalInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_literal_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for LiteralRationalInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_literal_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_literal_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for LiteralRationalInner {}
impl NamespaceStructRefMut for LiteralRationalInner {}
impl NamespaceStructRef for LiteralRationalInner {}
impl ElementStruct for LiteralRationalInner {
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
impl ElementStructRefMut for LiteralRationalInner {
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
impl ElementStructRef for LiteralRationalInner {
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
pub enum LiteralRational {
    Itself(LiteralRationalInner),
}
pub enum LiteralRationalRefMut<'a> {
    Itself(&'a mut LiteralRationalInner),
}
pub enum LiteralRationalRef<'a> {
    Itself(&'a LiteralRationalInner),
}
impl LiteralRational {
    pub fn as_ref(&self) -> LiteralRationalRef {
        match self {
            LiteralRational::Itself(inner) => LiteralRationalRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> LiteralRationalRefMut {
        match self {
            LiteralRational::Itself(inner) => LiteralRationalRefMut::Itself(inner),
        }
    }
}
impl<'a> LiteralRationalRefMut<'a> {
    pub fn as_ref(self) -> LiteralRationalRef<'a> {
        match self {
            LiteralRationalRefMut::Itself(inner) => LiteralRationalRef::Itself(inner),
        }
    }
}
impl LiteralRationalStruct for LiteralRational {
    fn value(self) -> f64 {
        match self {
            LiteralRational::Itself(x) => x.value(),
        }
    }
}
impl LiteralRationalStructRefMut for LiteralRational {
    fn value_ref_mut(&mut self) -> &mut f64 {
        match self {
            LiteralRational::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl LiteralRationalStructRef for LiteralRational {
    fn value_ref(&self) -> &f64 {
        match self {
            LiteralRational::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralRationalStructRefMut for LiteralRationalRefMut<'a> {
    fn value_ref_mut(&mut self) -> &mut f64 {
        match self {
            LiteralRationalRefMut::Itself(x) => x.value_ref_mut(),
        }
    }
}
impl<'a> LiteralRationalStructRef for LiteralRationalRefMut<'a> {
    fn value_ref(&self) -> &f64 {
        match self {
            LiteralRationalRefMut::Itself(x) => x.value_ref(),
        }
    }
}
impl<'a> LiteralRationalStructRef for LiteralRationalRef<'a> {
    fn value_ref(&self) -> &f64 {
        match self {
            LiteralRationalRef::Itself(x) => x.value_ref(),
        }
    }
}
impl LiteralExpressionStruct for LiteralRational {}
impl LiteralExpressionStructRefMut for LiteralRational {}
impl LiteralExpressionStructRef for LiteralRational {}
impl<'a> LiteralExpressionStructRefMut for LiteralRationalRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralRationalRefMut<'a> {}
impl<'a> LiteralExpressionStructRef for LiteralRationalRef<'a> {}
impl ExpressionStruct for LiteralRational {}
impl ExpressionStructRefMut for LiteralRational {}
impl ExpressionStructRef for LiteralRational {}
impl<'a> ExpressionStructRefMut for LiteralRationalRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralRationalRefMut<'a> {}
impl<'a> ExpressionStructRef for LiteralRationalRef<'a> {}
impl StepStruct for LiteralRational {}
impl StepStructRefMut for LiteralRational {}
impl StepStructRef for LiteralRational {}
impl<'a> StepStructRefMut for LiteralRationalRefMut<'a> {}
impl<'a> StepStructRef for LiteralRationalRefMut<'a> {}
impl<'a> StepStructRef for LiteralRationalRef<'a> {}
impl FeatureStruct for LiteralRational {
    fn is_unique(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_constant(),
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
            LiteralRational::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for LiteralRational {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralRational::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for LiteralRational {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_constant_ref(),
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
            LiteralRational::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for LiteralRationalRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            LiteralRationalRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralRationalRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_constant_ref(),
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
            LiteralRationalRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for LiteralRationalRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_constant_ref(),
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
            LiteralRationalRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for LiteralRational {
    fn is_abstract(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for LiteralRational {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for LiteralRational {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for LiteralRationalRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for LiteralRationalRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for LiteralRationalRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for LiteralRational {}
impl NamespaceStructRefMut for LiteralRational {}
impl NamespaceStructRef for LiteralRational {}
impl<'a> NamespaceStructRefMut for LiteralRationalRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralRationalRefMut<'a> {}
impl<'a> NamespaceStructRef for LiteralRationalRef<'a> {}
impl ElementStruct for LiteralRational {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRational::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRational::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            LiteralRational::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            LiteralRational::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            LiteralRational::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            LiteralRational::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            LiteralRational::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for LiteralRational {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRational::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralRational::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralRational::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralRational::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralRational::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralRational::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRational::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for LiteralRational {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRational::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRational::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralRational::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralRational::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralRational::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralRational::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralRational::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for LiteralRationalRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            LiteralRationalRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            LiteralRationalRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for LiteralRationalRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralRationalRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralRationalRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralRationalRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for LiteralRationalRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRationalRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            LiteralRationalRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            LiteralRationalRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            LiteralRationalRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            LiteralRationalRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            LiteralRationalRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            LiteralRationalRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl LiteralRationalUpcast for LiteralRational {
    fn into_literal_rational(self) -> LiteralRational {
        self
    }
}
impl<'a> LiteralRationalUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_literal_rational_ref_mut(self) -> LiteralRationalRefMut<'a> {
        self
    }
}
impl<'a> LiteralRationalUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_literal_rational_ref(self) -> LiteralRationalRef<'a> {
        self
    }
}
impl LiteralExpressionUpcast for LiteralRational {
    fn into_literal_expression(self) -> LiteralExpression {
        LiteralExpression::LiteralRational(self).into_literal_expression()
    }
}
impl<'a> LiteralExpressionUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_literal_expression_ref_mut(self) -> LiteralExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_literal_expression_ref(self) -> LiteralExpressionRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_literal_expression_ref()
    }
}
impl ExpressionUpcast for LiteralRational {
    fn into_expression(self) -> Expression {
        LiteralExpression::LiteralRational(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_expression_ref()
    }
}
impl StepUpcast for LiteralRational {
    fn into_step(self) -> Step {
        LiteralExpression::LiteralRational(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_step_ref()
    }
}
impl FeatureUpcast for LiteralRational {
    fn into_feature(self) -> Feature {
        LiteralExpression::LiteralRational(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_feature_ref()
    }
}
impl TypeUpcast for LiteralRational {
    fn into_type_(self) -> Type {
        LiteralExpression::LiteralRational(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_type_ref()
    }
}
impl NamespaceUpcast for LiteralRational {
    fn into_namespace(self) -> Namespace {
        LiteralExpression::LiteralRational(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_namespace_ref()
    }
}
impl ElementUpcast for LiteralRational {
    fn into_element(self) -> Element {
        LiteralExpression::LiteralRational(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for LiteralRationalRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        LiteralExpressionRefMut::LiteralRational(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for LiteralRationalRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        LiteralExpressionRef::LiteralRational(self).as_element_ref()
    }
}
pub trait LiteralRationalDowncast {}
pub trait LiteralRationalDowncastRefMut<'a> {}
pub trait LiteralRationalDowncastRef<'a> {}
impl LiteralRationalDowncast for LiteralRational {}
impl<'a> LiteralRationalDowncastRefMut<'a> for LiteralRationalRefMut<'a> {}
impl<'a> LiteralRationalDowncastRef<'a> for LiteralRationalRef<'a> {}
pub trait LiteralRationalMethodsDescendants
where
    Self: DescendantOf<LiteralRational>,
    Self::Via: LiteralRationalMethods,
    Self: Sized,
{}
pub trait LiteralRationalMethods: Sized {}
impl<T: LiteralRationalMethodsDescendants> LiteralRationalMethods for T
where
    T::Via: LiteralRationalMethods,
{}
impl DescendantOf<LiteralExpression> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl LiteralExpressionMethodsDescendants for LiteralRational {}
impl DescendantOf<Expression> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ExpressionMethodsDescendants for LiteralRational {}
impl DescendantOf<Step> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl StepMethodsDescendants for LiteralRational {}
impl DescendantOf<Feature> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl FeatureMethodsDescendants for LiteralRational {}
impl DescendantOf<Type> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl TypeMethodsDescendants for LiteralRational {}
impl DescendantOf<Namespace> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl NamespaceMethodsDescendants for LiteralRational {}
impl DescendantOf<Element> for LiteralRational {
    type Via = LiteralExpression;
    fn into_via(self) -> Self::Via {
        self.into_literal_expression()
    }
}
impl ElementMethodsDescendants for LiteralRational {}
pub trait LiteralRationalRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralRationalRefMut<'a>>,
    Self::Via: LiteralRationalRefMutMethods,
    Self: Sized,
{}
pub trait LiteralRationalRefMutMethods: Sized {}
impl<'a, T: LiteralRationalRefMutMethodsDescendants<'a>> LiteralRationalRefMutMethods
for T
where
    T::Via: LiteralRationalRefMutMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> LiteralExpressionRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for LiteralRationalRefMut<'a> {
    type Via = LiteralExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for LiteralRationalRefMut<'a> {}
pub trait LiteralRationalRefMethodsDescendants<'a>
where
    Self: DescendantOf<LiteralRationalRef<'a>>,
    Self::Via: LiteralRationalRefMethods,
    Self: Sized,
{}
pub trait LiteralRationalRefMethods: Sized {}
impl<'a, T: LiteralRationalRefMethodsDescendants<'a>> LiteralRationalRefMethods for T
where
    T::Via: LiteralRationalRefMethods,
{}
impl<'a> DescendantOf<LiteralExpressionRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> LiteralExpressionRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}
impl<'a> DescendantOf<ExpressionRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for LiteralRationalRef<'a> {
    type Via = LiteralExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_literal_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for LiteralRationalRef<'a> {}

