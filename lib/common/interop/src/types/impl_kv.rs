use serde::ser::SerializeMap;
use serde::Serialize;

use super::KV;

impl<K, V> Serialize for KV<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut m = serializer.serialize_map(Some(1))?;
        m.serialize_key(&self.0)?;
        m.serialize_value(&self.1)?;
        m.end()
    }
}
