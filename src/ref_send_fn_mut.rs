use crate::ref_fn_mut::RefFnMut;
use crate::static_ref_mut_function::StaticRefMutFunction;

/// A simple function wrapper that behaves like a [`&mut (dyn FnMut(T) -> R + Send)`](`FnMut`) type, but does not
/// require a virtual table.
pub struct RefSendFnMut<'a, T, R> {
    inner: RefFnMut<'a, T, R>,
}

impl<'a, T, R> RefSendFnMut<'a, T, R> {
    /// Create a [`RefSendFnMut`] object by binding [`F::call_mut`](`StaticRefMutFunction::call_mut`) function with
    /// `data`.
    pub fn new<F, D>(data: &'a mut D) -> Self
    where
        F: StaticRefMutFunction<'a, D, T, Output = R> + ?Sized,
        D: Send,
    {
        Self {
            inner: RefFnMut::new::<F, D>(data),
        }
    }

    /// Create a [`RefSendFnMut`] object from a function reference.
    pub fn from_fn_mut<F>(f: &'a mut F) -> Self
    where
        F: FnMut(T) -> R + Send,
    {
        Self::new::<F, F>(f)
    }

    /// Call the wrapped function.
    pub fn call_mut(&mut self, arg: T) -> R {
        self.inner.call_mut(arg)
    }
}

impl<'a, F, T, R> From<&'a mut F> for RefSendFnMut<'a, T, R>
where
    F: FnMut(T) -> R + Send,
{
    fn from(value: &'a mut F) -> Self {
        Self::from_fn_mut(value)
    }
}

unsafe impl<T, R> Send for RefSendFnMut<'_, T, R> {}

#[cfg(test)]
mod tests {
    use super::RefSendFnMut;
    use crate::static_ref_mut_function::StaticRefMutFunction;

    static_assertions::assert_impl_all!(RefSendFnMut<'static, (), ()>: From<&'static mut fn(())>, Send, Sync);

    #[test]
    fn test_ref_send_fn_mut_new() {
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
        let mut f: RefSendFnMut<u32, u32> = F::bind_send(&mut data);

        assert_eq!(f.call_mut(3), 2);
        assert_eq!(f.call_mut(5), 5);
        assert_eq!(f.call_mut(7), 10);

        assert_eq!(data, 17);
    }

    #[test]
    fn test_ref_send_fn_mut_from() {
        let mut data = 2_u32;

        let mut closure = |arg: u32| {
            let old_value = data;

            data += arg;

            old_value
        };

        let mut f: RefSendFnMut<u32, u32> = RefSendFnMut::from(&mut closure);

        assert_eq!(f.call_mut(3), 2);
        assert_eq!(f.call_mut(5), 5);
        assert_eq!(f.call_mut(7), 10);

        assert_eq!(data, 17);
    }
}
