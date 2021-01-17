mod ingredient;
mod yields;

use anyhow::Result;
pub use ingredient::*;
use serde::{Deserialize, Serialize};
pub use yields::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OpenRecipe {
    pub recipe_name: String,
    pub recipe_uuid: Option<String>,
    pub ingredients: Vec<Ingredient>,
    pub steps: Option<Vec<OpenRecipeStep>>,
    pub yields: Option<Vec<Yield>>,
    pub notes: Option<Vec<String>>,
    pub oven_fan: Option<OvenFan>,
    pub oven_temp: Option<OvenTemp>,
    pub source_url: Option<String>,
    pub source_authors: Option<Vec<String>>,
    pub source_book: Option<Book>,
}

impl OpenRecipe {
    pub fn parse(recipe: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(recipe)?)
    }

    pub fn dump(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OpenRecipeStep {
    pub step: String,
    pub notes: Option<Vec<String>>,
    pub haccp: Option<Haccp>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Haccp {
    pub control_point: Option<String>,
    pub critical_control_point: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum OvenFan {
    Off,
    Low,
    High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OvenTemp {
    pub amount: u16,
    pub unit: OvenTempUnit,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum OvenTempUnit {
    C,
    F,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Book {
    pub authors: Option<Vec<String>>,
    pub title: String,
    pub isbn: Option<String>,
    pub notes: Option<Vec<String>>,
}
