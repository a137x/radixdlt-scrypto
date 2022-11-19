use crate::constants::*;
use crate::type_id::*;
use crate::*;

impl<X: CustomTypeId, T: Encode<X> + TypeId<X>> Encode<X> for Option<T> {
    #[inline]
    fn encode_type_id(&self, encoder: &mut Encoder<X>) {
        encoder.write_type_id(Self::type_id());
    }
    #[inline]
    fn encode_body(&self, encoder: &mut Encoder<X>) {
        match self {
            Some(v) => {
                encoder.write_discriminator(OPTION_VARIANT_SOME);
                encoder.write_size(1);
                v.encode(encoder);
            }
            None => {
                encoder.write_discriminator(OPTION_VARIANT_NONE);
                encoder.write_size(0);
            }
        }
    }
}

impl<X: CustomTypeId, D: Decoder<X>, T: Decode<X, D>> Decode<X, D> for Option<T> {
    fn decode_body_with_type_id(
        decoder: &mut D,
        type_id: SborTypeId<X>,
    ) -> Result<Self, DecodeError> {
        decoder.check_preloaded_type_id(type_id, Self::type_id())?;
        let discriminator = decoder.read_discriminator()?;

        match discriminator.as_ref() {
            OPTION_VARIANT_SOME => {
                decoder.read_and_check_size(1)?;
                Ok(Some(decoder.decode()?))
            }
            OPTION_VARIANT_NONE => {
                decoder.read_and_check_size(0)?;
                Ok(None)
            }
            _ => Err(DecodeError::UnknownDiscriminator(discriminator)),
        }
    }
}
