use crate::rust::marker::PhantomData;
use crate::rust::string::String;
use crate::type_id::*;
use crate::*;

/// Represents an error ocurred during decoding.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeId)]
pub enum DecodeError {
    ExtraTrailingBytes(usize),

    BufferUnderflow { required: usize, remaining: usize },

    UnexpectedTypeId { expected: u8, actual: u8 },

    UnexpectedCustomTypeId { actual: u8 },

    UnexpectedSize { expected: usize, actual: usize },

    UnknownTypeId(u8),

    UnknownDiscriminator(String),

    InvalidUnit(u8),

    InvalidBool(u8),

    InvalidUtf8,

    SizeTooLarge,

    InvalidCustomValue, // TODO: generify custom error codes
}

pub trait Decoder<X: CustomTypeId>: Sized {
    fn decode<T: Decode<X, Self>>(&mut self) -> Result<T, DecodeError> {
        let type_id = self.read_type_id()?;
        self.decode_body_with_type_id(type_id)
    }

    fn decode_body_with_type_id<T: Decode<X, Self>>(
        &mut self,
        type_id: SborTypeId<X>,
    ) -> Result<T, DecodeError> {
        self.track_decode_depth_increase()?;
        let decoded = T::decode_body_with_type_id(self, type_id)?;
        self.track_decode_depth_decrease();
        Ok(decoded)
    }

    fn read_type_id(&mut self) -> Result<SborTypeId<X>, DecodeError> {
        let id = self.read_byte()?;
        SborTypeId::from_u8(id).ok_or(DecodeError::UnknownTypeId(id))
    }

    fn read_discriminator(&mut self) -> Result<String, DecodeError> {
        let n = self.read_size()?;
        let slice = self.read_slice(n)?;
        String::from_utf8(slice.to_vec()).map_err(|_| DecodeError::InvalidUtf8)
    }

    fn read_size(&mut self) -> Result<usize, DecodeError> {
        // LEB128 and 4 bytes max
        let mut size = 0usize;
        let mut shift = 0;
        loop {
            let byte = self.read_byte()?;
            size |= ((byte & 0x7F) as usize) << shift;
            if byte < 0x80 {
                break;
            }
            shift += 7;
            if shift >= 28 {
                return Err(DecodeError::SizeTooLarge);
            }
        }
        Ok(size)
    }

    fn check_preloaded_type_id(
        &self,
        type_id: SborTypeId<X>,
        expected: SborTypeId<X>,
    ) -> Result<SborTypeId<X>, DecodeError> {
        if type_id == expected {
            Ok(type_id)
        } else {
            Err(DecodeError::UnexpectedTypeId {
                actual: type_id.as_u8(),
                expected: expected.as_u8(),
            })
        }
    }

    fn read_and_check_type_id(
        &mut self,
        expected: SborTypeId<X>,
    ) -> Result<SborTypeId<X>, DecodeError> {
        let type_id = self.read_type_id()?;
        self.check_preloaded_type_id(type_id, expected)
    }

    fn read_and_check_size(&mut self, expected: usize) -> Result<(), DecodeError> {
        let len = self.read_size()?;
        if len != expected {
            return Err(DecodeError::UnexpectedSize {
                expected,
                actual: len,
            });
        }

        Ok(())
    }

    fn check_end(&self) -> Result<(), DecodeError>;

    fn track_decode_depth_increase(&mut self) -> Result<(), DecodeError>;

    fn track_decode_depth_decrease(&mut self);

    fn read_byte(&mut self) -> Result<u8, DecodeError>;

    fn read_slice(&mut self, n: usize) -> Result<&[u8], DecodeError>;
}

/// A `Decoder` abstracts the logic for decoding basic types.
pub struct VecDecoder<'de, X: CustomTypeId> {
    input: &'de [u8],
    offset: usize,
    phantom: PhantomData<X>,
}

