#[macro_export]
macro_rules! component_filter {
  ($($x:ty),*) => (
    ::recs::ComponentFilter::from_slice(
      &vec![$(::std::any::TypeId::of::<$x>()),*]
    )
  );
  ($($x:ty,)*) => (component_filter![$($x),*])
}

pub fn clamp(input: f64, min: f64, max: f64) -> f64 {
  input.max(min).min(max)
}