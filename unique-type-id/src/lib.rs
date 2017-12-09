extern crate syn;
extern crate quote;

/// A strong type for type id.
pub struct TypeId(pub u64);

pub trait SequentialTypeId {
    /// Provides sequential type id number.
    fn id() -> TypeId;
}
