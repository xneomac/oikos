use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone)]
pub struct Yield {
    pub unit: String,
    pub amount: u64,
}

struct YieldVisitor {
    marker: PhantomData<fn() -> Yield>,
}

impl YieldVisitor {
    fn new() -> Self {
        YieldVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for YieldVisitor {
    type Value = Yield;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut item = None;

        while let Some((key, value)) = access.next_entry::<String, u64>()? {
            item = Some(Yield {
                unit: key,
                amount: value,
            })
        }

        if let Some(item) = item {
            Ok(item)
        } else {
            Err(serde::de::Error::custom("missing yield key"))
        }
    }
}

impl<'de> Deserialize<'de> for Yield {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(YieldVisitor::new())
    }
}

impl Serialize for Yield {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.unit, &self.amount)?;
        map.end()
    }
}
