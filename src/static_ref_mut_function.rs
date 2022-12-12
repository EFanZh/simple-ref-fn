use crate::ref_fn_mut::RefFnMut;
use crate::ref_send_fn_mut::RefSendFnMut;

/// A trait for defining a static function that will be type erased.
pub trait StaticRefMutFunction<'a, D, T> {
    /// Return type of the defined function.
    type Output;

    /// Function definition.
    fn call_mut(data: &'a mut D, arg: T) -> Self::Output;

    /// A helper function to create a [`RefFnMut`] object with the defined function.
    fn bind(data: &'a mut D) -> RefFnMut<'a, T, Self::Output> {
        RefFnMut::new::<Self, D>(data)
    }

    /// A helper function to create a [`RefSendFnMut`] object with the defined function.
    fn bind_send(data: &'a mut D) -> RefSendFnMut<'a, T, Self::Output>
    where
        D: Send,
    {
        RefSendFnMut::new::<Self, D>(data)
    }
}

impl<'a, T, F, R> StaticRefMutFunction<'a, F, T> for F
where
    F: FnMut(T) -> R,
{
    type Output = R;

    fn call_mut(data: &'a mut Self, arg: T) -> Self::Output {
        (*data)(arg)
    }
}
