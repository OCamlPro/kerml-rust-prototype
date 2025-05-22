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
pub struct FeatureReferenceExpressionInner {
    pub(super) sup_expression: ExpressionInner,
}
pub trait FeatureReferenceExpressionStruct
where
    Self: FeatureReferenceExpressionStructRefMut,
    Self: FeatureReferenceExpressionStructRef,
    Self: ExpressionStruct,
{}
pub trait FeatureReferenceExpressionStructRefMut
where
    Self: FeatureReferenceExpressionStructRef,
    Self: ExpressionStructRefMut,
{}
pub trait FeatureReferenceExpressionStructRef
where
    Self: ExpressionStructRef,
{}
pub trait FeatureReferenceExpressionUpcast: FeatureReferenceExpressionStruct {
    fn into_feature_reference_expression(self) -> FeatureReferenceExpression;
}
pub trait FeatureReferenceExpressionUpcastRefMut<
    'a,
>: FeatureReferenceExpressionStructRefMut {
    fn as_feature_reference_expression_ref_mut(
        self,
    ) -> FeatureReferenceExpressionRefMut<'a>;
}
pub trait FeatureReferenceExpressionUpcastRef<'a>: FeatureReferenceExpressionStructRef {
    fn as_feature_reference_expression_ref(self) -> FeatureReferenceExpressionRef<'a>;
}
impl FeatureReferenceExpressionStruct for FeatureReferenceExpressionInner {}
impl FeatureReferenceExpressionStructRefMut for FeatureReferenceExpressionInner {}
impl FeatureReferenceExpressionStructRef for FeatureReferenceExpressionInner {}
impl ExpressionStruct for FeatureReferenceExpressionInner {}
impl ExpressionStructRefMut for FeatureReferenceExpressionInner {}
impl ExpressionStructRef for FeatureReferenceExpressionInner {}
impl StepStruct for FeatureReferenceExpressionInner {}
impl StepStructRefMut for FeatureReferenceExpressionInner {}
impl StepStructRef for FeatureReferenceExpressionInner {}
impl FeatureStruct for FeatureReferenceExpressionInner {
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
impl FeatureStructRefMut for FeatureReferenceExpressionInner {
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
impl FeatureStructRef for FeatureReferenceExpressionInner {
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
impl TypeStruct for FeatureReferenceExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_expression.is_sufficient()
    }
}
impl TypeStructRefMut for FeatureReferenceExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for FeatureReferenceExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for FeatureReferenceExpressionInner {}
impl NamespaceStructRefMut for FeatureReferenceExpressionInner {}
impl NamespaceStructRef for FeatureReferenceExpressionInner {}
impl ElementStruct for FeatureReferenceExpressionInner {
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
impl ElementStructRefMut for FeatureReferenceExpressionInner {
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
impl ElementStructRef for FeatureReferenceExpressionInner {
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
pub enum FeatureReferenceExpression {
    Itself(FeatureReferenceExpressionInner),
}
pub enum FeatureReferenceExpressionRefMut<'a> {
    Itself(&'a mut FeatureReferenceExpressionInner),
}
pub enum FeatureReferenceExpressionRef<'a> {
    Itself(&'a FeatureReferenceExpressionInner),
}
impl FeatureReferenceExpression {
    pub fn as_ref(&self) -> FeatureReferenceExpressionRef {
        match self {
            FeatureReferenceExpression::Itself(inner) => {
                FeatureReferenceExpressionRef::Itself(&inner)
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> FeatureReferenceExpressionRefMut {
        match self {
            FeatureReferenceExpression::Itself(inner) => {
                FeatureReferenceExpressionRefMut::Itself(inner)
            }
        }
    }
}
impl<'a> FeatureReferenceExpressionRefMut<'a> {
    pub fn as_ref(self) -> FeatureReferenceExpressionRef<'a> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(inner) => {
                FeatureReferenceExpressionRef::Itself(inner)
            }
        }
    }
}
impl FeatureReferenceExpressionStruct for FeatureReferenceExpression {}
impl FeatureReferenceExpressionStructRefMut for FeatureReferenceExpression {}
impl FeatureReferenceExpressionStructRef for FeatureReferenceExpression {}
impl<'a> FeatureReferenceExpressionStructRefMut
for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> FeatureReferenceExpressionStructRef for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> FeatureReferenceExpressionStructRef for FeatureReferenceExpressionRef<'a> {}
impl ExpressionStruct for FeatureReferenceExpression {}
impl ExpressionStructRefMut for FeatureReferenceExpression {}
impl ExpressionStructRef for FeatureReferenceExpression {}
impl<'a> ExpressionStructRefMut for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for FeatureReferenceExpressionRef<'a> {}
impl StepStruct for FeatureReferenceExpression {}
impl StepStructRefMut for FeatureReferenceExpression {}
impl StepStructRef for FeatureReferenceExpression {}
impl<'a> StepStructRefMut for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> StepStructRef for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> StepStructRef for FeatureReferenceExpressionRef<'a> {}
impl FeatureStruct for FeatureReferenceExpression {
    fn is_unique(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_constant(),
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
            FeatureReferenceExpression::Itself(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for FeatureReferenceExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_constant_ref_mut(),
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
            FeatureReferenceExpression::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for FeatureReferenceExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_constant_ref(),
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
            FeatureReferenceExpression::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for FeatureReferenceExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
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
            FeatureReferenceExpressionRefMut::Itself(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for FeatureReferenceExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_constant_ref(),
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
            FeatureReferenceExpressionRefMut::Itself(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for FeatureReferenceExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_constant_ref(),
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
            FeatureReferenceExpressionRef::Itself(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for FeatureReferenceExpression {
    fn is_abstract(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for FeatureReferenceExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for FeatureReferenceExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for FeatureReferenceExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for FeatureReferenceExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for FeatureReferenceExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for FeatureReferenceExpression {}
impl NamespaceStructRefMut for FeatureReferenceExpression {}
impl NamespaceStructRef for FeatureReferenceExpression {}
impl<'a> NamespaceStructRefMut for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for FeatureReferenceExpressionRef<'a> {}
impl ElementStruct for FeatureReferenceExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            FeatureReferenceExpression::Itself(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for FeatureReferenceExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureReferenceExpression::Itself(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureReferenceExpression::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for FeatureReferenceExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureReferenceExpression::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureReferenceExpression::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpression::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for FeatureReferenceExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for FeatureReferenceExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for FeatureReferenceExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            FeatureReferenceExpressionRef::Itself(x) => x.is_implied_included_ref(),
        }
    }
}
impl FeatureReferenceExpressionUpcast for FeatureReferenceExpression {
    fn into_feature_reference_expression(self) -> FeatureReferenceExpression {
        self
    }
}
impl<'a> FeatureReferenceExpressionUpcastRefMut<'a>
for FeatureReferenceExpressionRefMut<'a> {
    fn as_feature_reference_expression_ref_mut(
        self,
    ) -> FeatureReferenceExpressionRefMut<'a> {
        self
    }
}
impl<'a> FeatureReferenceExpressionUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_feature_reference_expression_ref(self) -> FeatureReferenceExpressionRef<'a> {
        self
    }
}
impl ExpressionUpcast for FeatureReferenceExpression {
    fn into_expression(self) -> Expression {
        Expression::FeatureReferenceExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for FeatureReferenceExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        ExpressionRefMut::FeatureReferenceExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        ExpressionRef::FeatureReferenceExpression(self).as_expression_ref()
    }
}
impl StepUpcast for FeatureReferenceExpression {
    fn into_step(self) -> Step {
        Expression::FeatureReferenceExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for FeatureReferenceExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        ExpressionRefMut::FeatureReferenceExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        ExpressionRef::FeatureReferenceExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for FeatureReferenceExpression {
    fn into_feature(self) -> Feature {
        Expression::FeatureReferenceExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for FeatureReferenceExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ExpressionRefMut::FeatureReferenceExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ExpressionRef::FeatureReferenceExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for FeatureReferenceExpression {
    fn into_type_(self) -> Type {
        Expression::FeatureReferenceExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for FeatureReferenceExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ExpressionRefMut::FeatureReferenceExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ExpressionRef::FeatureReferenceExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for FeatureReferenceExpression {
    fn into_namespace(self) -> Namespace {
        Expression::FeatureReferenceExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for FeatureReferenceExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ExpressionRefMut::FeatureReferenceExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ExpressionRef::FeatureReferenceExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for FeatureReferenceExpression {
    fn into_element(self) -> Element {
        Expression::FeatureReferenceExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for FeatureReferenceExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ExpressionRefMut::FeatureReferenceExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for FeatureReferenceExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ExpressionRef::FeatureReferenceExpression(self).as_element_ref()
    }
}
pub trait FeatureReferenceExpressionDowncast {}
pub trait FeatureReferenceExpressionDowncastRefMut<'a> {}
pub trait FeatureReferenceExpressionDowncastRef<'a> {}
impl FeatureReferenceExpressionDowncast for FeatureReferenceExpression {}
impl<'a> FeatureReferenceExpressionDowncastRefMut<'a>
for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> FeatureReferenceExpressionDowncastRef<'a>
for FeatureReferenceExpressionRef<'a> {}
pub trait FeatureReferenceExpressionMethodsDescendants
where
    Self: DescendantOf<FeatureReferenceExpression>,
    Self::Via: FeatureReferenceExpressionMethods,
    Self: Sized,
{}
pub trait FeatureReferenceExpressionMethods: Sized {}
impl<T: FeatureReferenceExpressionMethodsDescendants> FeatureReferenceExpressionMethods
for T
where
    T::Via: FeatureReferenceExpressionMethods,
{}
impl DescendantOf<Expression> for FeatureReferenceExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ExpressionMethodsDescendants for FeatureReferenceExpression {}
impl DescendantOf<Step> for FeatureReferenceExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl StepMethodsDescendants for FeatureReferenceExpression {}
impl DescendantOf<Feature> for FeatureReferenceExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl FeatureMethodsDescendants for FeatureReferenceExpression {}
impl DescendantOf<Type> for FeatureReferenceExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl TypeMethodsDescendants for FeatureReferenceExpression {}
impl DescendantOf<Namespace> for FeatureReferenceExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl NamespaceMethodsDescendants for FeatureReferenceExpression {}
impl DescendantOf<Element> for FeatureReferenceExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ElementMethodsDescendants for FeatureReferenceExpression {}
pub trait FeatureReferenceExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureReferenceExpressionRefMut<'a>>,
    Self::Via: FeatureReferenceExpressionRefMutMethods,
    Self: Sized,
{}
pub trait FeatureReferenceExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: FeatureReferenceExpressionRefMutMethodsDescendants<'a>,
> FeatureReferenceExpressionRefMutMethods for T
where
    T::Via: FeatureReferenceExpressionRefMutMethods,
{}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for FeatureReferenceExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a>
for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for FeatureReferenceExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for FeatureReferenceExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for FeatureReferenceExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for FeatureReferenceExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for FeatureReferenceExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for FeatureReferenceExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for FeatureReferenceExpressionRefMut<'a> {}
pub trait FeatureReferenceExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<FeatureReferenceExpressionRef<'a>>,
    Self::Via: FeatureReferenceExpressionRefMethods,
    Self: Sized,
{}
pub trait FeatureReferenceExpressionRefMethods: Sized {}
impl<
    'a,
    T: FeatureReferenceExpressionRefMethodsDescendants<'a>,
> FeatureReferenceExpressionRefMethods for T
where
    T::Via: FeatureReferenceExpressionRefMethods,
{}
impl<'a> DescendantOf<ExpressionRef<'a>> for FeatureReferenceExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for FeatureReferenceExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for FeatureReferenceExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for FeatureReferenceExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for FeatureReferenceExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for FeatureReferenceExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for FeatureReferenceExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for FeatureReferenceExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for FeatureReferenceExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for FeatureReferenceExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for FeatureReferenceExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for FeatureReferenceExpressionRef<'a> {}

