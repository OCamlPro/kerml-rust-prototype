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
use super::constructor_expression::{
    ConstructorExpression, ConstructorExpressionRefMut, ConstructorExpressionRef,
};
use super::invocation_expression::{
    InvocationExpression, InvocationExpressionRefMut, InvocationExpressionRef,
};
pub struct InstantiationExpressionInner {
    pub(super) sup_expression: ExpressionInner,
}
pub trait InstantiationExpressionStruct
where
    Self: InstantiationExpressionStructRefMut,
    Self: InstantiationExpressionStructRef,
    Self: ExpressionStruct,
{}
pub trait InstantiationExpressionStructRefMut
where
    Self: InstantiationExpressionStructRef,
    Self: ExpressionStructRefMut,
{}
pub trait InstantiationExpressionStructRef
where
    Self: ExpressionStructRef,
{}
pub trait InstantiationExpressionUpcast: InstantiationExpressionStruct {
    fn into_instantiation_expression(self) -> InstantiationExpression;
}
pub trait InstantiationExpressionUpcastRefMut<'a>: InstantiationExpressionStructRefMut {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a>;
}
pub trait InstantiationExpressionUpcastRef<'a>: InstantiationExpressionStructRef {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a>;
}
impl InstantiationExpressionStruct for InstantiationExpressionInner {}
impl InstantiationExpressionStructRefMut for InstantiationExpressionInner {}
impl InstantiationExpressionStructRef for InstantiationExpressionInner {}
impl ExpressionStruct for InstantiationExpressionInner {}
impl ExpressionStructRefMut for InstantiationExpressionInner {}
impl ExpressionStructRef for InstantiationExpressionInner {}
impl StepStruct for InstantiationExpressionInner {}
impl StepStructRefMut for InstantiationExpressionInner {}
impl StepStructRef for InstantiationExpressionInner {}
impl FeatureStruct for InstantiationExpressionInner {
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
impl FeatureStructRefMut for InstantiationExpressionInner {
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
impl FeatureStructRef for InstantiationExpressionInner {
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
impl TypeStruct for InstantiationExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_expression.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_expression.is_sufficient()
    }
}
impl TypeStructRefMut for InstantiationExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_expression.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for InstantiationExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_expression.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_expression.is_sufficient_ref()
    }
}
impl NamespaceStruct for InstantiationExpressionInner {}
impl NamespaceStructRefMut for InstantiationExpressionInner {}
impl NamespaceStructRef for InstantiationExpressionInner {}
impl ElementStruct for InstantiationExpressionInner {
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
impl ElementStructRefMut for InstantiationExpressionInner {
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
impl ElementStructRef for InstantiationExpressionInner {
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
pub enum InstantiationExpression {
    Itself(InstantiationExpressionInner),
    ConstructorExpression(ConstructorExpression),
    InvocationExpression(InvocationExpression),
}
pub enum InstantiationExpressionRefMut<'a> {
    Itself(&'a mut InstantiationExpressionInner),
    ConstructorExpression(ConstructorExpressionRefMut<'a>),
    InvocationExpression(InvocationExpressionRefMut<'a>),
}
pub enum InstantiationExpressionRef<'a> {
    Itself(&'a InstantiationExpressionInner),
    ConstructorExpression(ConstructorExpressionRef<'a>),
    InvocationExpression(InvocationExpressionRef<'a>),
}
impl InstantiationExpression {
    pub fn as_ref(&self) -> InstantiationExpressionRef {
        match self {
            InstantiationExpression::Itself(inner) => {
                InstantiationExpressionRef::Itself(&inner)
            }
            InstantiationExpression::ConstructorExpression(inner) => {
                InstantiationExpressionRef::ConstructorExpression(inner.as_ref())
            }
            InstantiationExpression::InvocationExpression(inner) => {
                InstantiationExpressionRef::InvocationExpression(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> InstantiationExpressionRefMut {
        match self {
            InstantiationExpression::Itself(inner) => {
                InstantiationExpressionRefMut::Itself(inner)
            }
            InstantiationExpression::ConstructorExpression(inner) => {
                InstantiationExpressionRefMut::ConstructorExpression(inner.as_ref_mut())
            }
            InstantiationExpression::InvocationExpression(inner) => {
                InstantiationExpressionRefMut::InvocationExpression(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> InstantiationExpressionRefMut<'a> {
    pub fn as_ref(self) -> InstantiationExpressionRef<'a> {
        match self {
            InstantiationExpressionRefMut::Itself(inner) => {
                InstantiationExpressionRef::Itself(inner)
            }
            InstantiationExpressionRefMut::ConstructorExpression(inner) => {
                InstantiationExpressionRef::ConstructorExpression(inner.as_ref())
            }
            InstantiationExpressionRefMut::InvocationExpression(inner) => {
                InstantiationExpressionRef::InvocationExpression(inner.as_ref())
            }
        }
    }
}
impl InstantiationExpressionStruct for InstantiationExpression {}
impl InstantiationExpressionStructRefMut for InstantiationExpression {}
impl InstantiationExpressionStructRef for InstantiationExpression {}
impl<'a> InstantiationExpressionStructRefMut for InstantiationExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for InstantiationExpressionRefMut<'a> {}
impl<'a> InstantiationExpressionStructRef for InstantiationExpressionRef<'a> {}
impl ExpressionStruct for InstantiationExpression {}
impl ExpressionStructRefMut for InstantiationExpression {}
impl ExpressionStructRef for InstantiationExpression {}
impl<'a> ExpressionStructRefMut for InstantiationExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for InstantiationExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for InstantiationExpressionRef<'a> {}
impl StepStruct for InstantiationExpression {}
impl StepStructRefMut for InstantiationExpression {}
impl StepStructRef for InstantiationExpression {}
impl<'a> StepStructRefMut for InstantiationExpressionRefMut<'a> {}
impl<'a> StepStructRef for InstantiationExpressionRefMut<'a> {}
impl<'a> StepStructRef for InstantiationExpressionRef<'a> {}
impl FeatureStruct for InstantiationExpression {
    fn is_unique(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_unique(),
            InstantiationExpression::ConstructorExpression(x) => x.is_unique(),
            InstantiationExpression::InvocationExpression(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_ordered(),
            InstantiationExpression::ConstructorExpression(x) => x.is_ordered(),
            InstantiationExpression::InvocationExpression(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_composite(),
            InstantiationExpression::ConstructorExpression(x) => x.is_composite(),
            InstantiationExpression::InvocationExpression(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_end(),
            InstantiationExpression::ConstructorExpression(x) => x.is_end(),
            InstantiationExpression::InvocationExpression(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_derived(),
            InstantiationExpression::ConstructorExpression(x) => x.is_derived(),
            InstantiationExpression::InvocationExpression(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_portion(),
            InstantiationExpression::ConstructorExpression(x) => x.is_portion(),
            InstantiationExpression::InvocationExpression(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_variable(),
            InstantiationExpression::ConstructorExpression(x) => x.is_variable(),
            InstantiationExpression::InvocationExpression(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_constant(),
            InstantiationExpression::ConstructorExpression(x) => x.is_constant(),
            InstantiationExpression::InvocationExpression(x) => x.is_constant(),
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
            InstantiationExpression::Itself(x) => x.direction(),
            InstantiationExpression::ConstructorExpression(x) => x.direction(),
            InstantiationExpression::InvocationExpression(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for InstantiationExpression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_unique_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_unique_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_ordered_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_ordered_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_composite_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_composite_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_end_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_end_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_derived_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_derived_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_portion_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_portion_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_variable_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_variable_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_constant_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_constant_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_constant_ref_mut(),
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
            InstantiationExpression::Itself(x) => x.direction_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.direction_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for InstantiationExpression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_unique_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_unique_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_ordered_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_ordered_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_composite_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_composite_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_end_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_end_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_derived_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_derived_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_portion_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_portion_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_variable_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_variable_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_constant_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_constant_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_constant_ref(),
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
            InstantiationExpression::Itself(x) => x.direction_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.direction_ref(),
            InstantiationExpression::InvocationExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for InstantiationExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_unique_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_unique_ref_mut()
            }
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_ordered_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_ordered_ref_mut()
            }
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_composite_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_composite_ref_mut()
            }
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.is_end_ref_mut(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_derived_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_derived_ref_mut()
            }
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_portion_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_portion_ref_mut()
            }
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_variable_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_variable_ref_mut()
            }
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_constant_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_constant_ref_mut()
            }
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
            InstantiationExpressionRefMut::Itself(x) => x.direction_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.direction_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.direction_ref_mut()
            }
        }
    }
}
impl<'a> FeatureStructRef for InstantiationExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_unique_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.is_unique_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_ordered_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.is_ordered_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_composite_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_composite_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_composite_ref()
            }
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_end_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.is_end_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_derived_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.is_derived_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_portion_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.is_portion_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_variable_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_variable_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_constant_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_constant_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_constant_ref(),
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
            InstantiationExpressionRefMut::Itself(x) => x.direction_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.direction_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for InstantiationExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_unique_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_unique_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_ordered_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_ordered_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_composite_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_composite_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_end_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_end_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_derived_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_derived_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_portion_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_portion_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_variable_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_variable_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_constant_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_constant_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_constant_ref(),
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
            InstantiationExpressionRef::Itself(x) => x.direction_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.direction_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for InstantiationExpression {
    fn is_abstract(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_abstract(),
            InstantiationExpression::ConstructorExpression(x) => x.is_abstract(),
            InstantiationExpression::InvocationExpression(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_sufficient(),
            InstantiationExpression::ConstructorExpression(x) => x.is_sufficient(),
            InstantiationExpression::InvocationExpression(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for InstantiationExpression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_abstract_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.is_abstract_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_sufficient_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.is_sufficient_ref_mut()
            }
            InstantiationExpression::InvocationExpression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for InstantiationExpression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_abstract_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_abstract_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_sufficient_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.is_sufficient_ref(),
            InstantiationExpression::InvocationExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for InstantiationExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_abstract_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_abstract_ref_mut()
            }
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_sufficient_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_sufficient_ref_mut()
            }
        }
    }
}
impl<'a> TypeStructRef for InstantiationExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_abstract_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_abstract_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_sufficient_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_sufficient_ref()
            }
        }
    }
}
impl<'a> TypeStructRef for InstantiationExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_abstract_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_abstract_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_sufficient_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.is_sufficient_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for InstantiationExpression {}
impl NamespaceStructRefMut for InstantiationExpression {}
impl NamespaceStructRef for InstantiationExpression {}
impl<'a> NamespaceStructRefMut for InstantiationExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for InstantiationExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for InstantiationExpressionRef<'a> {}
impl ElementStruct for InstantiationExpression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpression::Itself(x) => x.owned_relationship(),
            InstantiationExpression::ConstructorExpression(x) => x.owned_relationship(),
            InstantiationExpression::InvocationExpression(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpression::Itself(x) => x.owning_relationship(),
            InstantiationExpression::ConstructorExpression(x) => x.owning_relationship(),
            InstantiationExpression::InvocationExpression(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            InstantiationExpression::Itself(x) => x.element_id(),
            InstantiationExpression::ConstructorExpression(x) => x.element_id(),
            InstantiationExpression::InvocationExpression(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            InstantiationExpression::Itself(x) => x.alias_ids(),
            InstantiationExpression::ConstructorExpression(x) => x.alias_ids(),
            InstantiationExpression::InvocationExpression(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            InstantiationExpression::Itself(x) => x.declared_short_name(),
            InstantiationExpression::ConstructorExpression(x) => x.declared_short_name(),
            InstantiationExpression::InvocationExpression(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            InstantiationExpression::Itself(x) => x.declared_name(),
            InstantiationExpression::ConstructorExpression(x) => x.declared_name(),
            InstantiationExpression::InvocationExpression(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_implied_included(),
            InstantiationExpression::ConstructorExpression(x) => x.is_implied_included(),
            InstantiationExpression::InvocationExpression(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for InstantiationExpression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpression::Itself(x) => x.owned_relationship_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.owned_relationship_ref_mut()
            }
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            InstantiationExpression::Itself(x) => x.owning_relationship_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            InstantiationExpression::Itself(x) => x.element_id_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.element_id_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            InstantiationExpression::Itself(x) => x.alias_ids_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => x.alias_ids_ref_mut(),
            InstantiationExpression::InvocationExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InstantiationExpression::Itself(x) => x.declared_short_name_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InstantiationExpression::Itself(x) => x.declared_name_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.declared_name_ref_mut()
            }
            InstantiationExpression::InvocationExpression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_implied_included_ref_mut(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl ElementStructRef for InstantiationExpression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpression::Itself(x) => x.owned_relationship_ref(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.owned_relationship_ref()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpression::Itself(x) => x.owning_relationship_ref(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.owning_relationship_ref()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InstantiationExpression::Itself(x) => x.element_id_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.element_id_ref(),
            InstantiationExpression::InvocationExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InstantiationExpression::Itself(x) => x.alias_ids_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.alias_ids_ref(),
            InstantiationExpression::InvocationExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InstantiationExpression::Itself(x) => x.declared_short_name_ref(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.declared_short_name_ref()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InstantiationExpression::Itself(x) => x.declared_name_ref(),
            InstantiationExpression::ConstructorExpression(x) => x.declared_name_ref(),
            InstantiationExpression::InvocationExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InstantiationExpression::Itself(x) => x.is_implied_included_ref(),
            InstantiationExpression::ConstructorExpression(x) => {
                x.is_implied_included_ref()
            }
            InstantiationExpression::InvocationExpression(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRefMut for InstantiationExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.owned_relationship_ref_mut()
            }
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.owning_relationship_ref_mut()
            }
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.element_id_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.element_id_ref_mut()
            }
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.alias_ids_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.alias_ids_ref_mut()
            }
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.declared_short_name_ref_mut()
            }
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.declared_name_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.declared_name_ref_mut()
            }
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_implied_included_ref_mut()
            }
        }
    }
}
impl<'a> ElementStructRef for InstantiationExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.owned_relationship_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.owning_relationship_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.element_id_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.element_id_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.alias_ids_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => x.alias_ids_ref(),
            InstantiationExpressionRefMut::InvocationExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.declared_short_name_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.declared_name_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.declared_name_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.declared_name_ref()
            }
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
            InstantiationExpressionRefMut::ConstructorExpression(x) => {
                x.is_implied_included_ref()
            }
            InstantiationExpressionRefMut::InvocationExpression(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl<'a> ElementStructRef for InstantiationExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpressionRef::Itself(x) => x.owned_relationship_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => {
                x.owned_relationship_ref()
            }
            InstantiationExpressionRef::InvocationExpression(x) => {
                x.owned_relationship_ref()
            }
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            InstantiationExpressionRef::Itself(x) => x.owning_relationship_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => {
                x.owning_relationship_ref()
            }
            InstantiationExpressionRef::InvocationExpression(x) => {
                x.owning_relationship_ref()
            }
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            InstantiationExpressionRef::Itself(x) => x.element_id_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.element_id_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            InstantiationExpressionRef::Itself(x) => x.alias_ids_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.alias_ids_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            InstantiationExpressionRef::Itself(x) => x.declared_short_name_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => {
                x.declared_short_name_ref()
            }
            InstantiationExpressionRef::InvocationExpression(x) => {
                x.declared_short_name_ref()
            }
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            InstantiationExpressionRef::Itself(x) => x.declared_name_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => x.declared_name_ref(),
            InstantiationExpressionRef::InvocationExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            InstantiationExpressionRef::Itself(x) => x.is_implied_included_ref(),
            InstantiationExpressionRef::ConstructorExpression(x) => {
                x.is_implied_included_ref()
            }
            InstantiationExpressionRef::InvocationExpression(x) => {
                x.is_implied_included_ref()
            }
        }
    }
}
impl InstantiationExpressionUpcast for InstantiationExpression {
    fn into_instantiation_expression(self) -> InstantiationExpression {
        self
    }
}
impl<'a> InstantiationExpressionUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_instantiation_expression_ref_mut(self) -> InstantiationExpressionRefMut<'a> {
        self
    }
}
impl<'a> InstantiationExpressionUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_instantiation_expression_ref(self) -> InstantiationExpressionRef<'a> {
        self
    }
}
impl ExpressionUpcast for InstantiationExpression {
    fn into_expression(self) -> Expression {
        Expression::InstantiationExpression(self).into_expression()
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        ExpressionRefMut::InstantiationExpression(self).as_expression_ref_mut()
    }
}
impl<'a> ExpressionUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        ExpressionRef::InstantiationExpression(self).as_expression_ref()
    }
}
impl StepUpcast for InstantiationExpression {
    fn into_step(self) -> Step {
        Expression::InstantiationExpression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        ExpressionRefMut::InstantiationExpression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        ExpressionRef::InstantiationExpression(self).as_step_ref()
    }
}
impl FeatureUpcast for InstantiationExpression {
    fn into_feature(self) -> Feature {
        Expression::InstantiationExpression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        ExpressionRefMut::InstantiationExpression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        ExpressionRef::InstantiationExpression(self).as_feature_ref()
    }
}
impl TypeUpcast for InstantiationExpression {
    fn into_type_(self) -> Type {
        Expression::InstantiationExpression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        ExpressionRefMut::InstantiationExpression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        ExpressionRef::InstantiationExpression(self).as_type_ref()
    }
}
impl NamespaceUpcast for InstantiationExpression {
    fn into_namespace(self) -> Namespace {
        Expression::InstantiationExpression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        ExpressionRefMut::InstantiationExpression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        ExpressionRef::InstantiationExpression(self).as_namespace_ref()
    }
}
impl ElementUpcast for InstantiationExpression {
    fn into_element(self) -> Element {
        Expression::InstantiationExpression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for InstantiationExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        ExpressionRefMut::InstantiationExpression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for InstantiationExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        ExpressionRef::InstantiationExpression(self).as_element_ref()
    }
}
pub trait InstantiationExpressionDowncast {
    fn try_into_constructor_expression(self) -> Result<ConstructorExpression, String>;
    fn try_into_invocation_expression(self) -> Result<InvocationExpression, String>;
}
pub trait InstantiationExpressionDowncastRefMut<'a> {
    fn try_as_constructor_expression_ref_mut(
        self,
    ) -> Result<ConstructorExpressionRefMut<'a>, String>;
    fn try_as_invocation_expression_ref_mut(
        self,
    ) -> Result<InvocationExpressionRefMut<'a>, String>;
}
pub trait InstantiationExpressionDowncastRef<'a> {
    fn try_as_constructor_expression_ref(
        self,
    ) -> Result<ConstructorExpressionRef<'a>, String>;
    fn try_as_invocation_expression_ref(
        self,
    ) -> Result<InvocationExpressionRef<'a>, String>;
}
impl InstantiationExpressionDowncast for InstantiationExpression {
    fn try_into_constructor_expression(self) -> Result<ConstructorExpression, String> {
        match self {
            InstantiationExpression::ConstructorExpression(e) => Ok(e),
            _ => Err("Not a ConstructorExpression".into()),
        }
    }
    fn try_into_invocation_expression(self) -> Result<InvocationExpression, String> {
        match self {
            InstantiationExpression::InvocationExpression(e) => Ok(e),
            _ => Err("Not a InvocationExpression".into()),
        }
    }
}
impl<'a> InstantiationExpressionDowncastRefMut<'a>
for InstantiationExpressionRefMut<'a> {
    fn try_as_constructor_expression_ref_mut(
        self,
    ) -> Result<ConstructorExpressionRefMut<'a>, String> {
        match self {
            InstantiationExpressionRefMut::ConstructorExpression(e) => Ok(e),
            _ => Err("Not a ConstructorExpression".into()),
        }
    }
    fn try_as_invocation_expression_ref_mut(
        self,
    ) -> Result<InvocationExpressionRefMut<'a>, String> {
        match self {
            InstantiationExpressionRefMut::InvocationExpression(e) => Ok(e),
            _ => Err("Not a InvocationExpression".into()),
        }
    }
}
impl<'a> InstantiationExpressionDowncastRef<'a> for InstantiationExpressionRef<'a> {
    fn try_as_constructor_expression_ref(
        self,
    ) -> Result<ConstructorExpressionRef<'a>, String> {
        match self {
            InstantiationExpressionRef::ConstructorExpression(e) => Ok(e),
            _ => Err("Not a ConstructorExpression".into()),
        }
    }
    fn try_as_invocation_expression_ref(
        self,
    ) -> Result<InvocationExpressionRef<'a>, String> {
        match self {
            InstantiationExpressionRef::InvocationExpression(e) => Ok(e),
            _ => Err("Not a InvocationExpression".into()),
        }
    }
}
pub trait InstantiationExpressionMethodsDescendants
where
    Self: DescendantOf<InstantiationExpression>,
    Self::Via: InstantiationExpressionMethods,
    Self: Sized,
{}
pub trait InstantiationExpressionMethods: Sized {}
impl<T: InstantiationExpressionMethodsDescendants> InstantiationExpressionMethods for T
where
    T::Via: InstantiationExpressionMethods,
{}
impl DescendantOf<Expression> for InstantiationExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ExpressionMethodsDescendants for InstantiationExpression {}
impl DescendantOf<Step> for InstantiationExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl StepMethodsDescendants for InstantiationExpression {}
impl DescendantOf<Feature> for InstantiationExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl FeatureMethodsDescendants for InstantiationExpression {}
impl DescendantOf<Type> for InstantiationExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl TypeMethodsDescendants for InstantiationExpression {}
impl DescendantOf<Namespace> for InstantiationExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl NamespaceMethodsDescendants for InstantiationExpression {}
impl DescendantOf<Element> for InstantiationExpression {
    type Via = Expression;
    fn into_via(self) -> Self::Via {
        self.into_expression()
    }
}
impl ElementMethodsDescendants for InstantiationExpression {}
pub trait InstantiationExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<InstantiationExpressionRefMut<'a>>,
    Self::Via: InstantiationExpressionRefMutMethods,
    Self: Sized,
{}
pub trait InstantiationExpressionRefMutMethods: Sized {}
impl<
    'a,
    T: InstantiationExpressionRefMutMethodsDescendants<'a>,
