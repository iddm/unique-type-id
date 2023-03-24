extern crate unique_type_id;
#[macro_use]
extern crate unique_type_id_derive;

#[test]
fn check_simple() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    struct Test1;
    #[derive(UniqueTypeId)]
    struct Test2;

    assert_eq!(Test1::id().0, 1u64);
    assert_eq!(Test2::id().0, 2u64);
}

#[test]
fn check_simple_custom_type() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdType = "i16"]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdType = "i16"]
    struct Test2;

    assert_eq!(Test1::id().0, 1i16);
    assert_eq!(Test2::id().0, 2i16);
}

#[test]
fn check_simple_second() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    struct Test1;
    #[derive(UniqueTypeId)]
    struct Test2;
    #[derive(UniqueTypeId)]
    struct Test3;
    #[derive(UniqueTypeId)]
    struct Test4;
    #[derive(UniqueTypeId)]
    struct Test5;

    assert_eq!(Test1::id().0, 1u64);
    assert_eq!(Test2::id().0, 2u64);
    assert_eq!(Test3::id().0, 5u64);
    assert_eq!(Test4::id().0, 0u64);
    assert_eq!(Test5::id().0, 3u64);

    assert_eq!(Test1::unique_type_id(), Test1::id());
    assert_eq!(Test2::unique_type_id(), Test2::id());
    assert_eq!(Test3::unique_type_id(), Test3::id());
    assert_eq!(Test4::unique_type_id(), Test4::id());
    assert_eq!(Test5::unique_type_id(), Test5::id());
}

#[test]
fn check_different_file() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test2;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test3;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test4;

    assert_eq!(Test1::id().0, 115u64);
    assert_eq!(Test2::id().0, 232u64);
    assert_eq!(Test3::id().0, 0u64);
    assert_eq!(Test4::id().0, 1u64);
}

#[test]
fn check_simple_empty_file() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types3.toml"]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types3.toml"]
    struct Test2;

    // One of Test1 or Test2 should get "0", and the other should get "1"
    let unique_ids = [Test1::id().0, Test2::id().0];
    assert!(unique_ids.contains(&0u64));
    assert!(unique_ids.contains(&1u64));
    assert_ne!(Test1::id().0, Test2::id().0);
}

#[test]
fn check_empty_file_custom_start() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types4.toml"]
    #[UniqueTypeIdStart = 23]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types4.toml"]
    #[UniqueTypeIdStart = 23]
    struct Test2;

    // One of Test1 or Test2 should get "23", and the other should get "24"
    let unique_ids = [Test1::id().0, Test2::id().0];
    assert!(unique_ids.contains(&23u64));
    assert!(unique_ids.contains(&24u64));
    assert_ne!(Test1::id().0, Test2::id().0);
}
