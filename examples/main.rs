use nson::m;

fn main() {
    let mut value = m! {
        "code": 200,
        "success": true,
        "payload": {
            "some": [
                "pay",
                "loads",
            ]
        }
    };

    println!("{:?}", value);
    // print: Map{"code": I32(200), "success": Bool(true), "payload": Map{"some": Array([String("pay"), String("loads")])}}

    println!("{:?}", value.get("code"));
    // print: Some(I32(200))

    // insert new key, value
    value.insert("hello", "world");

    println!("{:?}", value.get("hello"));
    // print: Some(String("world"))
}
