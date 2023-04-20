use super::*;
use crate::address::Bech32Encoder;
use crate::*;
use sbor::representations::*;
use sbor::rust::prelude::*;
use sbor::traversal::*;
use utils::ContextualDisplay;

#[derive(Clone, Copy, Debug, Default)]
pub struct ScryptoValueDisplayContext<'a> {
    pub bech32_encoder: Option<&'a Bech32Encoder>,
}

impl<'a> ScryptoValueDisplayContext<'a> {
    pub fn no_context() -> Self {
        Self {
            bech32_encoder: None,
        }
    }

    pub fn with_optional_bech32(bech32_encoder: Option<&'a Bech32Encoder>) -> Self {
        Self { bech32_encoder }
    }
}

impl<'a> Into<ScryptoValueDisplayContext<'a>> for &'a Bech32Encoder {
    fn into(self) -> ScryptoValueDisplayContext<'a> {
        ScryptoValueDisplayContext::with_optional_bech32(Some(self))
    }
}

impl<'a> Into<ScryptoValueDisplayContext<'a>> for Option<&'a Bech32Encoder> {
    fn into(self) -> ScryptoValueDisplayContext<'a> {
        ScryptoValueDisplayContext::with_optional_bech32(self)
    }
}

impl<'a> CustomDisplayContext<'a> for ScryptoValueDisplayContext<'a> {
    type CustomTypeExtension = ScryptoCustomTypeExtension;
}

impl FormattableCustomTypeExtension for ScryptoCustomTypeExtension {
    type CustomDisplayContext<'a> = ScryptoValueDisplayContext<'a>;

    fn display_string_content<'s, 'de, 'a, 't, 's1, 's2, F: fmt::Write>(
        f: &mut F,
        context: &Self::CustomDisplayContext<'a>,
        value: &<Self::CustomTraversal as CustomTraversal>::CustomTerminalValueRef<'de>,
    ) -> Result<(), fmt::Error> {
        match &value.0 {
            ScryptoCustomValue::Reference(value) => {
                write!(f, "\"{}\"", value.0.display(context.bech32_encoder))?;
            }
            ScryptoCustomValue::Own(value) => {
                write!(f, "\"{}\"", value.0.display(context.bech32_encoder))?;
            }
            ScryptoCustomValue::Decimal(value) => {
                write!(f, "\"{}\"", value)?;
            }
            ScryptoCustomValue::PreciseDecimal(value) => {
                write!(f, "\"{}\"", value)?;
            }
            ScryptoCustomValue::NonFungibleLocalId(value) => {
                write!(f, "\"{}\"", value)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Bech32Encoder;
    use crate::data::scrypto::model::*;
    use crate::types::NodeId;

    #[test]
    fn test_rustlike_string_format_with_network() {
        use crate::math::{Decimal, PreciseDecimal};

        let encoder = Bech32Encoder::for_simulator();
        let value = ScryptoValue::Tuple {
            fields: vec![
                Value::Custom {
                    value: ScryptoCustomValue::Reference(Reference(NodeId([0; NodeId::LENGTH]))),
                },
                Value::Custom {
                    value: ScryptoCustomValue::Own(Own(NodeId([0; NodeId::LENGTH]))),
                },
                Value::Custom {
                    value: ScryptoCustomValue::Decimal(Decimal::ONE),
                },
                Value::Custom {
                    value: ScryptoCustomValue::Decimal(Decimal::ONE / 100),
                },
                Value::Custom {
                    value: ScryptoCustomValue::PreciseDecimal(PreciseDecimal::ZERO),
                },
                Value::Custom {
                    value: ScryptoCustomValue::NonFungibleLocalId(
                        NonFungibleLocalId::string("hello").unwrap(),
                    ),
                },
                Value::Custom {
                    value: ScryptoCustomValue::NonFungibleLocalId(NonFungibleLocalId::integer(123)),
                },
                Value::Custom {
                    value: ScryptoCustomValue::NonFungibleLocalId(
                        NonFungibleLocalId::bytes(vec![0x23, 0x45]).unwrap(),
                    ),
                },
                Value::Custom {
                    value: ScryptoCustomValue::NonFungibleLocalId(
                        NonFungibleLocalId::uuid(0x1f52cb1e_86c4_47ae_9847_9cdb14662ebd).unwrap(),
                    ),
                },
            ],
        };

        let expected = "Tuple(Reference(\"package_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq57ks9j\"), Own(\"package_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq57ks9j\"), Decimal(\"1\"), Decimal(\"0.01\"), PreciseDecimal(\"0\"), NonFungibleLocalId(\"<hello>\"), NonFungibleLocalId(\"#123#\"), NonFungibleLocalId(\"[2345]\"), NonFungibleLocalId(\"{1f52cb1e-86c4-47ae-9847-9cdb14662ebd}\"))";

        let context = ScryptoValueDisplayContext::with_optional_bech32(Some(&encoder));

        let payload = ScryptoRawPayload::new_from_valid_owned(scrypto_encode(&value).unwrap());

        let actual_rustlike = payload.to_string(ValueDisplayParameters::Schemaless {
            display_mode: DisplayMode::RustLike,
            print_mode: PrintMode::SingleLine,
            custom_display_context: context,
        });
        let actual_nested = payload.to_string(ValueDisplayParameters::Schemaless {
            display_mode: DisplayMode::RustLike,
            print_mode: PrintMode::SingleLine,
            custom_display_context: context,
        });

        // They're both the same
        assert_eq!(&actual_rustlike, expected);
        assert_eq!(&actual_nested, expected);
    }
}
