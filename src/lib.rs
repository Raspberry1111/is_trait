//! This crate adds a `is_trait!` macro for getting a runtime value for if a type implements a trait
//! This can be useful for test, where you can have a test fail instead of getting a compile error
//! Example:
//! ```should_panic
//! use is_trait::is_trait;
//!  
//! struct Thing;
//! struct OtherThing;
//! trait SomeTrait {}
//! impl SomeTrait for Thing {}

//! assert!(is_trait!(Thing, SomeTrait));
//! assert!(is_trait!(OtherThing, SomeTrait)); // FAILS
//! ```

/// [See crate level docs for usage](index.html)
///
/// Under the hood, this macro creates trait `A` and struct `B<T>`
///
/// `B<T>` implements [`std::ops::Deref`] to `()`
///
/// `A` is implemented for `()` to return `false`
///
/// `A` is implemented for `B<T>` where `T: SomeTrait` to return true
///
/// We then call `A` on `B::<SomeType>`.
/// Because of rust dereferencing rules, if `SomeType` is `SomeTrait`, then we call `A` on `B<T>` which is true.
/// However, if `SomeType` is not `SomeTrait`, we dereference `B<T>` into `()` and call `A` on that, which is false
#[macro_export]
macro_rules! is_trait {
    ($type:ty, $trait:path) => {{
        trait A {
            fn is(&self) -> bool;
        }

        struct B<T: ?Sized>(core::marker::PhantomData<T>);

        impl<T: ?Sized> core::ops::Deref for B<T> {
            type Target = ();
            fn deref(&self) -> &Self::Target {
                &()
            }
        }

        impl<T: ?Sized> A for B<T>
        where
            T: $trait,
        {
            fn is(&self) -> bool {
                true
            }
        }

        impl A for () {
            fn is(&self) -> bool {
                false
            }
        }

        B::<$type>(core::marker::PhantomData).is()
    }};
}

/// Like [`is_trait`] but uses const trait impls
#[macro_export]
macro_rules! const_is_trait {
    ($type:ty, $trait:path) => {{
        trait A {
            fn is(&self) -> bool;
        }

        struct B<T: ?Sized>(core::marker::PhantomData<T>);

        impl<T: ?Sized> const core::ops::Deref for B<T> {
            type Target = ();
            fn deref(&self) -> &Self::Target {
                &()
            }
        }

        impl<T: ?Sized> const A for B<T>
        where
            T: $trait,
        {
            fn is(&self) -> bool {
                true
            }
        }

        impl const A for () {
            fn is(&self) -> bool {
                false
            }
        }

        B::<$type>(core::marker::PhantomData).is()
    }};
}
