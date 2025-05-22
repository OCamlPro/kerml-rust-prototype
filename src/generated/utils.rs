pub trait DescendantOf<T> {
    type Via;
    fn into_via(self) -> Self::Via;
}