impl<'de, X: CustomTypeId> VecDecoder<'de, X> {
    pub fn new(input: &'de [u8]) -> Self {
        Self {
            input,
            offset: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn require_remaining(&self, n: usize) -> Result<(), DecodeError> {
        if self.remaining_bytes() < n {
            Err(DecodeError::BufferUnderflow {
                required: n,
                remaining: self.remaining_bytes(),
            })
        } else {
            Ok(())
        }
    }

    #[inline]
    fn remaining_bytes(&self) -> usize {
        self.input.len() - self.offset
    }
}

impl<'de, X: CustomTypeId> Decoder<X> for VecDecoder<'de, X> {
    #[inline]
    fn read_byte(&mut self) -> Result<u8, DecodeError> {
        self.require_remaining(1)?;
        let result = self.input[self.offset];
        self.offset += 1;
        Ok(result)
    }

    #[inline]
    fn read_slice(&mut self, n: usize) -> Result<&'de [u8], DecodeError> {
        self.require_remaining(n)?;
        let slice = &self.input[self.offset..self.offset + n];
        self.offset += n;
        Ok(slice)
    }

    #[inline]
    fn check_end(&self) -> Result<(), DecodeError> {
        let n = self.remaining_bytes();
        if n != 0 {
            Err(DecodeError::ExtraTrailingBytes(n))
        } else {
            Ok(())
        }
    }

    #[inline]
    fn track_decode_depth_increase(&mut self) -> Result<(), DecodeError> {
        // TODO
        Ok(())
    }

    #[inline]
    fn track_decode_depth_decrease(&mut self) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::Encode;
    use crate::encode::Encoder;
    use crate::rust::borrow::ToOwned;
    use crate::rust::boxed::Box;
    use crate::rust::cell::RefCell;
    use crate::rust::collections::*;
    use crate::rust::rc::Rc;
    use crate::rust::string::String;
    use crate::rust::vec;
    use crate::rust::vec::Vec;

    fn encode_decode_size(size: usize) -> Result<(), DecodeError> {
        // Encode
        let mut bytes = Vec::with_capacity(512);
        let mut enc = Encoder::<NoCustomTypeId>::new(&mut bytes);
        enc.write_size(size);

        let mut dec = VecDecoder::<NoCustomTypeId>::new(&bytes);
        dec.read_and_check_size(size)?;
        dec.check_end()?;
        Ok(())
    }

    #[test]
    pub fn test_vlq() {
        encode_decode_size(0x00000000).unwrap();
        encode_decode_size(0x0000007F).unwrap();
        encode_decode_size(0x00000080).unwrap();
        encode_decode_size(0x00002000).unwrap();
        encode_decode_size(0x00003FFF).unwrap();
        encode_decode_size(0x00004000).unwrap();
        encode_decode_size(0x001FFFFF).unwrap();
        encode_decode_size(0x00200000).unwrap();
        encode_decode_size(0x08000000).unwrap();
        encode_decode_size(0x0FFFFFFF).unwrap();
    }

    #[test]
    pub fn test_vlq_too_large() {
        let mut dec = VecDecoder::<NoCustomTypeId>::new(&[0xff, 0xff, 0xff, 0xff, 0x00]);
        assert_eq!(dec.read_size(), Err(DecodeError::SizeTooLarge));
    }

    fn assert_decoding<X: CustomTypeId>(dec: &mut VecDecoder<X>) {
        dec.decode::<()>().unwrap();
        assert_eq!(true, dec.decode::<bool>().unwrap());
        assert_eq!(1, dec.decode::<i8>().unwrap());
        assert_eq!(1, dec.decode::<i16>().unwrap());
        assert_eq!(1, dec.decode::<i32>().unwrap());
        assert_eq!(1, dec.decode::<i64>().unwrap());
        assert_eq!(1, dec.decode::<i128>().unwrap());
        assert_eq!(1, dec.decode::<u8>().unwrap());
        assert_eq!(1, dec.decode::<u16>().unwrap());
        assert_eq!(1, dec.decode::<u32>().unwrap());
        assert_eq!(1, dec.decode::<u64>().unwrap());
        assert_eq!(1, dec.decode::<u128>().unwrap());
        assert_eq!("hello", dec.decode::<String>().unwrap());

        assert_eq!([1u32, 2u32, 3u32], dec.decode::<[u32; 3]>().unwrap());
        assert_eq!((1u32, 2u32), dec.decode::<(u32, u32)>().unwrap());

        assert_eq!(vec![1u32, 2u32, 3u32], dec.decode::<Vec<u32>>().unwrap());
        let mut set = BTreeSet::<u8>::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set, dec.decode::<BTreeSet<u8>>().unwrap());
        let mut map = BTreeMap::<u8, u8>::new();
        map.insert(1, 2);
        map.insert(3, 4);
        assert_eq!(map, dec.decode::<BTreeMap<u8, u8>>().unwrap());

        assert_eq!(Some(1u32), dec.decode::<Option<u32>>().unwrap());
        assert_eq!(None, dec.decode::<Option<u32>>().unwrap());
        assert_eq!(Ok(1u32), dec.decode::<Result<u32, String>>().unwrap());
        assert_eq!(
            Err("hello".to_owned()),
            dec.decode::<Result<u32, String>>().unwrap()
        );
    }

