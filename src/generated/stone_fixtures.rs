// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::doc_markdown,
)]

/// This struct left intentionally empty
#[derive(Debug)]
pub struct EmptyContainer {
}

impl Default for EmptyContainer {
    fn default() -> Self {
        EmptyContainer {
        }
    }
}

const EMPTY_CONTAINER_FIELDS: &[&str] = &[];
impl EmptyContainer {
    // no _opt deserializer
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
    ) -> Result<EmptyContainer, V::Error> {
        // ignore any fields found; none are presently recognized
        crate::eat_json_fields(&mut map)?;
        Ok(EmptyContainer {})
    }
}

impl<'de> ::serde::de::Deserialize<'de> for EmptyContainer {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = EmptyContainer;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a EmptyContainer struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                EmptyContainer::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("EmptyContainer", EMPTY_CONTAINER_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for EmptyContainer {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        serializer.serialize_struct("EmptyContainer", 0)?.end()
    }
}

#[derive(Debug)]
pub struct MixedInternalOnlyContainer {
    pub public_value: i32,
}

impl Default for MixedInternalOnlyContainer {
    fn default() -> Self {
        MixedInternalOnlyContainer {
            public_value: 0,
        }
    }
}

const MIXED_INTERNAL_ONLY_CONTAINER_FIELDS: &[&str] = &["public_value"];
impl MixedInternalOnlyContainer {
    // no _opt deserializer
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
    ) -> Result<MixedInternalOnlyContainer, V::Error> {
        let mut field_public_value = None;
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "public_value" => {
                    if field_public_value.is_some() {
                        return Err(::serde::de::Error::duplicate_field("public_value"));
                    }
                    field_public_value = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        let result = MixedInternalOnlyContainer {
            public_value: field_public_value.unwrap_or(0),
        };
        Ok(result)
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("public_value", &self.public_value)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for MixedInternalOnlyContainer {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = MixedInternalOnlyContainer;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a MixedInternalOnlyContainer struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                MixedInternalOnlyContainer::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("MixedInternalOnlyContainer", MIXED_INTERNAL_ONLY_CONTAINER_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for MixedInternalOnlyContainer {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("MixedInternalOnlyContainer", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug)]
pub enum MixedInternalOnlyEnum {
    Public,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for MixedInternalOnlyEnum {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = MixedInternalOnlyEnum;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a MixedInternalOnlyEnum structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "public" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(MixedInternalOnlyEnum::Public)
                    }
                    _ => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(MixedInternalOnlyEnum::Other)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["public",
                                    "other"];
        deserializer.deserialize_struct("MixedInternalOnlyEnum", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for MixedInternalOnlyEnum {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            MixedInternalOnlyEnum::Public => {
                // unit
                let mut s = serializer.serialize_struct("MixedInternalOnlyEnum", 1)?;
                s.serialize_field(".tag", "public")?;
                s.end()
            }
            MixedInternalOnlyEnum::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

