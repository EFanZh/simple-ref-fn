use crate::ref_fn::RefFn;
use crate::static_ref_function::StaticRefFunction;

/// A simple function wrapper that behaves like a [`&dyn Fn(T) -> R + Sync`](`Fn`) type, but does not require a virtual table.
pub struct RefSyncFn<'a, T, R> {
    inner: RefFn<'a, T, R>,
}

impl<'a, T, R> RefSyncFn<'a, T, R> {
    /// Create a [`RefSyncFn`] object by binding [`F::call`](`StaticRefFunction::call`) function with `data`.
    pub fn new<F, D>(data: &'a D) -> Self
    where
        F: StaticRefFunction<'a, D, T, Output = R> + ?Sized,
        D: Sync,
    {
        Self {
            inner: RefFn::new::<F, D>(data),
        }
    }

    /// Create a [`RefSyncFn`] object from a function reference.
    pub fn from_fn<F>(f: &'a F) -> Self
    where
        F: Fn(T) -> R + Sync,
    {
        Self::new::<F, F>(f)
    }

    /// Call the wrapped function.
    pub fn call(&self, arg: T) -> R {
        self.inner.call(arg)
    }
}

impl<T, R> Clone for RefSyncFn<'_, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, R> Copy for RefSyncFn<'_, T, R> {}

impl<'a, F, T, R> From<&'a F> for RefSyncFn<'a, T, R>
where
    F: Fn(T) -> R + Sync,
{
    fn from(value: &'a F) -> Self {
        Self::from_fn(value)
    }
}

unsafe impl<T, R> Send for RefSyncFn<'_, T, R> {}
unsafe impl<T, R> Sync for RefSyncFn<'_, T, R> {}

#[cfg(test)]
mod tests {
    use super::RefSyncFn;
    use crate::static_ref_function::StaticRefFunction;

    static_assertions::assert_impl_all!(RefSyncFn<'static, (), ()>: Clone, Copy, From<&'static fn(())>, Send, Sync);

    #[test]
    fn test_ref_sync_fn_new() {
        struct F;

        impl StaticRefFunction<'_, u32, u32> for F {
            type Output = u32;

            fn call(data: &u32, arg: u32) -> Self::Output {
                data + arg
            }
        }

        let data = 2;
        let f: RefSyncFn<u32, u32> = F::bind_sync(&data);

        assert_eq!(f.call(3), 5);
        assert_eq!(f.call(5), 7);
        assert_eq!(f.call(7), 9);
    }

    #[test]
    fn test_ref_sync_fn_from() {
        let data = 2_u32;
        let closure = |arg: u32| data + arg;
        let f: RefSyncFn<u32, u32> = RefSyncFn::from(&closure);

        assert_eq!(f.call(3), 5);
        assert_eq!(f.call(5), 7);
        assert_eq!(f.call(7), 9);
    }
}