    #[test]
    pub fn test_decoding() {
        let bytes = vec![
            0, 0, // unit
            1, 1, // bool
            2, 1, // i8
            3, 1, 0, // i16
            4, 1, 0, 0, 0, // i32
            5, 1, 0, 0, 0, 0, 0, 0, 0, // i64
            6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // i128
            7, 1, // u8
            8, 1, 0, // u16
            9, 1, 0, 0, 0, // u32
            10, 1, 0, 0, 0, 0, 0, 0, 0, // u64
            11, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // u128
            12, 5, 104, 101, 108, 108, 111, // string
            32, 9, 3, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // array
            33, 2, 9, 1, 0, 0, 0, 9, 2, 0, 0, 0, // tuple
            32, 9, 3, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // vec
            32, 7, 2, 1, 2, // set
            32, 33, 2, 2, 7, 1, 7, 2, 2, 7, 3, 7, 4, // map
            17, 4, 83, 111, 109, 101, 1, 9, 1, 0, 0, 0, // Some<T>
            17, 4, 78, 111, 110, 101, 0, // None
            17, 2, 79, 107, 1, 9, 1, 0, 0, 0, // Ok<T>
            17, 3, 69, 114, 114, 1, 12, 5, 104, 101, 108, 108, 111, // Err<T>
        ];
        let mut dec = VecDecoder::<NoCustomTypeId>::new(&bytes);
        assert_decoding(&mut dec);
    }

    #[test]
    pub fn test_decode_box() {
        let bytes = vec![7u8, 5u8];
        let mut dec = VecDecoder::<NoCustomTypeId>::new(&bytes);
        let x = dec.decode::<Box<u8>>().unwrap();
        assert_eq!(Box::new(5u8), x);
    }

    #[test]
    pub fn test_decode_rc() {
        let bytes = vec![7u8, 5u8];
        let mut dec = VecDecoder::<NoCustomTypeId>::new(&bytes);
        let x = dec.decode::<Rc<u8>>().unwrap();
        assert_eq!(Rc::new(5u8), x);
    }

    #[test]
    pub fn test_decode_ref_cell() {
        let bytes = vec![7u8, 5u8];
        let mut dec = VecDecoder::<NoCustomTypeId>::new(&bytes);
        let x = dec.decode::<RefCell<u8>>().unwrap();
        assert_eq!(RefCell::new(5u8), x);
    }

    #[derive(sbor::TypeId, sbor::Encode, sbor::Decode, PartialEq, Eq, Debug)]
    struct NFA {
        a: [u8; 32],
        b: Vec<u8>,
    }

    #[test]
    pub fn test_generic_array() {
        let value1 = [
            NFA {
                a: [1u8; 32],
                b: vec![1],
            },
            NFA {
                a: [2u8; 32],
                b: vec![2],
            },
        ];

        // Encode
        let mut bytes = Vec::with_capacity(512);
        let mut enc = Encoder::<NoCustomTypeId>::new(&mut bytes);
        value1.encode(&mut enc);

        let mut dec = VecDecoder::<NoCustomTypeId>::new(&bytes);
        let value2 = dec.decode::<[NFA; 2]>().unwrap();
        assert_eq!(value1, value2);
    }
}
