pub use value::Value;
pub use message::Message;

mod macros;
pub mod value;
pub mod message;
pub mod encode;
pub mod decode;
pub mod serde_impl;
mod spec;
mod util;
pub mod message_id;

#[cfg(test)]
mod test {
	use serde_derive::{Serialize, Deserialize};
	use serde_bytes;
	use crate::encode::to_nson;
	use crate::decode::from_nson;
	use crate::msg;

	#[derive(Serialize, Deserialize, Debug, PartialEq)]
	pub struct Foo {
		a: i32,
		b: i64,
		c: f64,
		d: String,
		#[serde(with = "serde_bytes")]
		e: Vec<u8>
	}

	#[test]
	fn serialize_and_deserialize() {
		let foo = Foo {
			a: 1,
			b: 2,
			c: 3.0,
			d: "4".to_string(),
			e: vec![1, 2, 3, 4]
		};

		let bson = to_nson(&foo).unwrap();
		let foo2: Foo = from_nson(bson).unwrap();

		assert_eq!(foo, foo2);
	}

	#[test]
	fn binary() {
		let byte = vec![1u8, 2, 3, 4];
		let msg = msg!{"aa": "bb", "byte": byte.clone()};
		let byte2 = msg.get_binary("byte").unwrap();

		assert_eq!(&byte, byte2);

		let mut msg2 = msg!{"aa": "bb"};
		msg2.insert("byte", byte);

		assert_eq!(msg, msg2);
	}
}
