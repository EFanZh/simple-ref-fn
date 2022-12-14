use crate::ref_fn::RefFn;
use crate::ref_sync_fn::RefSyncFn;

/// A trait for defining a static function will be type erased.
pub trait StaticRefFunction<'a, D, T> {
    /// Return type of the defined function.
    type Output;

    /// Function definition.
    fn call(data: &'a D, arg: T) -> Self::Output;

    /// A helper function to create a [`RefFn`] object with the defined function.
    fn bind(data: &'a D) -> RefFn<'a, T, Self::Output> {
        RefFn::new::<Self, D>(data)
    }

    /// A helper function to create a [`RefSyncFn`] object with the defined function.
    fn bind_sync(data: &'a D) -> RefSyncFn<'a, T, Self::Output>
    where
        D: Sync,
    {
        RefSyncFn::new::<Self, D>(data)
    }
}

impl<'a, T, F, R> StaticRefFunction<'a, F, T> for F
where
    F: Fn(T) -> R,
{
    type Output = R;

    fn call(data: &'a Self, arg: T) -> Self::Output {
        (*data)(arg)
    }
}
