pub use crate::generated::multiplicity_range::{
    MultiplicityRange, MultiplicityRangeMethods, MultiplicityRangeRef, MultiplicityRangeRefMethods,
    MultiplicityRangeRefMut, MultiplicityRangeRefMutMethods,
};

impl MultiplicityRangeMethods for MultiplicityRange {}

impl MultiplicityRangeRefMutMethods for MultiplicityRangeRefMut<'_> {}

impl MultiplicityRangeRefMethods for MultiplicityRangeRef<'_> {
    fn has_bounds(self, lower: i64, upper: u64) -> bool {
        todo!()
    }

    fn value_of(
        self,
        bound: Option<std::rc::Rc<std::cell::RefCell<crate::generated::expression::Expression>>>,
    ) -> Option<u64> {
        todo!()
    }
}
