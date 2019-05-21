use std::prelude::v1::*;
type BytesShort = elastic_array::ElasticArray2<u8>;

//#[test]
pub fn it_works() {
	let mut bytes = BytesShort::new();
	assert_eq!(bytes.len(), 0);
	bytes.push(1);
	assert_eq!(bytes.len(), 1);
	assert_eq!(bytes[0], 1);
	bytes.push(2);
	assert_eq!(bytes[1], 2);
	assert_eq!(bytes.len(), 2);
	bytes.push(3);
	assert_eq!(bytes[2], 3);
	assert_eq!(bytes.len(), 3);
	assert_eq!(bytes.pop(), Some(3));
	assert_eq!(bytes.len(), 2);
	assert_eq!(bytes.pop(), Some(2));
	assert_eq!(bytes.pop(), Some(1));
	assert_eq!(bytes.pop(), None);
}

//#[test]
pub fn test_insert_slice() {
	let mut bytes = BytesShort::new();
	bytes.push(1);
	bytes.push(2);
	bytes.insert_slice(1, &[3, 4]);
	assert_eq!(bytes.len(), 4);
	let r: &[u8] = &bytes;
	assert_eq!(r, &[1, 3, 4, 2]);
}

//#[test]
pub fn append_slice() {
	let mut bytes = BytesShort::new();
	bytes.push(1);
	bytes.append_slice(&[3, 4]);
	let r: &[u8] = &bytes;
	assert_eq!(r.len(), 3);
	assert_eq!(r, &[1, 3 ,4]);
}

//#[test]
pub fn use_in_map() {
	//#[cfg(feature = "std")]
	use std::collections::BTreeMap;
	//#[cfg(not(feature = "std"))]
	//use alloc::collections::BTreeMap;
	use core::borrow::Borrow;
	let mut map: BTreeMap<BytesShort, i32> = Default::default();
	let mut bytes = BytesShort::new();
	bytes.append_slice(&[3, 4]);
	assert_eq!(bytes.borrow() as &[u8], &[3, 4][..]);
	map.insert(bytes, 1);
	assert_eq!(map.get(&[3, 4][..]), Some(&1i32));
}

