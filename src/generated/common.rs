// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::doc_markdown,
)]

pub type Date = String /*Timestamp*/;
pub type DisplayName = String;
pub type DisplayNameLegacy = String;
pub type DropboxTimestamp = String /*Timestamp*/;
pub type EmailAddress = String;
pub type LanguageCode = String;
pub type NamePart = String;
pub type NamespaceId = String;
pub type OptionalNamePart = String;
pub type SessionId = String;
pub type SharedFolderId = NamespaceId;

#[derive(Debug)]
pub enum PathRoot {
    /// Paths are relative to the authenticating user's home namespace, whether or not that user
    /// belongs to a team.
    Home,
    /// Paths are relative to the authenticating user's root namespace (This results in
    /// [`PathRootError::InvalidRoot`](PathRootError::InvalidRoot) if the user's root namespace has
    /// changed.).
    Root(NamespaceId),
    /// Paths are relative to given namespace id (This results in
    /// [`PathRootError::NoPermission`](PathRootError::NoPermission) if you don't have access to
    /// this namespace.).
    NamespaceId(NamespaceId),
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PathRoot {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PathRoot;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a PathRoot structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "home" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(PathRoot::Home)
                    }
                    "root" => {
                        match map.next_key()? {
                            Some("root") => Ok(PathRoot::Root(map.next_value()?)),
                            None => Err(de::Error::missing_field("root")),
                            _ => Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    "namespace_id" => {
                        match map.next_key()? {
                            Some("namespace_id") => Ok(PathRoot::NamespaceId(map.next_value()?)),
                            None => Err(de::Error::missing_field("namespace_id")),
                            _ => Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    _ => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(PathRoot::Other)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["home",
                                    "root",
                                    "namespace_id",
                                    "other"];
        deserializer.deserialize_struct("PathRoot", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PathRoot {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PathRoot::Home => {
                // unit
                let mut s = serializer.serialize_struct("PathRoot", 1)?;
                s.serialize_field(".tag", "home")?;
                s.end()
            }
            PathRoot::Root(ref x) => {
                // primitive
                let mut s = serializer.serialize_struct("PathRoot", 2)?;
                s.serialize_field(".tag", "root")?;
                s.serialize_field("root", x)?;
                s.end()
            }
            PathRoot::NamespaceId(ref x) => {
                // primitive
                let mut s = serializer.serialize_struct("PathRoot", 2)?;
                s.serialize_field(".tag", "namespace_id")?;
                s.serialize_field("namespace_id", x)?;
                s.end()
            }
            PathRoot::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum PathRootError {
    /// The root namespace id in Dropbox-API-Path-Root header is not valid. The value of this error
    /// is use's latest root info.
    InvalidRoot(RootInfo),
    /// You don't have permission to access the namespace id in Dropbox-API-Path-Root  header.
    NoPermission,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PathRootError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PathRootError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a PathRootError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "invalid_root" => {
                        match map.next_key()? {
                            Some("invalid_root") => Ok(PathRootError::InvalidRoot(map.next_value()?)),
                            None => Err(de::Error::missing_field("invalid_root")),
                            _ => Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    "no_permission" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(PathRootError::NoPermission)
                    }
                    _ => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(PathRootError::Other)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["invalid_root",
                                    "no_permission",
                                    "other"];
        deserializer.deserialize_struct("PathRootError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PathRootError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PathRootError::InvalidRoot(ref x) => {
                // union or polymporphic struct
                let mut s = serializer.serialize_struct("PathRootError", 2)?;
                s.serialize_field(".tag", "invalid_root")?;
                s.serialize_field("invalid_root", x)?;
                s.end()
            }
            PathRootError::NoPermission => {
                // unit
                let mut s = serializer.serialize_struct("PathRootError", 1)?;
                s.serialize_field(".tag", "no_permission")?;
                s.end()
            }
            PathRootError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for PathRootError {
    fn description(&self) -> &str {
        "PathRootError"
    }
}

impl ::std::fmt::Display for PathRootError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

/// Information about current user's root.
#[derive(Debug)]
pub enum RootInfo {
    Team(TeamRootInfo),
    User(UserRootInfo),
    _Unknown
}

impl<'de> ::serde::de::Deserialize<'de> for RootInfo {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // polymorphic struct deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = RootInfo;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a RootInfo structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "team" => Ok(RootInfo::Team(TeamRootInfo::internal_deserialize(map)?)),
                    "user" => Ok(RootInfo::User(UserRootInfo::internal_deserialize(map)?)),
                    _ => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(RootInfo::_Unknown)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["user",
                                    "user"];
        deserializer.deserialize_struct("RootInfo", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for RootInfo {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // polymorphic struct serializer
        use serde::ser::SerializeStruct;
        match *self {
            RootInfo::Team(ref x) => {
                let mut s = serializer.serialize_struct("RootInfo", 4)?;
                s.serialize_field(".tag", "team")?;
                s.serialize_field("root_namespace_id", &x.root_namespace_id)?;
                s.serialize_field("home_namespace_id", &x.home_namespace_id)?;
                s.serialize_field("home_path", &x.home_path)?;
                s.end()
            }
            RootInfo::User(ref x) => {
                let mut s = serializer.serialize_struct("RootInfo", 3)?;
                s.serialize_field(".tag", "user")?;
                s.serialize_field("root_namespace_id", &x.root_namespace_id)?;
                s.serialize_field("home_namespace_id", &x.home_namespace_id)?;
                s.end()
            }
            RootInfo::_Unknown => Err(::serde::ser::Error::custom("cannot serialize unknown variant"))
        }
    }
}

/// Root info when user is member of a team with a separate root namespace ID.
#[derive(Debug)]
pub struct TeamRootInfo {
    /// The namespace ID for user's root namespace. It will be the namespace ID of the shared team
    /// root if the user is member of a team with a separate team root. Otherwise it will be same as
    /// `home_namespace_id`.
    pub root_namespace_id: NamespaceId,
    /// The namespace ID for user's home namespace.
    pub home_namespace_id: NamespaceId,
    /// The path for user's home directory under the shared team root.
    pub home_path: String,
}

impl TeamRootInfo {
    pub fn new(
        root_namespace_id: NamespaceId,
        home_namespace_id: NamespaceId,
        home_path: String,
    ) -> Self {
        TeamRootInfo {
            root_namespace_id,
            home_namespace_id,
            home_path,
        }
    }

}

const TEAM_ROOT_INFO_FIELDS: &[&str] = &["root_namespace_id",
                                         "home_namespace_id",
                                         "home_path"];
impl TeamRootInfo {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<TeamRootInfo, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<TeamRootInfo>, V::Error> {
        let mut field_root_namespace_id = None;
        let mut field_home_namespace_id = None;
        let mut field_home_path = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "root_namespace_id" => {
                    if field_root_namespace_id.is_some() {
                        return Err(::serde::de::Error::duplicate_field("root_namespace_id"));
                    }
                    field_root_namespace_id = Some(map.next_value()?);
                }
                "home_namespace_id" => {
                    if field_home_namespace_id.is_some() {
                        return Err(::serde::de::Error::duplicate_field("home_namespace_id"));
                    }
                    field_home_namespace_id = Some(map.next_value()?);
                }
                "home_path" => {
                    if field_home_path.is_some() {
                        return Err(::serde::de::Error::duplicate_field("home_path"));
                    }
                    field_home_path = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = TeamRootInfo {
            root_namespace_id: field_root_namespace_id.ok_or_else(|| ::serde::de::Error::missing_field("root_namespace_id"))?,
            home_namespace_id: field_home_namespace_id.ok_or_else(|| ::serde::de::Error::missing_field("home_namespace_id"))?,
            home_path: field_home_path.ok_or_else(|| ::serde::de::Error::missing_field("home_path"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("root_namespace_id", &self.root_namespace_id)?;
        s.serialize_field("home_namespace_id", &self.home_namespace_id)?;
        s.serialize_field("home_path", &self.home_path)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TeamRootInfo {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TeamRootInfo;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a TeamRootInfo struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TeamRootInfo::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TeamRootInfo", TEAM_ROOT_INFO_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TeamRootInfo {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TeamRootInfo", 3)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

/// Root info when user is not member of a team or the user is a member of a team and the team does
/// not have a separate root namespace.
#[derive(Debug)]
pub struct UserRootInfo {
    /// The namespace ID for user's root namespace. It will be the namespace ID of the shared team
    /// root if the user is member of a team with a separate team root. Otherwise it will be same as
    /// `home_namespace_id`.
    pub root_namespace_id: NamespaceId,
    /// The namespace ID for user's home namespace.
    pub home_namespace_id: NamespaceId,
}

impl UserRootInfo {
    pub fn new(root_namespace_id: NamespaceId, home_namespace_id: NamespaceId) -> Self {
        UserRootInfo {
            root_namespace_id,
            home_namespace_id,
        }
    }

}

const USER_ROOT_INFO_FIELDS: &[&str] = &["root_namespace_id",
                                         "home_namespace_id"];
impl UserRootInfo {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<UserRootInfo, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<UserRootInfo>, V::Error> {
        let mut field_root_namespace_id = None;
        let mut field_home_namespace_id = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "root_namespace_id" => {
                    if field_root_namespace_id.is_some() {
                        return Err(::serde::de::Error::duplicate_field("root_namespace_id"));
                    }
                    field_root_namespace_id = Some(map.next_value()?);
                }
                "home_namespace_id" => {
                    if field_home_namespace_id.is_some() {
                        return Err(::serde::de::Error::duplicate_field("home_namespace_id"));
                    }
                    field_home_namespace_id = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = UserRootInfo {
            root_namespace_id: field_root_namespace_id.ok_or_else(|| ::serde::de::Error::missing_field("root_namespace_id"))?,
            home_namespace_id: field_home_namespace_id.ok_or_else(|| ::serde::de::Error::missing_field("home_namespace_id"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("root_namespace_id", &self.root_namespace_id)?;
        s.serialize_field("home_namespace_id", &self.home_namespace_id)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for UserRootInfo {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = UserRootInfo;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a UserRootInfo struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                UserRootInfo::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("UserRootInfo", USER_ROOT_INFO_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for UserRootInfo {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("UserRootInfo", 2)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