> InstantiationExpressionRefMutMethods for T
where
    T::Via: InstantiationExpressionRefMutMethods,
{}
impl<'a> DescendantOf<ExpressionRefMut<'a>> for InstantiationExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ExpressionRefMutMethodsDescendants<'a> for InstantiationExpressionRefMut<'a> {}
impl<'a> DescendantOf<StepRefMut<'a>> for InstantiationExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for InstantiationExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for InstantiationExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for InstantiationExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for InstantiationExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for InstantiationExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for InstantiationExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for InstantiationExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for InstantiationExpressionRefMut<'a> {
    type Via = ExpressionRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for InstantiationExpressionRefMut<'a> {}
pub trait InstantiationExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<InstantiationExpressionRef<'a>>,
    Self::Via: InstantiationExpressionRefMethods,
    Self: Sized,
{
    fn instantiated_type_impl(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        self.into_via().instantiated_type()
    }
}
pub trait InstantiationExpressionRefMethods: Sized {
    fn instantiated_type(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::type_::Type>>>;
}
impl<
    'a,
    T: InstantiationExpressionRefMethodsDescendants<'a>,
> InstantiationExpressionRefMethods for T
where
    T::Via: InstantiationExpressionRefMethods,
{
    fn instantiated_type(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::type_::Type>>> {
        InstantiationExpressionRefMethodsDescendants::instantiated_type_impl(self)
    }
}
impl<'a> DescendantOf<ExpressionRef<'a>> for InstantiationExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ExpressionRefMethodsDescendants<'a> for InstantiationExpressionRef<'a> {}
impl<'a> DescendantOf<StepRef<'a>> for InstantiationExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for InstantiationExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for InstantiationExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for InstantiationExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for InstantiationExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for InstantiationExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for InstantiationExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for InstantiationExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for InstantiationExpressionRef<'a> {
    type Via = ExpressionRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_expression_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for InstantiationExpressionRef<'a> {}

