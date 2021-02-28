use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum IngredientCategory {
    Boucherie,
    Poissonerie,
    Charcuterie,
    Boulangerie,
    Patisserie,
    Cremerie,
    Fromagerie,
    FruitEtLegumes,
    Epicerie,
}

impl std::fmt::Display for IngredientCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Boucherie => write!(f, "Boucherie"),
            Self::Poissonerie => write!(f, "Poissonerie"),
            Self::Charcuterie => write!(f, "Charcuterie"),
            Self::Boulangerie => write!(f, "Boulangerie"),
            Self::Patisserie => write!(f, "Pâtisserie"),
            Self::Cremerie => write!(f, "Crèmerie"),
            Self::Fromagerie => write!(f, "Fromagerie"),
            Self::FruitEtLegumes => write!(f, "Fruit et légumes"),
            Self::Epicerie => write!(f, "Épicerie"),
        }
    }
}

use IngredientCategory::*;

#[derive(Clone, Copy)]
pub struct IngredientInfo {
    pub icon: &'static str,
    pub category: IngredientCategory,
}

impl IngredientInfo {
    pub fn new(icon: &'static str, category: IngredientCategory) -> Self {
        Self { icon, category }
    }
}

lazy_static! {
    static ref ICONS: HashMap<&'static str, IngredientInfo> = [
        (
            "abricot",
            IngredientInfo::new("/icons/icons8-abricot-96.png", FruitEtLegumes)
        ),
        (
            "agrumes",
            IngredientInfo::new("/icons/icons8-agrumes-96.png", FruitEtLegumes)
        ),
        (
            "ananas",
            IngredientInfo::new("/icons/icons8-ananas-96.png", FruitEtLegumes)
        ),
        (
            "asperges",
            IngredientInfo::new("/icons/icons8-asperges-96.png", FruitEtLegumes)
        ),
        (
            "avocat",
            IngredientInfo::new("/icons/icons8-avocat-96.png", FruitEtLegumes)
        ),
        (
            "bacon",
            IngredientInfo::new("/icons/icons8-bacon-96.png", Boucherie)
        ),
        (
            "banane",
            IngredientInfo::new("/icons/icons8-banane-96.png", FruitEtLegumes)
        ),
        (
            "beurre",
            IngredientInfo::new("/icons/icons8-beurre-96.png", Cremerie)
        ),
        (
            "blueberry",
            IngredientInfo::new("/icons/icons8-blueberry-96.png", FruitEtLegumes)
        ),
        (
            "cacahuètes",
            IngredientInfo::new("/icons/icons8-cacahuètes-96.png", FruitEtLegumes)
        ),
        (
            "carotte",
            IngredientInfo::new("/icons/icons8-carotte-96.png", FruitEtLegumes)
        ),
        (
            "cerise",
            IngredientInfo::new("/icons/icons8-cerise-96.png", FruitEtLegumes)
        ),
        (
            "chou-fleur",
            IngredientInfo::new("/icons/icons8-chou-fleur-96.png", FruitEtLegumes)
        ),
        (
            "crabe",
            IngredientInfo::new("/icons/icons8-crabe-96.png", Poissonerie)
        ),
        (
            "crevette",
            IngredientInfo::new("/icons/icons8-crevette-96.png", Poissonerie)
        ),
        (
            "boeuf",
            IngredientInfo::new("/icons/icons8-cuts-of-beef-96.png", Boucherie)
        ),
        (
            "porc",
            IngredientInfo::new("/icons/icons8-cuts-of-pork-96.png", Boucherie)
        ),
        (
            "date",
            IngredientInfo::new("/icons/icons8-date-fruit-96.png", FruitEtLegumes)
        ),
        (
            "durian",
            IngredientInfo::new("/icons/icons8-durian-96.png", FruitEtLegumes)
        ),
        (
            "fenouil",
            IngredientInfo::new("/icons/icons8-finocchio-96.png", FruitEtLegumes)
        ),
        (
            "farine",
            IngredientInfo::new("/icons/icons8-flour-in-paper-packaging-96.png", Epicerie)
        ),
        (
            "fraise",
            IngredientInfo::new("/icons/icons8-fraise-96.png", FruitEtLegumes)
        ),
        (
            "framboise",
            IngredientInfo::new("/icons/icons8-framboise-96.png", FruitEtLegumes)
        ),
        (
            "fruit du dragon",
            IngredientInfo::new("/icons/icons8-fruit-du-dragon-96.png", FruitEtLegumes)
        ),
        (
            "fruits-de-mer",
            IngredientInfo::new("/icons/icons8-fruits-de-mer-96.png", Poissonerie)
        ),
        (
            "riz",
            IngredientInfo::new("/icons/icons8-grains-of-rice-96.png", Epicerie)
        ),
        (
            "grenade",
            IngredientInfo::new("/icons/icons8-grenade-96.png", FruitEtLegumes)
        ),
        (
            "orange",
            IngredientInfo::new("/icons/icons8-half-orange-96.png", FruitEtLegumes)
        ),
        (
            "ketchup",
            IngredientInfo::new("/icons/icons8-ketchup-96.png", Epicerie)
        ),
        (
            "kiwi",
            IngredientInfo::new("/icons/icons8-kiwi-96.png", FruitEtLegumes)
        ),
        (
            "lime",
            IngredientInfo::new("/icons/icons8-lime-96.png", FruitEtLegumes)
        ),
        (
            "lychee",
            IngredientInfo::new("/icons/icons8-lychee-96.png", FruitEtLegumes)
        ),
        (
            "mangouste",
            IngredientInfo::new("/icons/icons8-mangosteen-96.png", FruitEtLegumes)
        ),
        (
            "mangue",
            IngredientInfo::new("/icons/icons8-mangue-96.png", FruitEtLegumes)
        ),
        (
            "melon",
            IngredientInfo::new("/icons/icons8-melon-96.png", FruitEtLegumes)
        ),
        (
            "miel",
            IngredientInfo::new("/icons/icons8-miel-96.png", FruitEtLegumes)
        ),
        (
            "moutarde",
            IngredientInfo::new("/icons/icons8-moutarde-96.png", Epicerie)
        ),
        (
            "naan",
            IngredientInfo::new("/icons/icons8-naan-96.png", Boulangerie)
        ),
        (
            "noisette",
            IngredientInfo::new("/icons/icons8-noisette-96.png", FruitEtLegumes)
        ),
        (
            "noix de coco",
            IngredientInfo::new("/icons/icons8-noix-de-coco-96.png", FruitEtLegumes)
        ),
        (
            "oeuf",
            IngredientInfo::new("/icons/icons8-œuf-96.png", Epicerie)
        ),
        (
            "oignon",
            IngredientInfo::new("/icons/icons8-oignon-96.png", FruitEtLegumes)
        ),
        (
            "huile d'olive",
            IngredientInfo::new("/icons/icons8-olive-oil-bottle-96.png", Epicerie)
        ),
        (
            "pain",
            IngredientInfo::new("/icons/icons8-pain-96.png", Boulangerie)
        ),
        (
            "papaye",
            IngredientInfo::new("/icons/icons8-papaye-96.png", FruitEtLegumes)
        ),
        (
            "paprika",
            IngredientInfo::new("/icons/icons8-paprika-96.png", Epicerie)
        ),
        (
            "pastèque",
            IngredientInfo::new("/icons/icons8-pastèque-96.png", FruitEtLegumes)
        ),
        (
            "pecan",
            IngredientInfo::new("/icons/icons8-pecan-96.png", FruitEtLegumes)
        ),
        (
            "pêche",
            IngredientInfo::new("/icons/icons8-pêche-96.png", FruitEtLegumes)
        ),
        (
            "poire",
            IngredientInfo::new("/icons/icons8-poire-96.png", FruitEtLegumes)
        ),
        (
            "pomme",
            IngredientInfo::new("/icons/icons8-pomme-96.png", FruitEtLegumes)
        ),
        (
            "pomme de terre",
            IngredientInfo::new("/icons/icons8-pomme-de-terre-96.png", FruitEtLegumes)
        ),
        (
            "poulpe",
            IngredientInfo::new("/icons/icons8-poulpe-96.png", Poissonerie)
        ),
        (
            "prune",
            IngredientInfo::new("/icons/icons8-prune-96.png", FruitEtLegumes)
        ),
        (
            "raisins",
            IngredientInfo::new("/icons/icons8-raisins-96.png", FruitEtLegumes)
        ),
        (
            "saucisse",
            IngredientInfo::new("/icons/icons8-saucisses-96.png", Boucherie)
        ),
        (
            "sauce soja",
            IngredientInfo::new("/icons/icons8-soy-sauce-96.png", Epicerie)
        ),
        (
            "steak",
            IngredientInfo::new("/icons/icons8-steak-96.png", Boucherie)
        ),
        (
            "sucre",
            IngredientInfo::new("/icons/icons8-sucre-96.png", Epicerie)
        ),
        (
            "huile de tournesol",
            IngredientInfo::new("/icons/icons8-sunflower-oil-96.png", Epicerie)
        ),
    ]
    .iter()
    .copied()
    .collect();
}

pub fn ingredient_info(name: &str) -> Option<IngredientInfo> {
    ICONS.get(name).copied()
}
