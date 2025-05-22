#![allow(unused)]
use super::utils::DescendantOf;
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
use super::boolean_expression::{
    BooleanExpression, BooleanExpressionRefMut, BooleanExpressionRef,
};
use super::instantiation_expression::{
    InstantiationExpression, InstantiationExpressionRefMut, InstantiationExpressionRef,
};
use super::metadata_access_expression::{
    MetadataAccessExpression, MetadataAccessExpressionRefMut, MetadataAccessExpressionRef,
};
use super::feature_reference_expression::{
    FeatureReferenceExpression, FeatureReferenceExpressionRefMut,
    FeatureReferenceExpressionRef,
};
use super::literal_expression::{
    LiteralExpression, LiteralExpressionRefMut, LiteralExpressionRef,
};
use super::null_expression::{NullExpression, NullExpressionRefMut, NullExpressionRef};
pub struct ExpressionInner {
    pub(super) sup_step: StepInner,
}
pub trait ExpressionStruct
where
    Self: ExpressionStructRefMut,
    Self: ExpressionStructRef,
    Self: StepStruct,
{}
pub trait ExpressionStructRefMut
where
    Self: ExpressionStructRef,
    Self: StepStructRefMut,
{}
pub trait ExpressionStructRef
where
    Self: StepStructRef,
{}
pub trait ExpressionUpcast: ExpressionStruct {
    fn into_expression(self) -> Expression;
}
pub trait ExpressionUpcastRefMut<'a>: ExpressionStructRefMut {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a>;
}
pub trait ExpressionUpcastRef<'a>: ExpressionStructRef {
    fn as_expression_ref(self) -> ExpressionRef<'a>;
}
impl ExpressionStruct for ExpressionInner {}
impl ExpressionStructRefMut for ExpressionInner {}
impl ExpressionStructRef for ExpressionInner {}
impl StepStruct for ExpressionInner {}
impl StepStructRefMut for ExpressionInner {}
impl StepStructRef for ExpressionInner {}
impl FeatureStruct for ExpressionInner {
    fn is_unique(self) -> bool {
        self.sup_step.is_unique()
    }
    fn is_ordered(self) -> bool {
        self.sup_step.is_ordered()
    }
    fn is_composite(self) -> bool {
        self.sup_step.is_composite()
    }
    fn is_end(self) -> bool {
        self.sup_step.is_end()
    }
    fn is_derived(self) -> bool {
        self.sup_step.is_derived()
    }
    fn is_portion(self) -> bool {
        self.sup_step.is_portion()
    }
    fn is_variable(self) -> bool {
        self.sup_step.is_variable()
    }
    fn is_constant(self) -> bool {
        self.sup_step.is_constant()
    }
    fn direction(
        self,
    ) -> Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_step.direction()
    }
}
impl FeatureStructRefMut for ExpressionInner {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_unique_ref_mut()
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_ordered_ref_mut()
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_composite_ref_mut()
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_end_ref_mut()
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_derived_ref_mut()
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_portion_ref_mut()
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_variable_ref_mut()
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_constant_ref_mut()
    }
    fn direction_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_step.direction_ref_mut()
    }
}
impl FeatureStructRef for ExpressionInner {
    fn is_unique_ref(&self) -> &bool {
        self.sup_step.is_unique_ref()
    }
    fn is_ordered_ref(&self) -> &bool {
        self.sup_step.is_ordered_ref()
    }
    fn is_composite_ref(&self) -> &bool {
        self.sup_step.is_composite_ref()
    }
    fn is_end_ref(&self) -> &bool {
        self.sup_step.is_end_ref()
    }
    fn is_derived_ref(&self) -> &bool {
        self.sup_step.is_derived_ref()
    }
    fn is_portion_ref(&self) -> &bool {
        self.sup_step.is_portion_ref()
    }
    fn is_variable_ref(&self) -> &bool {
        self.sup_step.is_variable_ref()
    }
    fn is_constant_ref(&self) -> &bool {
        self.sup_step.is_constant_ref()
    }
    fn direction_ref(
        &self,
    ) -> &Option<
        std::rc::Rc<
            std::cell::RefCell<super::feature_direction_kind::FeatureDirectionKind>,
        >,
    > {
        self.sup_step.direction_ref()
    }
}
impl TypeStruct for ExpressionInner {
    fn is_abstract(self) -> bool {
        self.sup_step.is_abstract()
    }
    fn is_sufficient(self) -> bool {
        self.sup_step.is_sufficient()
    }
}
impl TypeStructRefMut for ExpressionInner {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_abstract_ref_mut()
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_sufficient_ref_mut()
    }
}
impl TypeStructRef for ExpressionInner {
    fn is_abstract_ref(&self) -> &bool {
        self.sup_step.is_abstract_ref()
    }
    fn is_sufficient_ref(&self) -> &bool {
        self.sup_step.is_sufficient_ref()
    }
}
impl NamespaceStruct for ExpressionInner {}
impl NamespaceStructRefMut for ExpressionInner {}
impl NamespaceStructRef for ExpressionInner {}
impl ElementStruct for ExpressionInner {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_step.owned_relationship()
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_step.owning_relationship()
    }
    fn element_id(self) -> String {
        self.sup_step.element_id()
    }
    fn alias_ids(self) -> Vec<String> {
        self.sup_step.alias_ids()
    }
    fn declared_short_name(self) -> Option<String> {
        self.sup_step.declared_short_name()
    }
    fn declared_name(self) -> Option<String> {
        self.sup_step.declared_name()
    }
    fn is_implied_included(self) -> bool {
        self.sup_step.is_implied_included()
    }
}
impl ElementStructRefMut for ExpressionInner {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_step.owned_relationship_ref_mut()
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        self.sup_step.owning_relationship_ref_mut()
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        self.sup_step.element_id_ref_mut()
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        self.sup_step.alias_ids_ref_mut()
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_step.declared_short_name_ref_mut()
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        self.sup_step.declared_name_ref_mut()
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        self.sup_step.is_implied_included_ref_mut()
    }
}
impl ElementStructRef for ExpressionInner {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_step.owned_relationship_ref()
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        self.sup_step.owning_relationship_ref()
    }
    fn element_id_ref(&self) -> &String {
        self.sup_step.element_id_ref()
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        self.sup_step.alias_ids_ref()
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        self.sup_step.declared_short_name_ref()
    }
    fn declared_name_ref(&self) -> &Option<String> {
        self.sup_step.declared_name_ref()
    }
    fn is_implied_included_ref(&self) -> &bool {
        self.sup_step.is_implied_included_ref()
    }
}
pub enum Expression {
    Itself(ExpressionInner),
    BooleanExpression(BooleanExpression),
    InstantiationExpression(InstantiationExpression),
    MetadataAccessExpression(MetadataAccessExpression),
    FeatureReferenceExpression(FeatureReferenceExpression),
    LiteralExpression(LiteralExpression),
    NullExpression(NullExpression),
}
pub enum ExpressionRefMut<'a> {
    Itself(&'a mut ExpressionInner),
    BooleanExpression(BooleanExpressionRefMut<'a>),
    InstantiationExpression(InstantiationExpressionRefMut<'a>),
    MetadataAccessExpression(MetadataAccessExpressionRefMut<'a>),
    FeatureReferenceExpression(FeatureReferenceExpressionRefMut<'a>),
    LiteralExpression(LiteralExpressionRefMut<'a>),
    NullExpression(NullExpressionRefMut<'a>),
}
pub enum ExpressionRef<'a> {
    Itself(&'a ExpressionInner),
    BooleanExpression(BooleanExpressionRef<'a>),
    InstantiationExpression(InstantiationExpressionRef<'a>),
    MetadataAccessExpression(MetadataAccessExpressionRef<'a>),
    FeatureReferenceExpression(FeatureReferenceExpressionRef<'a>),
    LiteralExpression(LiteralExpressionRef<'a>),
    NullExpression(NullExpressionRef<'a>),
}
impl Expression {
    pub fn as_ref(&self) -> ExpressionRef {
        match self {
            Expression::Itself(inner) => ExpressionRef::Itself(&inner),
            Expression::BooleanExpression(inner) => {
                ExpressionRef::BooleanExpression(inner.as_ref())
            }
            Expression::InstantiationExpression(inner) => {
                ExpressionRef::InstantiationExpression(inner.as_ref())
            }
            Expression::MetadataAccessExpression(inner) => {
                ExpressionRef::MetadataAccessExpression(inner.as_ref())
            }
            Expression::FeatureReferenceExpression(inner) => {
                ExpressionRef::FeatureReferenceExpression(inner.as_ref())
            }
            Expression::LiteralExpression(inner) => {
                ExpressionRef::LiteralExpression(inner.as_ref())
            }
            Expression::NullExpression(inner) => {
                ExpressionRef::NullExpression(inner.as_ref())
            }
        }
    }
    pub fn as_ref_mut(&mut self) -> ExpressionRefMut {
        match self {
            Expression::Itself(inner) => ExpressionRefMut::Itself(inner),
            Expression::BooleanExpression(inner) => {
                ExpressionRefMut::BooleanExpression(inner.as_ref_mut())
            }
            Expression::InstantiationExpression(inner) => {
                ExpressionRefMut::InstantiationExpression(inner.as_ref_mut())
            }
            Expression::MetadataAccessExpression(inner) => {
                ExpressionRefMut::MetadataAccessExpression(inner.as_ref_mut())
            }
            Expression::FeatureReferenceExpression(inner) => {
                ExpressionRefMut::FeatureReferenceExpression(inner.as_ref_mut())
            }
            Expression::LiteralExpression(inner) => {
                ExpressionRefMut::LiteralExpression(inner.as_ref_mut())
            }
            Expression::NullExpression(inner) => {
                ExpressionRefMut::NullExpression(inner.as_ref_mut())
            }
        }
    }
}
impl<'a> ExpressionRefMut<'a> {
    pub fn as_ref(self) -> ExpressionRef<'a> {
        match self {
            ExpressionRefMut::Itself(inner) => ExpressionRef::Itself(inner),
            ExpressionRefMut::BooleanExpression(inner) => {
                ExpressionRef::BooleanExpression(inner.as_ref())
            }
            ExpressionRefMut::InstantiationExpression(inner) => {
                ExpressionRef::InstantiationExpression(inner.as_ref())
            }
            ExpressionRefMut::MetadataAccessExpression(inner) => {
                ExpressionRef::MetadataAccessExpression(inner.as_ref())
            }
            ExpressionRefMut::FeatureReferenceExpression(inner) => {
                ExpressionRef::FeatureReferenceExpression(inner.as_ref())
            }
            ExpressionRefMut::LiteralExpression(inner) => {
                ExpressionRef::LiteralExpression(inner.as_ref())
            }
            ExpressionRefMut::NullExpression(inner) => {
                ExpressionRef::NullExpression(inner.as_ref())
            }
        }
    }
}
impl ExpressionStruct for Expression {}
impl ExpressionStructRefMut for Expression {}
impl ExpressionStructRef for Expression {}
impl<'a> ExpressionStructRefMut for ExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for ExpressionRefMut<'a> {}
impl<'a> ExpressionStructRef for ExpressionRef<'a> {}
impl StepStruct for Expression {}
impl StepStructRefMut for Expression {}
impl StepStructRef for Expression {}
impl<'a> StepStructRefMut for ExpressionRefMut<'a> {}
impl<'a> StepStructRef for ExpressionRefMut<'a> {}
impl<'a> StepStructRef for ExpressionRef<'a> {}
impl FeatureStruct for Expression {
    fn is_unique(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_unique(),
            Expression::BooleanExpression(x) => x.is_unique(),
            Expression::InstantiationExpression(x) => x.is_unique(),
            Expression::MetadataAccessExpression(x) => x.is_unique(),
            Expression::FeatureReferenceExpression(x) => x.is_unique(),
            Expression::LiteralExpression(x) => x.is_unique(),
            Expression::NullExpression(x) => x.is_unique(),
        }
    }
    fn is_ordered(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_ordered(),
            Expression::BooleanExpression(x) => x.is_ordered(),
            Expression::InstantiationExpression(x) => x.is_ordered(),
            Expression::MetadataAccessExpression(x) => x.is_ordered(),
            Expression::FeatureReferenceExpression(x) => x.is_ordered(),
            Expression::LiteralExpression(x) => x.is_ordered(),
            Expression::NullExpression(x) => x.is_ordered(),
        }
    }
    fn is_composite(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_composite(),
            Expression::BooleanExpression(x) => x.is_composite(),
            Expression::InstantiationExpression(x) => x.is_composite(),
            Expression::MetadataAccessExpression(x) => x.is_composite(),
            Expression::FeatureReferenceExpression(x) => x.is_composite(),
            Expression::LiteralExpression(x) => x.is_composite(),
            Expression::NullExpression(x) => x.is_composite(),
        }
    }
    fn is_end(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_end(),
            Expression::BooleanExpression(x) => x.is_end(),
            Expression::InstantiationExpression(x) => x.is_end(),
            Expression::MetadataAccessExpression(x) => x.is_end(),
            Expression::FeatureReferenceExpression(x) => x.is_end(),
            Expression::LiteralExpression(x) => x.is_end(),
            Expression::NullExpression(x) => x.is_end(),
        }
    }
    fn is_derived(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_derived(),
            Expression::BooleanExpression(x) => x.is_derived(),
            Expression::InstantiationExpression(x) => x.is_derived(),
            Expression::MetadataAccessExpression(x) => x.is_derived(),
            Expression::FeatureReferenceExpression(x) => x.is_derived(),
            Expression::LiteralExpression(x) => x.is_derived(),
            Expression::NullExpression(x) => x.is_derived(),
        }
    }
    fn is_portion(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_portion(),
            Expression::BooleanExpression(x) => x.is_portion(),
            Expression::InstantiationExpression(x) => x.is_portion(),
            Expression::MetadataAccessExpression(x) => x.is_portion(),
            Expression::FeatureReferenceExpression(x) => x.is_portion(),
            Expression::LiteralExpression(x) => x.is_portion(),
            Expression::NullExpression(x) => x.is_portion(),
        }
    }
    fn is_variable(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_variable(),
            Expression::BooleanExpression(x) => x.is_variable(),
            Expression::InstantiationExpression(x) => x.is_variable(),
            Expression::MetadataAccessExpression(x) => x.is_variable(),
            Expression::FeatureReferenceExpression(x) => x.is_variable(),
            Expression::LiteralExpression(x) => x.is_variable(),
            Expression::NullExpression(x) => x.is_variable(),
        }
    }
    fn is_constant(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_constant(),
            Expression::BooleanExpression(x) => x.is_constant(),
            Expression::InstantiationExpression(x) => x.is_constant(),
            Expression::MetadataAccessExpression(x) => x.is_constant(),
            Expression::FeatureReferenceExpression(x) => x.is_constant(),
            Expression::LiteralExpression(x) => x.is_constant(),
            Expression::NullExpression(x) => x.is_constant(),
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
            Expression::Itself(x) => x.direction(),
            Expression::BooleanExpression(x) => x.direction(),
            Expression::InstantiationExpression(x) => x.direction(),
            Expression::MetadataAccessExpression(x) => x.direction(),
            Expression::FeatureReferenceExpression(x) => x.direction(),
            Expression::LiteralExpression(x) => x.direction(),
            Expression::NullExpression(x) => x.direction(),
        }
    }
}
impl FeatureStructRefMut for Expression {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_unique_ref_mut(),
            Expression::BooleanExpression(x) => x.is_unique_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_unique_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_unique_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_unique_ref_mut(),
            Expression::LiteralExpression(x) => x.is_unique_ref_mut(),
            Expression::NullExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_ordered_ref_mut(),
            Expression::BooleanExpression(x) => x.is_ordered_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_ordered_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_ordered_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_ordered_ref_mut(),
            Expression::LiteralExpression(x) => x.is_ordered_ref_mut(),
            Expression::NullExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_composite_ref_mut(),
            Expression::BooleanExpression(x) => x.is_composite_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_composite_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_composite_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_composite_ref_mut(),
            Expression::LiteralExpression(x) => x.is_composite_ref_mut(),
            Expression::NullExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_end_ref_mut(),
            Expression::BooleanExpression(x) => x.is_end_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_end_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_end_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_end_ref_mut(),
            Expression::LiteralExpression(x) => x.is_end_ref_mut(),
            Expression::NullExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_derived_ref_mut(),
            Expression::BooleanExpression(x) => x.is_derived_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_derived_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_derived_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_derived_ref_mut(),
            Expression::LiteralExpression(x) => x.is_derived_ref_mut(),
            Expression::NullExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_portion_ref_mut(),
            Expression::BooleanExpression(x) => x.is_portion_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_portion_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_portion_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_portion_ref_mut(),
            Expression::LiteralExpression(x) => x.is_portion_ref_mut(),
            Expression::NullExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_variable_ref_mut(),
            Expression::BooleanExpression(x) => x.is_variable_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_variable_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_variable_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_variable_ref_mut(),
            Expression::LiteralExpression(x) => x.is_variable_ref_mut(),
            Expression::NullExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_constant_ref_mut(),
            Expression::BooleanExpression(x) => x.is_constant_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_constant_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_constant_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_constant_ref_mut(),
            Expression::LiteralExpression(x) => x.is_constant_ref_mut(),
            Expression::NullExpression(x) => x.is_constant_ref_mut(),
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
            Expression::Itself(x) => x.direction_ref_mut(),
            Expression::BooleanExpression(x) => x.direction_ref_mut(),
            Expression::InstantiationExpression(x) => x.direction_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.direction_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.direction_ref_mut(),
            Expression::LiteralExpression(x) => x.direction_ref_mut(),
            Expression::NullExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl FeatureStructRef for Expression {
    fn is_unique_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_unique_ref(),
            Expression::BooleanExpression(x) => x.is_unique_ref(),
            Expression::InstantiationExpression(x) => x.is_unique_ref(),
            Expression::MetadataAccessExpression(x) => x.is_unique_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_unique_ref(),
            Expression::LiteralExpression(x) => x.is_unique_ref(),
            Expression::NullExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_ordered_ref(),
            Expression::BooleanExpression(x) => x.is_ordered_ref(),
            Expression::InstantiationExpression(x) => x.is_ordered_ref(),
            Expression::MetadataAccessExpression(x) => x.is_ordered_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_ordered_ref(),
            Expression::LiteralExpression(x) => x.is_ordered_ref(),
            Expression::NullExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_composite_ref(),
            Expression::BooleanExpression(x) => x.is_composite_ref(),
            Expression::InstantiationExpression(x) => x.is_composite_ref(),
            Expression::MetadataAccessExpression(x) => x.is_composite_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_composite_ref(),
            Expression::LiteralExpression(x) => x.is_composite_ref(),
            Expression::NullExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_end_ref(),
            Expression::BooleanExpression(x) => x.is_end_ref(),
            Expression::InstantiationExpression(x) => x.is_end_ref(),
            Expression::MetadataAccessExpression(x) => x.is_end_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_end_ref(),
            Expression::LiteralExpression(x) => x.is_end_ref(),
            Expression::NullExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_derived_ref(),
            Expression::BooleanExpression(x) => x.is_derived_ref(),
            Expression::InstantiationExpression(x) => x.is_derived_ref(),
            Expression::MetadataAccessExpression(x) => x.is_derived_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_derived_ref(),
            Expression::LiteralExpression(x) => x.is_derived_ref(),
            Expression::NullExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_portion_ref(),
            Expression::BooleanExpression(x) => x.is_portion_ref(),
            Expression::InstantiationExpression(x) => x.is_portion_ref(),
            Expression::MetadataAccessExpression(x) => x.is_portion_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_portion_ref(),
            Expression::LiteralExpression(x) => x.is_portion_ref(),
            Expression::NullExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_variable_ref(),
            Expression::BooleanExpression(x) => x.is_variable_ref(),
            Expression::InstantiationExpression(x) => x.is_variable_ref(),
            Expression::MetadataAccessExpression(x) => x.is_variable_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_variable_ref(),
            Expression::LiteralExpression(x) => x.is_variable_ref(),
            Expression::NullExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_constant_ref(),
            Expression::BooleanExpression(x) => x.is_constant_ref(),
            Expression::InstantiationExpression(x) => x.is_constant_ref(),
            Expression::MetadataAccessExpression(x) => x.is_constant_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_constant_ref(),
            Expression::LiteralExpression(x) => x.is_constant_ref(),
            Expression::NullExpression(x) => x.is_constant_ref(),
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
            Expression::Itself(x) => x.direction_ref(),
            Expression::BooleanExpression(x) => x.direction_ref(),
            Expression::InstantiationExpression(x) => x.direction_ref(),
            Expression::MetadataAccessExpression(x) => x.direction_ref(),
            Expression::FeatureReferenceExpression(x) => x.direction_ref(),
            Expression::LiteralExpression(x) => x.direction_ref(),
            Expression::NullExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRefMut for ExpressionRefMut<'a> {
    fn is_unique_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_unique_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_unique_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_unique_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_unique_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_unique_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_unique_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_unique_ref_mut(),
        }
    }
    fn is_ordered_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_ordered_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_ordered_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_ordered_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_ordered_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_ordered_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_ordered_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_ordered_ref_mut(),
        }
    }
    fn is_composite_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_composite_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_composite_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_composite_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_composite_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_composite_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_composite_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_composite_ref_mut(),
        }
    }
    fn is_end_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_end_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_end_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_end_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_end_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_end_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_end_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_end_ref_mut(),
        }
    }
    fn is_derived_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_derived_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_derived_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_derived_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_derived_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_derived_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_derived_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_derived_ref_mut(),
        }
    }
    fn is_portion_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_portion_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_portion_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_portion_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_portion_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_portion_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_portion_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_portion_ref_mut(),
        }
    }
    fn is_variable_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_variable_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_variable_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_variable_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_variable_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_variable_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_variable_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_variable_ref_mut(),
        }
    }
    fn is_constant_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_constant_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_constant_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_constant_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_constant_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_constant_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_constant_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_constant_ref_mut(),
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
            ExpressionRefMut::Itself(x) => x.direction_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.direction_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.direction_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.direction_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.direction_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.direction_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.direction_ref_mut(),
        }
    }
}
impl<'a> FeatureStructRef for ExpressionRefMut<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_unique_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_unique_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_unique_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_unique_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_unique_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_unique_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_ordered_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_ordered_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_ordered_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_ordered_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_ordered_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_ordered_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_composite_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_composite_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_composite_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_composite_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_composite_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_composite_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_end_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_end_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_end_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_end_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_end_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_end_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_derived_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_derived_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_derived_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_derived_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_derived_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_derived_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_portion_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_portion_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_portion_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_portion_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_portion_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_portion_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_variable_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_variable_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_variable_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_variable_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_variable_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_variable_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_constant_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_constant_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_constant_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_constant_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_constant_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_constant_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_constant_ref(),
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
            ExpressionRefMut::Itself(x) => x.direction_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.direction_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.direction_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.direction_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.direction_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.direction_ref(),
            ExpressionRefMut::NullExpression(x) => x.direction_ref(),
        }
    }
}
impl<'a> FeatureStructRef for ExpressionRef<'a> {
    fn is_unique_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_unique_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_unique_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_unique_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_unique_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_unique_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_unique_ref(),
            ExpressionRef::NullExpression(x) => x.is_unique_ref(),
        }
    }
    fn is_ordered_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_ordered_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_ordered_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_ordered_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_ordered_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_ordered_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_ordered_ref(),
            ExpressionRef::NullExpression(x) => x.is_ordered_ref(),
        }
    }
    fn is_composite_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_composite_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_composite_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_composite_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_composite_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_composite_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_composite_ref(),
            ExpressionRef::NullExpression(x) => x.is_composite_ref(),
        }
    }
    fn is_end_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_end_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_end_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_end_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_end_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_end_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_end_ref(),
            ExpressionRef::NullExpression(x) => x.is_end_ref(),
        }
    }
    fn is_derived_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_derived_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_derived_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_derived_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_derived_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_derived_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_derived_ref(),
            ExpressionRef::NullExpression(x) => x.is_derived_ref(),
        }
    }
    fn is_portion_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_portion_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_portion_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_portion_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_portion_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_portion_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_portion_ref(),
            ExpressionRef::NullExpression(x) => x.is_portion_ref(),
        }
    }
    fn is_variable_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_variable_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_variable_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_variable_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_variable_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_variable_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_variable_ref(),
            ExpressionRef::NullExpression(x) => x.is_variable_ref(),
        }
    }
    fn is_constant_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_constant_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_constant_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_constant_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_constant_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_constant_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_constant_ref(),
            ExpressionRef::NullExpression(x) => x.is_constant_ref(),
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
            ExpressionRef::Itself(x) => x.direction_ref(),
            ExpressionRef::BooleanExpression(x) => x.direction_ref(),
            ExpressionRef::InstantiationExpression(x) => x.direction_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.direction_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.direction_ref(),
            ExpressionRef::LiteralExpression(x) => x.direction_ref(),
            ExpressionRef::NullExpression(x) => x.direction_ref(),
        }
    }
}
impl TypeStruct for Expression {
    fn is_abstract(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_abstract(),
            Expression::BooleanExpression(x) => x.is_abstract(),
            Expression::InstantiationExpression(x) => x.is_abstract(),
            Expression::MetadataAccessExpression(x) => x.is_abstract(),
            Expression::FeatureReferenceExpression(x) => x.is_abstract(),
            Expression::LiteralExpression(x) => x.is_abstract(),
            Expression::NullExpression(x) => x.is_abstract(),
        }
    }
    fn is_sufficient(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_sufficient(),
            Expression::BooleanExpression(x) => x.is_sufficient(),
            Expression::InstantiationExpression(x) => x.is_sufficient(),
            Expression::MetadataAccessExpression(x) => x.is_sufficient(),
            Expression::FeatureReferenceExpression(x) => x.is_sufficient(),
            Expression::LiteralExpression(x) => x.is_sufficient(),
            Expression::NullExpression(x) => x.is_sufficient(),
        }
    }
}
impl TypeStructRefMut for Expression {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_abstract_ref_mut(),
            Expression::BooleanExpression(x) => x.is_abstract_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_abstract_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_abstract_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_abstract_ref_mut(),
            Expression::LiteralExpression(x) => x.is_abstract_ref_mut(),
            Expression::NullExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_sufficient_ref_mut(),
            Expression::BooleanExpression(x) => x.is_sufficient_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_sufficient_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_sufficient_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_sufficient_ref_mut(),
            Expression::LiteralExpression(x) => x.is_sufficient_ref_mut(),
            Expression::NullExpression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl TypeStructRef for Expression {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_abstract_ref(),
            Expression::BooleanExpression(x) => x.is_abstract_ref(),
            Expression::InstantiationExpression(x) => x.is_abstract_ref(),
            Expression::MetadataAccessExpression(x) => x.is_abstract_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_abstract_ref(),
            Expression::LiteralExpression(x) => x.is_abstract_ref(),
            Expression::NullExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_sufficient_ref(),
            Expression::BooleanExpression(x) => x.is_sufficient_ref(),
            Expression::InstantiationExpression(x) => x.is_sufficient_ref(),
            Expression::MetadataAccessExpression(x) => x.is_sufficient_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_sufficient_ref(),
            Expression::LiteralExpression(x) => x.is_sufficient_ref(),
            Expression::NullExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRefMut for ExpressionRefMut<'a> {
    fn is_abstract_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_abstract_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_abstract_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_abstract_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_abstract_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_abstract_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_abstract_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_abstract_ref_mut(),
        }
    }
    fn is_sufficient_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_sufficient_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_sufficient_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_sufficient_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_sufficient_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_sufficient_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.is_sufficient_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_sufficient_ref_mut(),
        }
    }
}
impl<'a> TypeStructRef for ExpressionRefMut<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_abstract_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_abstract_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_abstract_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_abstract_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_abstract_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_abstract_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_sufficient_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_sufficient_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_sufficient_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_sufficient_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.is_sufficient_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.is_sufficient_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl<'a> TypeStructRef for ExpressionRef<'a> {
    fn is_abstract_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_abstract_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_abstract_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_abstract_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_abstract_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_abstract_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_abstract_ref(),
            ExpressionRef::NullExpression(x) => x.is_abstract_ref(),
        }
    }
    fn is_sufficient_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_sufficient_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_sufficient_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_sufficient_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_sufficient_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_sufficient_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_sufficient_ref(),
            ExpressionRef::NullExpression(x) => x.is_sufficient_ref(),
        }
    }
}
impl NamespaceStruct for Expression {}
impl NamespaceStructRefMut for Expression {}
impl NamespaceStructRef for Expression {}
impl<'a> NamespaceStructRefMut for ExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for ExpressionRefMut<'a> {}
impl<'a> NamespaceStructRef for ExpressionRef<'a> {}
impl ElementStruct for Expression {
    fn owned_relationship(
        self,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Expression::Itself(x) => x.owned_relationship(),
            Expression::BooleanExpression(x) => x.owned_relationship(),
            Expression::InstantiationExpression(x) => x.owned_relationship(),
            Expression::MetadataAccessExpression(x) => x.owned_relationship(),
            Expression::FeatureReferenceExpression(x) => x.owned_relationship(),
            Expression::LiteralExpression(x) => x.owned_relationship(),
            Expression::NullExpression(x) => x.owned_relationship(),
        }
    }
    fn owning_relationship(
        self,
    ) -> Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Expression::Itself(x) => x.owning_relationship(),
            Expression::BooleanExpression(x) => x.owning_relationship(),
            Expression::InstantiationExpression(x) => x.owning_relationship(),
            Expression::MetadataAccessExpression(x) => x.owning_relationship(),
            Expression::FeatureReferenceExpression(x) => x.owning_relationship(),
            Expression::LiteralExpression(x) => x.owning_relationship(),
            Expression::NullExpression(x) => x.owning_relationship(),
        }
    }
    fn element_id(self) -> String {
        match self {
            Expression::Itself(x) => x.element_id(),
            Expression::BooleanExpression(x) => x.element_id(),
            Expression::InstantiationExpression(x) => x.element_id(),
            Expression::MetadataAccessExpression(x) => x.element_id(),
            Expression::FeatureReferenceExpression(x) => x.element_id(),
            Expression::LiteralExpression(x) => x.element_id(),
            Expression::NullExpression(x) => x.element_id(),
        }
    }
    fn alias_ids(self) -> Vec<String> {
        match self {
            Expression::Itself(x) => x.alias_ids(),
            Expression::BooleanExpression(x) => x.alias_ids(),
            Expression::InstantiationExpression(x) => x.alias_ids(),
            Expression::MetadataAccessExpression(x) => x.alias_ids(),
            Expression::FeatureReferenceExpression(x) => x.alias_ids(),
            Expression::LiteralExpression(x) => x.alias_ids(),
            Expression::NullExpression(x) => x.alias_ids(),
        }
    }
    fn declared_short_name(self) -> Option<String> {
        match self {
            Expression::Itself(x) => x.declared_short_name(),
            Expression::BooleanExpression(x) => x.declared_short_name(),
            Expression::InstantiationExpression(x) => x.declared_short_name(),
            Expression::MetadataAccessExpression(x) => x.declared_short_name(),
            Expression::FeatureReferenceExpression(x) => x.declared_short_name(),
            Expression::LiteralExpression(x) => x.declared_short_name(),
            Expression::NullExpression(x) => x.declared_short_name(),
        }
    }
    fn declared_name(self) -> Option<String> {
        match self {
            Expression::Itself(x) => x.declared_name(),
            Expression::BooleanExpression(x) => x.declared_name(),
            Expression::InstantiationExpression(x) => x.declared_name(),
            Expression::MetadataAccessExpression(x) => x.declared_name(),
            Expression::FeatureReferenceExpression(x) => x.declared_name(),
            Expression::LiteralExpression(x) => x.declared_name(),
            Expression::NullExpression(x) => x.declared_name(),
        }
    }
    fn is_implied_included(self) -> bool {
        match self {
            Expression::Itself(x) => x.is_implied_included(),
            Expression::BooleanExpression(x) => x.is_implied_included(),
            Expression::InstantiationExpression(x) => x.is_implied_included(),
            Expression::MetadataAccessExpression(x) => x.is_implied_included(),
            Expression::FeatureReferenceExpression(x) => x.is_implied_included(),
            Expression::LiteralExpression(x) => x.is_implied_included(),
            Expression::NullExpression(x) => x.is_implied_included(),
        }
    }
}
impl ElementStructRefMut for Expression {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Expression::Itself(x) => x.owned_relationship_ref_mut(),
            Expression::BooleanExpression(x) => x.owned_relationship_ref_mut(),
            Expression::InstantiationExpression(x) => x.owned_relationship_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.owned_relationship_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.owned_relationship_ref_mut(),
            Expression::LiteralExpression(x) => x.owned_relationship_ref_mut(),
            Expression::NullExpression(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            Expression::Itself(x) => x.owning_relationship_ref_mut(),
            Expression::BooleanExpression(x) => x.owning_relationship_ref_mut(),
            Expression::InstantiationExpression(x) => x.owning_relationship_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.owning_relationship_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.owning_relationship_ref_mut(),
            Expression::LiteralExpression(x) => x.owning_relationship_ref_mut(),
            Expression::NullExpression(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            Expression::Itself(x) => x.element_id_ref_mut(),
            Expression::BooleanExpression(x) => x.element_id_ref_mut(),
            Expression::InstantiationExpression(x) => x.element_id_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.element_id_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.element_id_ref_mut(),
            Expression::LiteralExpression(x) => x.element_id_ref_mut(),
            Expression::NullExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            Expression::Itself(x) => x.alias_ids_ref_mut(),
            Expression::BooleanExpression(x) => x.alias_ids_ref_mut(),
            Expression::InstantiationExpression(x) => x.alias_ids_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.alias_ids_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.alias_ids_ref_mut(),
            Expression::LiteralExpression(x) => x.alias_ids_ref_mut(),
            Expression::NullExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Expression::Itself(x) => x.declared_short_name_ref_mut(),
            Expression::BooleanExpression(x) => x.declared_short_name_ref_mut(),
            Expression::InstantiationExpression(x) => x.declared_short_name_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.declared_short_name_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.declared_short_name_ref_mut(),
            Expression::LiteralExpression(x) => x.declared_short_name_ref_mut(),
            Expression::NullExpression(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            Expression::Itself(x) => x.declared_name_ref_mut(),
            Expression::BooleanExpression(x) => x.declared_name_ref_mut(),
            Expression::InstantiationExpression(x) => x.declared_name_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.declared_name_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.declared_name_ref_mut(),
            Expression::LiteralExpression(x) => x.declared_name_ref_mut(),
            Expression::NullExpression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            Expression::Itself(x) => x.is_implied_included_ref_mut(),
            Expression::BooleanExpression(x) => x.is_implied_included_ref_mut(),
            Expression::InstantiationExpression(x) => x.is_implied_included_ref_mut(),
            Expression::MetadataAccessExpression(x) => x.is_implied_included_ref_mut(),
            Expression::FeatureReferenceExpression(x) => x.is_implied_included_ref_mut(),
            Expression::LiteralExpression(x) => x.is_implied_included_ref_mut(),
            Expression::NullExpression(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl ElementStructRef for Expression {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Expression::Itself(x) => x.owned_relationship_ref(),
            Expression::BooleanExpression(x) => x.owned_relationship_ref(),
            Expression::InstantiationExpression(x) => x.owned_relationship_ref(),
            Expression::MetadataAccessExpression(x) => x.owned_relationship_ref(),
            Expression::FeatureReferenceExpression(x) => x.owned_relationship_ref(),
            Expression::LiteralExpression(x) => x.owned_relationship_ref(),
            Expression::NullExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            Expression::Itself(x) => x.owning_relationship_ref(),
            Expression::BooleanExpression(x) => x.owning_relationship_ref(),
            Expression::InstantiationExpression(x) => x.owning_relationship_ref(),
            Expression::MetadataAccessExpression(x) => x.owning_relationship_ref(),
            Expression::FeatureReferenceExpression(x) => x.owning_relationship_ref(),
            Expression::LiteralExpression(x) => x.owning_relationship_ref(),
            Expression::NullExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            Expression::Itself(x) => x.element_id_ref(),
            Expression::BooleanExpression(x) => x.element_id_ref(),
            Expression::InstantiationExpression(x) => x.element_id_ref(),
            Expression::MetadataAccessExpression(x) => x.element_id_ref(),
            Expression::FeatureReferenceExpression(x) => x.element_id_ref(),
            Expression::LiteralExpression(x) => x.element_id_ref(),
            Expression::NullExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            Expression::Itself(x) => x.alias_ids_ref(),
            Expression::BooleanExpression(x) => x.alias_ids_ref(),
            Expression::InstantiationExpression(x) => x.alias_ids_ref(),
            Expression::MetadataAccessExpression(x) => x.alias_ids_ref(),
            Expression::FeatureReferenceExpression(x) => x.alias_ids_ref(),
            Expression::LiteralExpression(x) => x.alias_ids_ref(),
            Expression::NullExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            Expression::Itself(x) => x.declared_short_name_ref(),
            Expression::BooleanExpression(x) => x.declared_short_name_ref(),
            Expression::InstantiationExpression(x) => x.declared_short_name_ref(),
            Expression::MetadataAccessExpression(x) => x.declared_short_name_ref(),
            Expression::FeatureReferenceExpression(x) => x.declared_short_name_ref(),
            Expression::LiteralExpression(x) => x.declared_short_name_ref(),
            Expression::NullExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            Expression::Itself(x) => x.declared_name_ref(),
            Expression::BooleanExpression(x) => x.declared_name_ref(),
            Expression::InstantiationExpression(x) => x.declared_name_ref(),
            Expression::MetadataAccessExpression(x) => x.declared_name_ref(),
            Expression::FeatureReferenceExpression(x) => x.declared_name_ref(),
            Expression::LiteralExpression(x) => x.declared_name_ref(),
            Expression::NullExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            Expression::Itself(x) => x.is_implied_included_ref(),
            Expression::BooleanExpression(x) => x.is_implied_included_ref(),
            Expression::InstantiationExpression(x) => x.is_implied_included_ref(),
            Expression::MetadataAccessExpression(x) => x.is_implied_included_ref(),
            Expression::FeatureReferenceExpression(x) => x.is_implied_included_ref(),
            Expression::LiteralExpression(x) => x.is_implied_included_ref(),
            Expression::NullExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRefMut for ExpressionRefMut<'a> {
    fn owned_relationship_ref_mut(
        &mut self,
    ) -> &mut Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ExpressionRefMut::Itself(x) => x.owned_relationship_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.owned_relationship_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            ExpressionRefMut::MetadataAccessExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.owned_relationship_ref_mut()
            }
            ExpressionRefMut::LiteralExpression(x) => x.owned_relationship_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.owned_relationship_ref_mut(),
        }
    }
    fn owning_relationship_ref_mut(
        &mut self,
    ) -> &mut Option<
        std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>,
    > {
        match self {
            ExpressionRefMut::Itself(x) => x.owning_relationship_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.owning_relationship_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            ExpressionRefMut::MetadataAccessExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.owning_relationship_ref_mut()
            }
            ExpressionRefMut::LiteralExpression(x) => x.owning_relationship_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.owning_relationship_ref_mut(),
        }
    }
    fn element_id_ref_mut(&mut self) -> &mut String {
        match self {
            ExpressionRefMut::Itself(x) => x.element_id_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.element_id_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.element_id_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.element_id_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.element_id_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.element_id_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.element_id_ref_mut(),
        }
    }
    fn alias_ids_ref_mut(&mut self) -> &mut Vec<String> {
        match self {
            ExpressionRefMut::Itself(x) => x.alias_ids_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.alias_ids_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.alias_ids_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.alias_ids_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.alias_ids_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.alias_ids_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.alias_ids_ref_mut(),
        }
    }
    fn declared_short_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ExpressionRefMut::Itself(x) => x.declared_short_name_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.declared_short_name_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            ExpressionRefMut::MetadataAccessExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.declared_short_name_ref_mut()
            }
            ExpressionRefMut::LiteralExpression(x) => x.declared_short_name_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.declared_short_name_ref_mut(),
        }
    }
    fn declared_name_ref_mut(&mut self) -> &mut Option<String> {
        match self {
            ExpressionRefMut::Itself(x) => x.declared_name_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.declared_name_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => x.declared_name_ref_mut(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.declared_name_ref_mut(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.declared_name_ref_mut(),
            ExpressionRefMut::LiteralExpression(x) => x.declared_name_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.declared_name_ref_mut(),
        }
    }
    fn is_implied_included_ref_mut(&mut self) -> &mut bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_implied_included_ref_mut(),
            ExpressionRefMut::BooleanExpression(x) => x.is_implied_included_ref_mut(),
            ExpressionRefMut::InstantiationExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            ExpressionRefMut::MetadataAccessExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.is_implied_included_ref_mut()
            }
            ExpressionRefMut::LiteralExpression(x) => x.is_implied_included_ref_mut(),
            ExpressionRefMut::NullExpression(x) => x.is_implied_included_ref_mut(),
        }
    }
}
impl<'a> ElementStructRef for ExpressionRefMut<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ExpressionRefMut::Itself(x) => x.owned_relationship_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.owned_relationship_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.owned_relationship_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.owned_relationship_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.owned_relationship_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.owned_relationship_ref(),
            ExpressionRefMut::NullExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ExpressionRefMut::Itself(x) => x.owning_relationship_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.owning_relationship_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.owning_relationship_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.owning_relationship_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.owning_relationship_ref()
            }
            ExpressionRefMut::LiteralExpression(x) => x.owning_relationship_ref(),
            ExpressionRefMut::NullExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ExpressionRefMut::Itself(x) => x.element_id_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.element_id_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.element_id_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.element_id_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.element_id_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.element_id_ref(),
            ExpressionRefMut::NullExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ExpressionRefMut::Itself(x) => x.alias_ids_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.alias_ids_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.alias_ids_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.alias_ids_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.alias_ids_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.alias_ids_ref(),
            ExpressionRefMut::NullExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ExpressionRefMut::Itself(x) => x.declared_short_name_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.declared_short_name_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.declared_short_name_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.declared_short_name_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.declared_short_name_ref()
            }
            ExpressionRefMut::LiteralExpression(x) => x.declared_short_name_ref(),
            ExpressionRefMut::NullExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ExpressionRefMut::Itself(x) => x.declared_name_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.declared_name_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.declared_name_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.declared_name_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => x.declared_name_ref(),
            ExpressionRefMut::LiteralExpression(x) => x.declared_name_ref(),
            ExpressionRefMut::NullExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ExpressionRefMut::Itself(x) => x.is_implied_included_ref(),
            ExpressionRefMut::BooleanExpression(x) => x.is_implied_included_ref(),
            ExpressionRefMut::InstantiationExpression(x) => x.is_implied_included_ref(),
            ExpressionRefMut::MetadataAccessExpression(x) => x.is_implied_included_ref(),
            ExpressionRefMut::FeatureReferenceExpression(x) => {
                x.is_implied_included_ref()
            }
            ExpressionRefMut::LiteralExpression(x) => x.is_implied_included_ref(),
            ExpressionRefMut::NullExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl<'a> ElementStructRef for ExpressionRef<'a> {
    fn owned_relationship_ref(
        &self,
    ) -> &Vec<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ExpressionRef::Itself(x) => x.owned_relationship_ref(),
            ExpressionRef::BooleanExpression(x) => x.owned_relationship_ref(),
            ExpressionRef::InstantiationExpression(x) => x.owned_relationship_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.owned_relationship_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.owned_relationship_ref(),
            ExpressionRef::LiteralExpression(x) => x.owned_relationship_ref(),
            ExpressionRef::NullExpression(x) => x.owned_relationship_ref(),
        }
    }
    fn owning_relationship_ref(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<super::relationship::Relationship>>> {
        match self {
            ExpressionRef::Itself(x) => x.owning_relationship_ref(),
            ExpressionRef::BooleanExpression(x) => x.owning_relationship_ref(),
            ExpressionRef::InstantiationExpression(x) => x.owning_relationship_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.owning_relationship_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.owning_relationship_ref(),
            ExpressionRef::LiteralExpression(x) => x.owning_relationship_ref(),
            ExpressionRef::NullExpression(x) => x.owning_relationship_ref(),
        }
    }
    fn element_id_ref(&self) -> &String {
        match self {
            ExpressionRef::Itself(x) => x.element_id_ref(),
            ExpressionRef::BooleanExpression(x) => x.element_id_ref(),
            ExpressionRef::InstantiationExpression(x) => x.element_id_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.element_id_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.element_id_ref(),
            ExpressionRef::LiteralExpression(x) => x.element_id_ref(),
            ExpressionRef::NullExpression(x) => x.element_id_ref(),
        }
    }
    fn alias_ids_ref(&self) -> &Vec<String> {
        match self {
            ExpressionRef::Itself(x) => x.alias_ids_ref(),
            ExpressionRef::BooleanExpression(x) => x.alias_ids_ref(),
            ExpressionRef::InstantiationExpression(x) => x.alias_ids_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.alias_ids_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.alias_ids_ref(),
            ExpressionRef::LiteralExpression(x) => x.alias_ids_ref(),
            ExpressionRef::NullExpression(x) => x.alias_ids_ref(),
        }
    }
    fn declared_short_name_ref(&self) -> &Option<String> {
        match self {
            ExpressionRef::Itself(x) => x.declared_short_name_ref(),
            ExpressionRef::BooleanExpression(x) => x.declared_short_name_ref(),
            ExpressionRef::InstantiationExpression(x) => x.declared_short_name_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.declared_short_name_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.declared_short_name_ref(),
            ExpressionRef::LiteralExpression(x) => x.declared_short_name_ref(),
            ExpressionRef::NullExpression(x) => x.declared_short_name_ref(),
        }
    }
    fn declared_name_ref(&self) -> &Option<String> {
        match self {
            ExpressionRef::Itself(x) => x.declared_name_ref(),
            ExpressionRef::BooleanExpression(x) => x.declared_name_ref(),
            ExpressionRef::InstantiationExpression(x) => x.declared_name_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.declared_name_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.declared_name_ref(),
            ExpressionRef::LiteralExpression(x) => x.declared_name_ref(),
            ExpressionRef::NullExpression(x) => x.declared_name_ref(),
        }
    }
    fn is_implied_included_ref(&self) -> &bool {
        match self {
            ExpressionRef::Itself(x) => x.is_implied_included_ref(),
            ExpressionRef::BooleanExpression(x) => x.is_implied_included_ref(),
            ExpressionRef::InstantiationExpression(x) => x.is_implied_included_ref(),
            ExpressionRef::MetadataAccessExpression(x) => x.is_implied_included_ref(),
            ExpressionRef::FeatureReferenceExpression(x) => x.is_implied_included_ref(),
            ExpressionRef::LiteralExpression(x) => x.is_implied_included_ref(),
            ExpressionRef::NullExpression(x) => x.is_implied_included_ref(),
        }
    }
}
impl ExpressionUpcast for Expression {
    fn into_expression(self) -> Expression {
        self
    }
}
impl<'a> ExpressionUpcastRefMut<'a> for ExpressionRefMut<'a> {
    fn as_expression_ref_mut(self) -> ExpressionRefMut<'a> {
        self
    }
}
impl<'a> ExpressionUpcastRef<'a> for ExpressionRef<'a> {
    fn as_expression_ref(self) -> ExpressionRef<'a> {
        self
    }
}
impl StepUpcast for Expression {
    fn into_step(self) -> Step {
        Step::Expression(self).into_step()
    }
}
impl<'a> StepUpcastRefMut<'a> for ExpressionRefMut<'a> {
    fn as_step_ref_mut(self) -> StepRefMut<'a> {
        StepRefMut::Expression(self).as_step_ref_mut()
    }
}
impl<'a> StepUpcastRef<'a> for ExpressionRef<'a> {
    fn as_step_ref(self) -> StepRef<'a> {
        StepRef::Expression(self).as_step_ref()
    }
}
impl FeatureUpcast for Expression {
    fn into_feature(self) -> Feature {
        Step::Expression(self).into_feature()
    }
}
impl<'a> FeatureUpcastRefMut<'a> for ExpressionRefMut<'a> {
    fn as_feature_ref_mut(self) -> FeatureRefMut<'a> {
        StepRefMut::Expression(self).as_feature_ref_mut()
    }
}
impl<'a> FeatureUpcastRef<'a> for ExpressionRef<'a> {
    fn as_feature_ref(self) -> FeatureRef<'a> {
        StepRef::Expression(self).as_feature_ref()
    }
}
impl TypeUpcast for Expression {
    fn into_type_(self) -> Type {
        Step::Expression(self).into_type_()
    }
}
impl<'a> TypeUpcastRefMut<'a> for ExpressionRefMut<'a> {
    fn as_type_ref_mut(self) -> TypeRefMut<'a> {
        StepRefMut::Expression(self).as_type_ref_mut()
    }
}
impl<'a> TypeUpcastRef<'a> for ExpressionRef<'a> {
    fn as_type_ref(self) -> TypeRef<'a> {
        StepRef::Expression(self).as_type_ref()
    }
}
impl NamespaceUpcast for Expression {
    fn into_namespace(self) -> Namespace {
        Step::Expression(self).into_namespace()
    }
}
impl<'a> NamespaceUpcastRefMut<'a> for ExpressionRefMut<'a> {
    fn as_namespace_ref_mut(self) -> NamespaceRefMut<'a> {
        StepRefMut::Expression(self).as_namespace_ref_mut()
    }
}
impl<'a> NamespaceUpcastRef<'a> for ExpressionRef<'a> {
    fn as_namespace_ref(self) -> NamespaceRef<'a> {
        StepRef::Expression(self).as_namespace_ref()
    }
}
impl ElementUpcast for Expression {
    fn into_element(self) -> Element {
        Step::Expression(self).into_element()
    }
}
impl<'a> ElementUpcastRefMut<'a> for ExpressionRefMut<'a> {
    fn as_element_ref_mut(self) -> ElementRefMut<'a> {
        StepRefMut::Expression(self).as_element_ref_mut()
    }
}
impl<'a> ElementUpcastRef<'a> for ExpressionRef<'a> {
    fn as_element_ref(self) -> ElementRef<'a> {
        StepRef::Expression(self).as_element_ref()
    }
}
pub trait ExpressionDowncast {
    fn try_into_boolean_expression(self) -> Result<BooleanExpression, String>;
    fn try_into_instantiation_expression(
        self,
    ) -> Result<InstantiationExpression, String>;
    fn try_into_metadata_access_expression(
        self,
    ) -> Result<MetadataAccessExpression, String>;
    fn try_into_feature_reference_expression(
        self,
    ) -> Result<FeatureReferenceExpression, String>;
    fn try_into_literal_expression(self) -> Result<LiteralExpression, String>;
    fn try_into_null_expression(self) -> Result<NullExpression, String>;
}
pub trait ExpressionDowncastRefMut<'a> {
    fn try_as_boolean_expression_ref_mut(
        self,
    ) -> Result<BooleanExpressionRefMut<'a>, String>;
    fn try_as_instantiation_expression_ref_mut(
        self,
    ) -> Result<InstantiationExpressionRefMut<'a>, String>;
    fn try_as_metadata_access_expression_ref_mut(
        self,
    ) -> Result<MetadataAccessExpressionRefMut<'a>, String>;
    fn try_as_feature_reference_expression_ref_mut(
        self,
    ) -> Result<FeatureReferenceExpressionRefMut<'a>, String>;
    fn try_as_literal_expression_ref_mut(
        self,
    ) -> Result<LiteralExpressionRefMut<'a>, String>;
    fn try_as_null_expression_ref_mut(self) -> Result<NullExpressionRefMut<'a>, String>;
}
pub trait ExpressionDowncastRef<'a> {
    fn try_as_boolean_expression_ref(self) -> Result<BooleanExpressionRef<'a>, String>;
    fn try_as_instantiation_expression_ref(
        self,
    ) -> Result<InstantiationExpressionRef<'a>, String>;
    fn try_as_metadata_access_expression_ref(
        self,
    ) -> Result<MetadataAccessExpressionRef<'a>, String>;
    fn try_as_feature_reference_expression_ref(
        self,
    ) -> Result<FeatureReferenceExpressionRef<'a>, String>;
    fn try_as_literal_expression_ref(self) -> Result<LiteralExpressionRef<'a>, String>;
    fn try_as_null_expression_ref(self) -> Result<NullExpressionRef<'a>, String>;
}
impl ExpressionDowncast for Expression {
    fn try_into_boolean_expression(self) -> Result<BooleanExpression, String> {
        match self {
            Expression::BooleanExpression(e) => Ok(e),
            _ => Err("Not a BooleanExpression".into()),
        }
    }
    fn try_into_instantiation_expression(
        self,
    ) -> Result<InstantiationExpression, String> {
        match self {
            Expression::InstantiationExpression(e) => Ok(e),
            _ => Err("Not a InstantiationExpression".into()),
        }
    }
    fn try_into_metadata_access_expression(
        self,
    ) -> Result<MetadataAccessExpression, String> {
        match self {
            Expression::MetadataAccessExpression(e) => Ok(e),
            _ => Err("Not a MetadataAccessExpression".into()),
        }
    }
    fn try_into_feature_reference_expression(
        self,
    ) -> Result<FeatureReferenceExpression, String> {
        match self {
            Expression::FeatureReferenceExpression(e) => Ok(e),
            _ => Err("Not a FeatureReferenceExpression".into()),
        }
    }
    fn try_into_literal_expression(self) -> Result<LiteralExpression, String> {
        match self {
            Expression::LiteralExpression(e) => Ok(e),
            _ => Err("Not a LiteralExpression".into()),
        }
    }
    fn try_into_null_expression(self) -> Result<NullExpression, String> {
        match self {
            Expression::NullExpression(e) => Ok(e),
            _ => Err("Not a NullExpression".into()),
        }
    }
}
impl<'a> ExpressionDowncastRefMut<'a> for ExpressionRefMut<'a> {
    fn try_as_boolean_expression_ref_mut(
        self,
    ) -> Result<BooleanExpressionRefMut<'a>, String> {
        match self {
            ExpressionRefMut::BooleanExpression(e) => Ok(e),
            _ => Err("Not a BooleanExpression".into()),
        }
    }
    fn try_as_instantiation_expression_ref_mut(
        self,
    ) -> Result<InstantiationExpressionRefMut<'a>, String> {
        match self {
            ExpressionRefMut::InstantiationExpression(e) => Ok(e),
            _ => Err("Not a InstantiationExpression".into()),
        }
    }
    fn try_as_metadata_access_expression_ref_mut(
        self,
    ) -> Result<MetadataAccessExpressionRefMut<'a>, String> {
        match self {
            ExpressionRefMut::MetadataAccessExpression(e) => Ok(e),
            _ => Err("Not a MetadataAccessExpression".into()),
        }
    }
    fn try_as_feature_reference_expression_ref_mut(
        self,
    ) -> Result<FeatureReferenceExpressionRefMut<'a>, String> {
        match self {
            ExpressionRefMut::FeatureReferenceExpression(e) => Ok(e),
            _ => Err("Not a FeatureReferenceExpression".into()),
        }
    }
    fn try_as_literal_expression_ref_mut(
        self,
    ) -> Result<LiteralExpressionRefMut<'a>, String> {
        match self {
            ExpressionRefMut::LiteralExpression(e) => Ok(e),
            _ => Err("Not a LiteralExpression".into()),
        }
    }
    fn try_as_null_expression_ref_mut(self) -> Result<NullExpressionRefMut<'a>, String> {
        match self {
            ExpressionRefMut::NullExpression(e) => Ok(e),
            _ => Err("Not a NullExpression".into()),
        }
    }
}
impl<'a> ExpressionDowncastRef<'a> for ExpressionRef<'a> {
    fn try_as_boolean_expression_ref(self) -> Result<BooleanExpressionRef<'a>, String> {
        match self {
            ExpressionRef::BooleanExpression(e) => Ok(e),
            _ => Err("Not a BooleanExpression".into()),
        }
    }
    fn try_as_instantiation_expression_ref(
        self,
    ) -> Result<InstantiationExpressionRef<'a>, String> {
        match self {
            ExpressionRef::InstantiationExpression(e) => Ok(e),
            _ => Err("Not a InstantiationExpression".into()),
        }
    }
    fn try_as_metadata_access_expression_ref(
        self,
    ) -> Result<MetadataAccessExpressionRef<'a>, String> {
        match self {
            ExpressionRef::MetadataAccessExpression(e) => Ok(e),
            _ => Err("Not a MetadataAccessExpression".into()),
        }
    }
    fn try_as_feature_reference_expression_ref(
        self,
    ) -> Result<FeatureReferenceExpressionRef<'a>, String> {
        match self {
            ExpressionRef::FeatureReferenceExpression(e) => Ok(e),
            _ => Err("Not a FeatureReferenceExpression".into()),
        }
    }
    fn try_as_literal_expression_ref(self) -> Result<LiteralExpressionRef<'a>, String> {
        match self {
            ExpressionRef::LiteralExpression(e) => Ok(e),
            _ => Err("Not a LiteralExpression".into()),
        }
    }
    fn try_as_null_expression_ref(self) -> Result<NullExpressionRef<'a>, String> {
        match self {
            ExpressionRef::NullExpression(e) => Ok(e),
            _ => Err("Not a NullExpression".into()),
        }
    }
}
pub trait ExpressionMethodsDescendants
where
    Self: DescendantOf<Expression>,
    Self::Via: ExpressionMethods,
    Self: Sized,
{}
pub trait ExpressionMethods: Sized {}
impl<T: ExpressionMethodsDescendants> ExpressionMethods for T
where
    T::Via: ExpressionMethods,
{}
impl DescendantOf<Step> for Expression {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl StepMethodsDescendants for Expression {}
impl DescendantOf<Feature> for Expression {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl FeatureMethodsDescendants for Expression {}
impl DescendantOf<Type> for Expression {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl TypeMethodsDescendants for Expression {}
impl DescendantOf<Namespace> for Expression {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl NamespaceMethodsDescendants for Expression {}
impl DescendantOf<Element> for Expression {
    type Via = Step;
    fn into_via(self) -> Self::Via {
        self.into_step()
    }
}
impl ElementMethodsDescendants for Expression {}
pub trait ExpressionRefMutMethodsDescendants<'a>
where
    Self: DescendantOf<ExpressionRefMut<'a>>,
    Self::Via: ExpressionRefMutMethods,
    Self: Sized,
{}
pub trait ExpressionRefMutMethods: Sized {}
impl<'a, T: ExpressionRefMutMethodsDescendants<'a>> ExpressionRefMutMethods for T
where
    T::Via: ExpressionRefMutMethods,
{}
impl<'a> DescendantOf<StepRefMut<'a>> for ExpressionRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> StepRefMutMethodsDescendants<'a> for ExpressionRefMut<'a> {}
impl<'a> DescendantOf<FeatureRefMut<'a>> for ExpressionRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> FeatureRefMutMethodsDescendants<'a> for ExpressionRefMut<'a> {}
impl<'a> DescendantOf<TypeRefMut<'a>> for ExpressionRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> TypeRefMutMethodsDescendants<'a> for ExpressionRefMut<'a> {}
impl<'a> DescendantOf<NamespaceRefMut<'a>> for ExpressionRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> NamespaceRefMutMethodsDescendants<'a> for ExpressionRefMut<'a> {}
impl<'a> DescendantOf<ElementRefMut<'a>> for ExpressionRefMut<'a> {
    type Via = StepRefMut<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref_mut()
    }
}
impl<'a> ElementRefMutMethodsDescendants<'a> for ExpressionRefMut<'a> {}
pub trait ExpressionRefMethodsDescendants<'a>
where
    Self: DescendantOf<ExpressionRef<'a>>,
    Self::Via: ExpressionRefMethods,
    Self: Sized,
{
    fn model_level_evaluable_impl(
        self,
        visited: Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>,
    ) -> bool {
        self.into_via().model_level_evaluable(visited)
    }
    fn evaluate_impl(
        self,
        target: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        self.into_via().evaluate(target)
    }
    fn check_condition_impl(
        self,
        target: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> bool {
        self.into_via().check_condition(target)
    }
}
pub trait ExpressionRefMethods: Sized {
    fn model_level_evaluable(
        self,
        visited: Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>,
    ) -> bool;
    fn evaluate(
        self,
        target: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>>;
    fn check_condition(
        self,
        target: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> bool;
}
impl<'a, T: ExpressionRefMethodsDescendants<'a>> ExpressionRefMethods for T
where
    T::Via: ExpressionRefMethods,
{
    fn model_level_evaluable(
        self,
        visited: Vec<std::rc::Rc<std::cell::RefCell<super::feature::Feature>>>,
    ) -> bool {
        ExpressionRefMethodsDescendants::model_level_evaluable_impl(self, visited)
    }
    fn evaluate(
        self,
        target: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<super::element::Element>>> {
        ExpressionRefMethodsDescendants::evaluate_impl(self, target)
    }
    fn check_condition(
        self,
        target: std::rc::Rc<std::cell::RefCell<super::element::Element>>,
    ) -> bool {
        ExpressionRefMethodsDescendants::check_condition_impl(self, target)
    }
}
impl<'a> DescendantOf<StepRef<'a>> for ExpressionRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> StepRefMethodsDescendants<'a> for ExpressionRef<'a> {}
impl<'a> DescendantOf<FeatureRef<'a>> for ExpressionRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> FeatureRefMethodsDescendants<'a> for ExpressionRef<'a> {}
impl<'a> DescendantOf<TypeRef<'a>> for ExpressionRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> TypeRefMethodsDescendants<'a> for ExpressionRef<'a> {}
impl<'a> DescendantOf<NamespaceRef<'a>> for ExpressionRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> NamespaceRefMethodsDescendants<'a> for ExpressionRef<'a> {}
impl<'a> DescendantOf<ElementRef<'a>> for ExpressionRef<'a> {
    type Via = StepRef<'a>;
    fn into_via(self) -> Self::Via {
        self.as_step_ref()
    }
}
impl<'a> ElementRefMethodsDescendants<'a> for ExpressionRef<'a> {}

