use use_serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::*;

impl<T> Serialize for TaggedId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        Uuid::serialize(&self.inner, serializer)
    }
}

impl<'de, T> Deserialize<'de> for TaggedId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Uuid::deserialize(deserializer)
            .map(TaggedId::from_uuid)
            .map_err(|_| {
                de::Error::custom("Invalid id format")
            })
    }
}