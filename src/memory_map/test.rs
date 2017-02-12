#[test]
fn test_mirrored_region_one_writes() {
    let mut map = super::MemoryMap::default();

    map.write_to_mirrored_region_one(0x0000, 1);

    // test we wrote to the right places
    assert_eq!(&map.read(0x0000), &1);
    assert_eq!(&map.read(0x0800), &1);
    assert_eq!(&map.read(0x1000), &1);
    assert_eq!(&map.read(0x1800), &1);

    // test we stayed within our limits
    assert_eq!(&map.read(0x2000), &0);
}

#[test]
fn test_mirrored_region_two_writes() {
    let mut map = super::MemoryMap::default();

    map.write_to_mirrored_region_two(0x2009, 1);

    // test we wrote to the right places
    assert_eq!(&map.read(0x2001), &1);
    assert_eq!(&map.read(0x2009), &1);
    assert_eq!(&map.read(0x3ff9), &1);

    // test we stayed within our limits
    assert_eq!(&map.read(0x4000), &0);
}