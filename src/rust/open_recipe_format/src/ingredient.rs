use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amounts: Vec<IngredientAmount>,
    pub notes: Option<Vec<String>>,
    pub usda_num: Option<String>,
    pub processing: Option<Vec<String>>,
    pub substitutions: Option<Vec<Ingredient>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct IngredientFields {
    pub amounts: Vec<IngredientAmount>,
    pub notes: Option<Vec<String>>,
    pub usda_num: Option<String>,
    pub processing: Option<Vec<String>>,
    pub substitutions: Option<Vec<Ingredient>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IngredientAmount {
    pub amount: f64,
    pub unit: String,
}

struct IngredientVisitor {
    marker: PhantomData<fn() -> Ingredient>,
}

impl IngredientVisitor {
    fn new() -> Self {
        IngredientVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for IngredientVisitor {
    type Value = Ingredient;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut ingredient = None;

        while let Some((key, value)) = access.next_entry::<String, IngredientFields>()? {
            ingredient = Some(Ingredient {
                name: key,
                amounts: value.amounts,
                notes: value.notes,
                usda_num: value.usda_num,
                processing: value.processing,
                substitutions: value.substitutions,
            })
        }

        if let Some(ingredient) = ingredient {
            Ok(ingredient)
        } else {
            Err(serde::de::Error::custom("missing ingredient key"))
        }
    }
}

impl<'de> Deserialize<'de> for Ingredient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IngredientVisitor::new())
    }
}

impl Serialize for Ingredient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(
            &self.name,
            &IngredientFields {
                amounts: self.amounts.clone(),
                notes: self.notes.clone(),
                usda_num: self.usda_num.clone(),
                processing: self.processing.clone(),
                substitutions: self.substitutions.clone(),
            },
        )?;
        map.end()
    }
}
