#[macro_export]
macro_rules! component_filter {
  ($($x:ty),*) => (
    ::recs::ComponentFilter::from_slice(
      &vec![$(::std::any::TypeId::of::<$x>()),*]
    )
  );
  ($($x:ty,)*) => (component_filter![$($x),*])
}