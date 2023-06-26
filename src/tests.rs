#[cfg(test)]
mod tests {
	use std::{io::Cursor, ops::Deref};
	use crate::CowBytes;
	#[test]
	fn basic_1() {
		let v = vec![1u8, 2u8, 3u8, 4u8];
		let mut buf = Cursor::new(Vec::new());
		let bytes = CowBytes::from(v.as_slice());
		assert!(bytes.is_borrowed());
		bincode::serialize_into(&mut buf, &bytes).unwrap();
		buf.set_position(0);
		let bytes: CowBytes = bincode::deserialize_from(&mut buf).unwrap();
		assert!(bytes.is_owned());
		assert_eq!(bytes.deref(), v);
	}
}
