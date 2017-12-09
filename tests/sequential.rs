#[macro_use]
extern crate unique_type_id_derive;
extern crate unique_type_id;


#[test]
fn sequential_simple() {
    use unique_type_id::{ SequentialTypeId };
    #[derive(SequentialTypeId)]
    struct Test1;
    #[derive(SequentialTypeId)]
    struct Test2;


    assert_eq!(Test1::id().0, 0u64);
    assert_eq!(Test2::id().0, 1u64);
}
