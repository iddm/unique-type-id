//! A unique id generator for rust types.
//!
//! The crate provides a trait and a procedural macro. By deriving one, you implement the
//! trait with `fn id() -> TypeId` static method. The type id is unique in the whole project.
//!
//! For examples, see the `tests` directory in the source tree.
//!
//! # Usage
//!
//! The example usage:
//!
//! ```rust
//!#[macro_use]
//!extern crate unique_type_id_derive;
//!extern crate unique_type_id;
//!
//!fn check_id() {
//!    use unique_type_id::UniqueTypeId;
//!
//!    // This code uses "types.toml" (the default) file for ids.
//!    #[derive(UniqueTypeId)]
//!    struct Test1;
//!    #[derive(UniqueTypeId)]
//!    struct Test2;
//!
//!    assert_eq!(Test1::id().0, 1u64);
//!    assert_eq!(Test2::id().0, 2u64);
//!}
extern crate quote;
extern crate syn;

/// A strong type for type id.
pub struct TypeId(pub u64);

/// A trait for providing a type id number.
pub trait UniqueTypeId {
    /// Returns the type id number.
    fn id() -> TypeId;
}
