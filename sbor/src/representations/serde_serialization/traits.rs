use super::*;
use crate::representations::*;
use crate::rust::prelude::*;
use crate::traversal::*;
use crate::*;

pub struct CustomTypeSerialization<'a, 't, 'de, 's1, 's2, 'c, E: SerializableCustomTypeExtension> {
    pub include_type_tag_in_simple_mode: bool,
    pub serialization: SerializableType<'a, 't, 'de, 's1, 's2, 'c, E>,
}

// Note - the Copy here is to work around the dodgy derive implementation of Copy on SerializationContext
pub trait SerializableCustomTypeExtension: FormattableCustomTypeExtension + Copy {
    fn map_value_for_serialization<'s, 'de, 'a, 't, 's1, 's2, 'c>(
        context: &SerializationContext<'s, 'a, 'c, Self>,
        type_index: LocalTypeIndex,
        value: <Self::CustomTraversal as CustomTraversal>::CustomTerminalValueRef<'de>,
    ) -> CustomTypeSerialization<'a, 't, 'de, 's1, 's2, 'c, Self>;
}
