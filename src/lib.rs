#![no_std]
#![warn(
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::cargo_common_metadata,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::empty_line_after_outer_attr,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::panic_in_result_fn,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trivial_regex,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::wildcard_dependencies
)]
#![allow(clippy::non_ascii_literal)]

//! This crate provides type-erased function wrappers that behave like `&dyn Fn(T) -> R` and `&mut dyn FnMut(T) -> R`,
//! but do not require compiler to generate virtual tables.
//!
//! The following wrapper types are provided:
//!
//! | Type                       | Behaves like                           | [`Send`] | [`Sync`] |
//! | -------------------------- | -------------------------------------- | -------- | -------- |
//! | [`RefFn<'a, T, R>`]        | [`&'a dyn Fn(T) -> R`]                 | No       | No       |
//! | [`RefSyncFn<'a, T, R>`]    | [`&'a (dyn Fn(T) -> R + Sync)`]        | Yes      | Yes      |
//! | [`RefFnMut<'a, T, R>`]     | [`&'a mut dyn FnMut(T) -> R`]          | No       | Yes      |
//! | [`RefSendFnMut<'a, T, R>`] | [`&'a mut (dyn FnMut(T) -> R + Send)`] | Yes      | Yes      |
//!
//! [`RefFn<'a, T, R>`]: `RefFn`
//! [`RefSyncFn<'a, T, R>`]: `RefSyncFn`
//! [`RefFnMut<'a, T, R>`]: `RefFnMut`
//! [`RefSendFnMut<'a, T, R>`]: `RefSendFnMut`
//! [`&'a dyn Fn(T) -> R`]: `Fn`
//! [`&'a (dyn Fn(T) -> R + Sync)`]: `Fn`
//! [`&'a mut dyn FnMut(T) -> R`]: `FnMut`
//! [`&'a mut (dyn FnMut(T) -> R + Send)`]: `FnMut`
//!
//! ## Examples
//!
//! ### Bind a function with a reference
//!
//! ```rust
//! use simple_ref_fn::{RefFn, StaticRefFunction};
//!
//! // Define a static function.
//!
//! struct F;
//!
//! impl StaticRefFunction<'_, u32, u32> for F {
//!     type Output = u32;
//!
//!     fn call(data: &u32, arg: u32) -> Self::Output {
//!         data + arg
//!     }
//! }
//!
//! // Bind the function with a reference.
//!
//! let data = 2;
//! let f: RefFn<u32, u32> = F::bind(&data); // Type-erasure.
//!
//! assert_eq!(f.call(3), 5);
//! assert_eq!(f.call(5), 7);
//! assert_eq!(f.call(7), 9);
//! ```
//!
//! ### Erase the type of a closure
//!
//! ```rust
//! use simple_ref_fn::RefFn;
//!
//! let data = 2_u32;
//! let closure = |arg: u32| data + arg;
//! let f: RefFn<u32, u32> = RefFn::from(&closure);  // Type-erasure.
//!
//! assert_eq!(f.call(3), 5);
//! assert_eq!(f.call(5), 7);
//! assert_eq!(f.call(7), 9);
//! ```

pub use self::ref_fn::RefFn;
pub use self::ref_fn_mut::RefFnMut;
pub use self::ref_send_fn_mut::RefSendFnMut;
pub use self::ref_sync_fn::RefSyncFn;
pub use self::static_ref_function::StaticRefFunction;
pub use self::static_ref_mut_function::StaticRefMutFunction;

mod ref_fn;
mod ref_fn_mut;
mod ref_send_fn_mut;
mod ref_sync_fn;
mod static_ref_function;
mod static_ref_mut_function;
