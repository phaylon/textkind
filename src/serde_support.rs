
use std::fmt;

use serde;

struct Error<K>
where
    K: ::Kind
{
    inner: ::Error<K>,
}

impl<K> fmt::Display for Error<K>
where
    K: ::Kind,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} because {}", self.inner, self.inner.0)
    }
}

impl<'de, K, D> serde::Deserialize<'de> for ::Text<K, D>
where
    K: ::Kind,
    D: ::Dynamic,
    D: serde::Deserialize<'de>,
{
    fn deserialize<T>(deserializer: T) -> Result<::Text<K, D>, T::Error>
    where
        T: serde::Deserializer<'de>,
    {
        let value = D::deserialize(deserializer)?;
        ::Text::try_from_dynamic(value).map_err(|error| serde::de::Error::custom(Error {
            inner: error.without_value(),
        }))
    }
}

impl<K, D> serde::Serialize for ::Text<K, D>
where
    K: ::Kind,
    D: ::Dynamic,
{
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
