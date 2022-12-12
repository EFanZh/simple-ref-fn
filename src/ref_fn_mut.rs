use crate::static_ref_mut_function::StaticRefMutFunction;
use core::marker::PhantomData;
use core::ptr::NonNull;

unsafe fn call_mut_fn<'a, F, D, T>(data: NonNull<()>, arg: T) -> F::Output
where
    F: StaticRefMutFunction<'a, D, T> + ?Sized,
    D: 'a,
{
    F::call_mut(unsafe { data.cast().as_mut() }, arg)
}

/// A simple function wrapper that behaves like a [`&mut dyn FnMut(T) -> R`](`FnMut`) type, but does not require a
/// virtual table.
pub struct RefFnMut<'a, T, R> {
    data: NonNull<()>,
    call_mut_fn: unsafe fn(NonNull<()>, T) -> R,
    _phantom: PhantomData<&'a mut ()>,
}

impl<'a, T, R> RefFnMut<'a, T, R> {
    /// Create a [`RefFnMut`] object by binding [`F::call_mut`](`StaticRefMutFunction::call_mut`) function with `data`.
    pub fn new<F, D>(data: &'a mut D) -> Self
    where
        F: StaticRefMutFunction<'a, D, T, Output = R> + ?Sized,
    {
        Self {
            data: NonNull::from(data).cast(),
            call_mut_fn: call_mut_fn::<'a, F, D, T>,
            _phantom: PhantomData,
        }
    }

    /// Create a [`RefFnMut`] object from a function reference.
    pub fn from_fn_mut<F>(f: &'a mut F) -> Self
    where
        F: FnMut(T) -> R,
    {
        Self::new::<F, F>(f)
    }

    /// Call the wrapped function.
    pub fn call_mut(&mut self, arg: T) -> R {
        unsafe { (self.call_mut_fn)(self.data, arg) }
    }
}

impl<'a, F, T, R> From<&'a mut F> for RefFnMut<'a, T, R>
where
    F: FnMut(T) -> R,
{
    fn from(value: &'a mut F) -> Self {
        Self::from_fn_mut(value)
    }
}

/// Unconditionally `Sync` because this type only provides mutable access, so sharing non-mutable reference is safe.
unsafe impl<T, R> Sync for RefFnMut<'_, T, R> {}

#[cfg(test)]
mod tests {
    use super::RefFnMut;
    use crate::static_ref_mut_function::StaticRefMutFunction;

    static_assertions::assert_impl_all!(RefFnMut<'static, (), ()>: From<&'static mut fn(())>, Sync);
    static_assertions::assert_not_impl_any!(RefFnMut<'static, (), ()>: Send);

    #[test]
    fn test_ref_fn_mut_new() {
        struct F;

        impl StaticRefMutFunction<'_, u32, u32> for F {
            type Output = u32;

            fn call_mut(data: &mut u32, arg: u32) -> Self::Output {
                let old_value = *data;

                *data += arg;

                old_value
            }
        }

        let mut data = 2;
        let mut f: RefFnMut<u32, u32> = F::bind(&mut data);

        assert_eq!(f.call_mut(3), 2);
        assert_eq!(f.call_mut(5), 5);
        assert_eq!(f.call_mut(7), 10);

        assert_eq!(data, 17);
    }

    #[test]
    fn test_ref_fn_mut_from() {
        let mut data = 2_u32;

        let mut closure = |arg: u32| {
            let old_value = data;

            data += arg;

            old_value
        };

        let mut f: RefFnMut<u32, u32> = RefFnMut::from(&mut closure);

        assert_eq!(f.call_mut(3), 2);
        assert_eq!(f.call_mut(5), 5);
        assert_eq!(f.call_mut(7), 10);

        assert_eq!(data, 17);
    }
}
