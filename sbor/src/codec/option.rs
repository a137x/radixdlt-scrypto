use crate::constants::*;
use crate::value_kind::*;
use crate::*;
use radix_rust::rust::collections::indexmap;

categorize_generic!(Option<T>, <T>, ValueKind::Enum);

impl<X: CustomValueKind, E: Encoder<X>, T: Encode<X, E>> Encode<X, E> for Option<T> {
    #[inline]
    fn encode_value_kind(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.write_value_kind(Self::value_kind())
    }

    #[inline]
    fn encode_body(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Some(v) => {
                encoder.write_discriminator(OPTION_VARIANT_SOME)?;
                encoder.write_size(1)?;
                encoder.encode(v)?;
            }
            None => {
                encoder.write_discriminator(OPTION_VARIANT_NONE)?;
                encoder.write_size(0)?;
            }
        }
        Ok(())
    }
}

impl<X: CustomValueKind, D: Decoder<X>, T: Decode<X, D>> Decode<X, D> for Option<T> {
    #[inline]
    fn decode_body_with_value_kind(
        decoder: &mut D,
        value_kind: ValueKind<X>,
    ) -> Result<Self, DecodeError> {
        decoder.check_preloaded_value_kind(value_kind, Self::value_kind())?;
        let discriminator = decoder.read_discriminator()?;

        match discriminator {
            OPTION_VARIANT_NONE => {
                decoder.read_and_check_size(0)?;
                Ok(None)
            }
            OPTION_VARIANT_SOME => {
                decoder.read_and_check_size(1)?;
                Ok(Some(decoder.decode()?))
            }
            _ => Err(DecodeError::UnknownDiscriminator(discriminator)),
        }
    }
}

impl<C: CustomTypeKind<RustTypeId>, T: Describe<C>> Describe<C> for Option<T> {
    const TYPE_ID: RustTypeId = RustTypeId::novel("Option", &[T::TYPE_ID]);

    fn type_data() -> TypeData<C, RustTypeId> {
        #[allow(unused_imports)]
        use radix_rust::rust::borrow::ToOwned;
        TypeData::enum_variants(
            "Option",
            indexmap![
                OPTION_VARIANT_NONE => TypeData::no_child_names(TypeKind::Tuple {field_types: radix_rust::rust::vec![]}, "None"),
                OPTION_VARIANT_SOME => TypeData::no_child_names(TypeKind::Tuple {field_types: radix_rust::rust::vec![T::TYPE_ID]}, "Some"),
            ],
        )
    }

    fn add_all_dependencies(aggregator: &mut TypeAggregator<C>) {
        aggregator.add_child_type_and_descendents::<T>();
    }
}
