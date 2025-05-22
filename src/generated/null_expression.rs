#![allow(unused)]
use super::utils::DescendantOf;
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
pub struct NullExpressionInner {
    pub(super) sup_expression: ExpressionInner,
}
pub trait NullExpressionStruct
where
    Self: NullExpressionStructRefMut,
    Self: NullExpressionStructRef,
    Self: ExpressionStruct,
{}
pub trait NullExpressionStructRefMut
where
    Self: NullExpressionStructRef,
    Self: ExpressionStructRefMut,
{}
pub trait NullExpressionStructRef
where
    Self: ExpressionStructRef,
{}
pub trait NullExpressionUpcast: NullExpressionStruct {
    fn into_null_expression(self) -> NullExpression;
}
pub trait NullExpressionUpcastRefMut<'a>: NullExpressionStructRefMut {
    fn as_null_expression_ref_mut(self) -> NullExpressionRefMut<'a>;
}
pub trait NullExpressionUpcastRef<'a>: NullExpressionStructRef {
    fn as_null_expression_ref(self) -> NullExpressionRef<'a>;
}
impl NullExpressionStruct for NullExpressionInner {}
impl NullExpressionStructRefMut for NullExpressionInner {}
impl NullExpressionStructRef for NullExpressionInner {}
impl ExpressionStruct for NullExpressionInner {}
impl ExpressionStructRefMut for NullExpressionInner {}
impl ExpressionStructRef for NullExpressionInner {}
impl StepStruct for NullExpressionInner {}
impl StepStructRefMut for NullExpressionInner {}
impl StepStructRef for NullExpressionInner {}
impl FeatureStruct for NullExpressionInner {
    fn is_unique(self) -> bool {
        self.sup_expression.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_expression.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_expression.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_expression.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_expression.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_expression.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_expression.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_expression.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_expression.direction()
    }
}
impl FeatureStructRefMut for NullExpressionInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_expression.direction_ref_mut()
    }
}
impl FeatureStructRef for NullExpressionInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_expression.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_expression.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_expression.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_expression.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_expression.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_expression.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_expression.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_expression.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_expression.direction_ref()
    }
}
impl TypeStruct for NullExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_expression.is_sufficient()
    }
}
impl TypeStructRefMut for NullExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for NullExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for NullExpressionInner {}
impl NamespaceStructRefMut for NullExpressionInner {}
impl NamespaceStructRef for NullExpressionInner {}
impl ElementStruct for NullExpressionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_expression.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_expression.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_expression.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_expression.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_expression.is_implied_included()
    }
}
impl ElementStructRefMut for NullExpressionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_expression.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_expression.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_expression.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_expression.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_expression.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for NullExpressionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_expression.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_expression.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_expression.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_expression.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_expression.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_expression.is_implied_included_ref()
    }
}
pub enum NullExpression {
    Itself(NullExpressionInner),
}
pub enum NullExpressionRefMut<'a> {
    Itself(&'a mut NullExpressionInner),
}
pub enum NullExpressionRef<'a> {
    Itself(&'a NullExpressionInner),
}
impl NullExpression {
    pub fn as_ref(&self) -> NullExpressionRef {
        match self {
            NullExpression::Itself(inner) => NullExpressionRef::Itself(&inner),
        }
    }
    pub fn as_ref_mut(&mut self) -> NullExpressionRefMut {
        match self {
            NullExpression::Itself(inner) => NullExpressionRefMut::Itself(inner),
        }
    }
}
impl<'a> NullExpressionRefMut<'a> {
    pub fn as_ref(self) -> NullExpressionRef<'a> {
        match self {
            NullExpressionRefMut::Itself(inner) => NullExpressionRef::Itself(inner),
        }
    }
}
impl NullExpressionStruct for NullExpression {}
impl NullExpressionStructRefMut for NullExpression {}
impl NullExpressionStructRef for NullExpression {}
impl<'a> NullExpressionStructRefMut for NullExpressionRefMut<'a> {}
impl<'a> NullExpressionStructRef for NullExpressionRefMut<'a> {}
impl<'a> NullExpressionStructRef for NullExpressionRef<'a> {}
impl ExpressionStruct for NullExpression {}
impl ExpressionStructRefMut for NullExpression {}
impl ExpressionStructRef for NullExpression {}
impl<'a> ExpressionStructRefMut for NullExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for NullExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for NullExpressionRef<'a> {}
impl StepStruct for NullExpression {}
impl StepStructRefMut for NullExpression {}
impl StepStructRef for NullExpression {}
impl<'a> StepStructRefMut for NullExpressionRefMut<'a> {}
impl<'a> StepStructRef for NullExpressionRefMut<'a> {}
impl<'a> StepStructRef for NullExpressionRef<'a> {}
impl FeatureStruct for NullExpression {
    fn is_unique(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_constant(),
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
            NullExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for NullExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_constant_ref_mut(),
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
            NullExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for NullExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_constant_ref(),
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
            NullExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for NullExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            NullExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for NullExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            NullExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for NullExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_constant_ref(),
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
            NullExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for NullExpression {
    fn is_abstract(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for NullExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for NullExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for NullExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for NullExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for NullExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for NullExpression {}
impl NamespaceStructRefMut for NullExpression {}
impl NamespaceStructRef for NullExpression {}
impl<'a> NamespaceStructRefMut for NullExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for NullExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for NullExpressionRef<'a> {}
impl ElementStruct for NullExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            NullExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            NullExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            NullExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            NullExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            NullExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for NullExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            NullExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            NullExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            NullExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NullExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NullExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for NullExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NullExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NullExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NullExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NullExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NullExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for NullExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            NullExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            NullExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            NullExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NullExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            NullExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for NullExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NullExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NullExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NullExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NullExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NullExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for NullExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            NullExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            NullExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            NullExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            NullExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            NullExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            NullExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl NullExpressionUpcast for NullExpression {
    fn into_null_expression(self) -> NullExpression {
        self
    }
}
impl<'a> NullExpressionUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_null_expression_ref_mut(self) -> NullExpressionRefMut<'a> {
        self
    }
}
impl<'a> NullExpressionUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_null_expression_ref(self) -> NullExpressionRef<'a> {
        self
    }
}
impl ExpressionUpcast for NullExpression {
    fn into_expression(self) -> Expression {
        Expression::NullExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        ExpressionRefMut::NullExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        ExpressionRef::NullExpression(self).as_expression_ref()
    }
}
impl StepUpcast for NullExpression {
    fn into_step(self) -> Step {
        Expression::NullExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        ExpressionRefMut::NullExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        ExpressionRef::NullExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for NullExpression {
    fn into_feature(self) -> Feature {
        Expression::NullExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ExpressionRefMut::NullExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ExpressionRef::NullExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for NullExpression {
    fn into_type_(self) -> Type {
        Expression::NullExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ExpressionRefMut::NullExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ExpressionRef::NullExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for NullExpression {
    fn into_namespace(self) -> Namespace {
        Expression::NullExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ExpressionRefMut::NullExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ExpressionRef::NullExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for NullExpression {
    fn into_element(self) -> Element {
        Expression::NullExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for NullExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ExpressionRefMut::NullExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for NullExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ExpressionRef::NullExpression(self).as_element_ref()
    }
}
pub trait NullExpressionDowncast {}
pub trait NullExpressionDowncastRefMut<'a> {}
pub trait NullExpressionDowncastRef<'a> {}
impl NullExpressionDowncast for NullExpression {}
impl<'a> NullExpressionDowncastRefMut<'a> for NullExpressionRefMut<'a> {}
impl<'a> NullExpressionDowncastRef<'a> for NullExpressionRef<'a> {}
pub trait NullExpressionMethodsDescendants
where
    Self: DescendantOf<NullExpression>,
    Self::Via: NullExpressionMethods,
    Self: Sized,
{}
pub trait NullExpressionMethods: Sized {}
impl<T: NullExpressionMethodsDescendants> NullExpressionMethods for T
where
    T::Via: NullExpressionMethods,
{}
impl DescendantOf<Expression> for NullExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ExpressionMethodsDescendants for NullExpression {}
impl DescendantOf<Step> for NullExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl StepMethodsDescendants for NullExpression {}
impl DescendantOf<Feature> for NullExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl FeatureMethodsDescendants for NullExpression {}
impl DescendantOf<Type> for NullExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl TypeMethodsDescendants for NullExpression {}
impl DescendantOf<Namespace> for NullExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl NamespaceMethodsDescendants for NullExpression {}
impl DescendantOf<Element> for NullExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ElementMethodsDescendants for NullExpression {}
pub trait NullExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<NullExpressionRefMut<'a>>,
    Self::Via: NullExpressionRefMutMethods,
    Self: Sized,
{}
pub trait NullExpressionRefMutMethods: Sized {}
impl<'a, T: NullExpressionRefMutMethodsDescendants<'a>> NullExpressionRefMutMethods for T
where
    T::Via: NullExpressionRefMutMethods,
{}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for NullExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for NullExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for NullExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for NullExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for NullExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for NullExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for NullExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for NullExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for NullExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for NullExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for NullExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for NullExpressionRefMut<'a> {}
pub trait NullExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<NullExpressionRef<'a>>,
    Self::Via: NullExpressionRefMethods,
    Self: Sized,
{}
pub trait NullExpressionRefMethods: Sized {}
impl<'a, T: NullExpressionRefMethodsDescendants<'a>> NullExpressionRefMethods for T
where
    T::Via: NullExpressionRefMethods,
{}
impl<'a> DescendantOf<ExpressionRef<'a>> for NullExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for NullExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for NullExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for NullExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for NullExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for NullExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for NullExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for NullExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for NullExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for NullExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for NullExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for NullExpressionRef<'a> {}

