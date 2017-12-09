//! A unique id generator for rust types.
//!
//! The crate provides a trait and a procedural macro. By deriving one, you implement the
//! trait with `fn id() -> TypeId` static method which is unique in the whole project.
//!
//! For examples, see the `tests` directory in the source tree.
//!
//! # Usage
//!
//! ```rust
//!#[macro_use]
//!extern crate unique_type_id_derive;
//!extern crate unique_type_id;
//!
//!fn sequential_id() {
//!    use unique_type_id::SequentialTypeId;
//!
//!    #[derive(SequentialTypeId)]
//!    struct Test1;
//!    #[derive(SequentialTypeId)]
//!    struct Test2;
//!
//!    assert_eq!(Test1::id().0, 0u64);
//!    assert_eq!(Test2::id().0, 1u64);
//!}
extern crate syn;
extern crate quote;

/// A strong type for type id.
pub struct TypeId(pub u64);

/// A trait for providing a sequential type id number.
pub trait SequentialTypeId {
    /// Provides sequential type id number.
    fn id() -> TypeId;
}
