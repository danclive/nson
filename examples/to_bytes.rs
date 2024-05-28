fn main() {
    use nson::m;
    use nson::Value;

    let a = Value::I32(123);
    println!("{:?}", a.to_bytes());

    let m = m! {
        "a": 123i32,
        "b": {
            "c": 456
        }
    };
    println!("{:?}", m.to_bytes());
    let m: Value = m.into();
    println!("{:?}", m.to_bytes());

    let a = 123;
    let bytes = nson::encode::to_bytes(&a).unwrap();
    println!("{:?}", bytes);

    let b: i32 = nson::decode::from_bytes(&bytes).unwrap();
    println!("{:?}", b);
    assert_eq!(a, b);
}
