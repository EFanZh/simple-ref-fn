use crate::static_ref_function::StaticRefFunction;
use core::marker::PhantomData;
use core::ptr::NonNull;

unsafe fn call_fn<'a, F, D, T>(data: NonNull<()>, arg: T) -> F::Output
where
    F: StaticRefFunction<'a, D, T> + ?Sized,
    D: 'a,
{
    F::call(unsafe { data.cast().as_ref() }, arg)
}

/// A simple function wrapper that behaves like a [`&dyn Fn(T) -> R`](`Fn`) type, but does not require a virtual table.
pub struct RefFn<'a, T, R> {
    data: NonNull<()>,
    call_fn: unsafe fn(NonNull<()>, T) -> R,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T, R> RefFn<'a, T, R> {
    /// Create a [`RefFn`] object by binding [`F::call`](`StaticRefFunction::call`) function with `data`.
    pub fn new<F, D>(data: &'a D) -> Self
    where
        F: StaticRefFunction<'a, D, T, Output = R> + ?Sized,
    {
        Self {
            data: NonNull::from(data).cast(),
            call_fn: call_fn::<'a, F, D, T>,
            _phantom: PhantomData,
        }
    }

    /// Create a [`RefFn`] object from a function reference.
    pub fn from_fn<F>(f: &'a F) -> Self
    where
        F: Fn(T) -> R,
    {
        Self::new::<F, F>(f)
    }

    /// Call the wrapped function.
    pub fn call(&self, arg: T) -> R {
        unsafe { (self.call_fn)(self.data, arg) }
    }
}

impl<T, R> Clone for RefFn<'_, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, R> Copy for RefFn<'_, T, R> {}

impl<'a, F, T, R> From<&'a F> for RefFn<'a, T, R>
where
    F: Fn(T) -> R,
{
    fn from(value: &'a F) -> Self {
        Self::from_fn(value)
    }
}

#[cfg(test)]
mod tests {
    use super::RefFn;
    use crate::static_ref_function::StaticRefFunction;

    static_assertions::assert_impl_all!(RefFn<'static, (), ()>: Clone, Copy, From<&'static fn(())>);
    static_assertions::assert_not_impl_any!(RefFn<'static, (), ()>: Send, Sync);

    #[test]
    fn test_ref_fn_new() {
        struct F;

        impl StaticRefFunction<'_, u32, u32> for F {
            type Output = u32;

            fn call(data: &u32, arg: u32) -> Self::Output {
                data + arg
            }
        }

        let data = 2;
        let f: RefFn<u32, u32> = F::bind(&data);

        assert_eq!(f.call(3), 5);
        assert_eq!(f.call(5), 7);
        assert_eq!(f.call(7), 9);
    }

    #[test]
    fn test_ref_fn_from() {
        let data = 2_u32;
        let closure = |arg: u32| data + arg;
        let f: RefFn<u32, u32> = RefFn::from(&closure);

        assert_eq!(f.call(3), 5);
        assert_eq!(f.call(5), 7);
        assert_eq!(f.call(7), 9);
    }
}
