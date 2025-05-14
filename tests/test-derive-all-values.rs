use derive_enum_all_values::AllValues;

#[test]
fn test_derive_all_values() {
    #[derive(AllValues)]
    enum MyEnum {
        Value1 = 0,
        Value2 = 2,
        Value3 = 4,
        Value4 = 8,
    }

    assert_eq!(4, MyEnum::all_values().len());

    // Asserts that `all_values` is a const.
    const _: &[MyEnum] = MyEnum::all_values();
    const _: [MyEnum; MyEnum::all_values().len()] = [MyEnum::Value1, MyEnum::Value2, MyEnum::Value3, MyEnum::Value4];
}

