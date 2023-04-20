use super::*;
use crate::rust::prelude::*;
use crate::traversal::*;
use crate::*;
use serde::Serializer;
use utils::*;

pub enum SerializationParameters<'s, 'a, E: SerializableCustomTypeExtension> {
    Schemaless {
        mode: SerializationMode,
        custom_display_context: E::CustomDisplayContext<'a>,
    },
    WithSchema {
        mode: SerializationMode,
        custom_display_context: E::CustomDisplayContext<'a>,
        schema: &'s Schema<E>,
        type_index: LocalTypeIndex,
    },
}

impl<'s, 'a, E: SerializableCustomTypeExtension> SerializationParameters<'s, 'a, E> {
    pub fn get_context_and_type_index(&self) -> (SerializationContext<'s, 'a, E>, LocalTypeIndex) {
        match self {
            SerializationParameters::Schemaless {
                mode,
                custom_display_context,
            } => (
                SerializationContext {
                    schema: E::empty_schema(),
                    mode: *mode,
                    custom_display_context: *custom_display_context,
                },
                LocalTypeIndex::any(),
            ),
            SerializationParameters::WithSchema {
                mode,
                custom_display_context,
                schema,
                type_index,
            } => (
                SerializationContext {
                    schema: *schema,
                    mode: *mode,
                    custom_display_context: *custom_display_context,
                },
                *type_index,
            ),
        }
    }
}

impl<'s, 'a, 'b, E: SerializableCustomTypeExtension>
    ContextualSerialize<SerializationParameters<'s, 'a, E>> for RawPayload<'b, E>
{
    fn contextual_serialize<S: Serializer>(
        &self,
        serializer: S,
        context: &SerializationParameters<'s, 'a, E>,
    ) -> Result<S::Ok, S::Error> {
        let (context, type_index) = context.get_context_and_type_index();
        serialize_payload(serializer, self.payload_bytes(), &context, type_index)
    }
}

impl<'s, 'a, 'b, E: SerializableCustomTypeExtension>
    ContextualSerialize<SerializationParameters<'s, 'a, E>> for RawValue<'b, E>
{
    fn contextual_serialize<S: Serializer>(
        &self,
        serializer: S,
        context: &SerializationParameters<'s, 'a, E>,
    ) -> Result<S::Ok, S::Error> {
        let (context, type_index) = context.get_context_and_type_index();
        serialize_partial_payload(
            serializer,
            self.value_body_bytes(),
            ExpectedStart::ValueBody(self.value_kind()),
            true,
            0,
            &context,
            type_index,
        )
    }
}
