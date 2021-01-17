use open_recipe_format::*;

#[test]
fn test_deserialize() {
    let recipe_text = r#"
        recipe_name: Giving an Apple to a Friend
        ingredients:
            - apple:
                amounts:
                    - amount: 1
                      unit: each
        steps:
            - step: Give an apple to a friend.
        "#;
    let recipe = OpenRecipe {
        recipe_uuid: None,
        recipe_name: "Giving an Apple to a Friend".to_string(),
        yields: None,
        ingredients: vec![Ingredient {
            name: "apple".to_string(),
            amounts: vec![IngredientAmount {
                amount: 1.0,
                unit: "each".to_string(),
            }],
            notes: None,
            usda_num: None,
            processing: None,
            substitutions: None,
        }],
        steps: Some(vec![OpenRecipeStep {
            step: "Give an apple to a friend.".to_string(),
            notes: None,
            haccp: None,
        }]),
        notes: None,
        oven_fan: None,
        oven_temp: None,
        source_url: None,
        source_authors: None,
        source_book: None,
    };
    let deserialized_recipe = OpenRecipe::parse(&recipe_text).unwrap();
    assert_eq!(deserialized_recipe, recipe);
    let dumped_recipe = deserialized_recipe.dump().unwrap();
    println!("{}", dumped_recipe);
    let deserialized_recipe = OpenRecipe::parse(&dumped_recipe).unwrap();
    assert_eq!(deserialized_recipe, recipe)
}

#[test]
fn test_deserialize_with_servings() {
    let recipe_text = r#"
        recipe_name: Basic Fruit Salad
        yields:
            - servings: 6
        ingredients:
            - apple:
                amounts:
                    - amount: 1
                      unit: each
            - banana:
                amounts:
                    - amount: 1
                      unit: each
            - orange:
                amounts:
                    - amount: 1
                      unit: each
            - grapes:
                amounts:
                    - amount: 1
                      unit: cup
        steps:
            - step: Cut the apple into cubes.
            - step: Cut the banana into slices.
            - step: Peel the orange, and divide into segments.
            - step: Combine all ingredients in a bowl.
            - step: Mix to combine.
        "#;
    let recipe = OpenRecipe {
        recipe_uuid: None,
        recipe_name: "Basic Fruit Salad".to_string(),
        yields: Some(vec![Yield {
            unit: "servings".to_string(),
            amount: 6,
        }]),
        ingredients: vec![
            Ingredient {
                name: "apple".to_string(),
                amounts: vec![IngredientAmount {
                    amount: 1.0,
                    unit: "each".to_string(),
                }],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
            Ingredient {
                name: "banana".to_string(),
                amounts: vec![IngredientAmount {
                    amount: 1.0,
                    unit: "each".to_string(),
                }],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
            Ingredient {
                name: "orange".to_string(),
                amounts: vec![IngredientAmount {
                    amount: 1.0,
                    unit: "each".to_string(),
                }],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
            Ingredient {
                name: "grapes".to_string(),
                amounts: vec![IngredientAmount {
                    amount: 1.0,
                    unit: "cup".to_string(),
                }],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
        ],
        steps: Some(vec![
            OpenRecipeStep {
                step: "Cut the apple into cubes.".to_string(),
                notes: None,
                haccp: None,
            },
            OpenRecipeStep {
                step: "Cut the banana into slices.".to_string(),
                notes: None,
                haccp: None,
            },
            OpenRecipeStep {
                step: "Peel the orange, and divide into segments.".to_string(),
                notes: None,
                haccp: None,
            },
            OpenRecipeStep {
                step: "Combine all ingredients in a bowl.".to_string(),
                notes: None,
                haccp: None,
            },
            OpenRecipeStep {
                step: "Mix to combine.".to_string(),
                notes: None,
                haccp: None,
            },
        ]),
        notes: None,
        oven_fan: None,
        oven_temp: None,
        source_url: None,
        source_authors: None,
        source_book: None,
    };
    let deserialized_recipe: OpenRecipe = OpenRecipe::parse(&recipe_text).unwrap();
    assert_eq!(deserialized_recipe, recipe);
    let dumped_recipe = deserialized_recipe.dump().unwrap();
    println!("{}", dumped_recipe);
    let deserialized_recipe = OpenRecipe::parse(&dumped_recipe).unwrap();
    assert_eq!(deserialized_recipe, recipe)
}

#[test]
fn test_deserialize_with_multiple_servings() {
    let recipe_text = r#"
        recipe_name: Basic Fruit Salad
        yields:
            - servings: 6
            - servings: 18
        ingredients:
            - apple:
                amounts:
                    - amount: 1
                      unit: each
                    - amount: 3
                      unit: each
            - banana:
                amounts:
                    - amount: 1
                      unit: each
                    - amount: 3
                      unit: each
            - orange:
                amounts:
                    - amount: 1
                      unit: each
                    - amount: 3
                      unit: each
            - grapes:
                amounts:
                    - amount: 1
                      unit: cup
                    - amount: 3
                      unit: cup
        "#;
    let recipe = OpenRecipe {
        recipe_uuid: None,
        recipe_name: "Basic Fruit Salad".to_string(),
        yields: Some(vec![
            Yield {
                unit: "servings".to_string(),
                amount: 6,
            },
            Yield {
                unit: "servings".to_string(),
                amount: 18,
            },
        ]),
        ingredients: vec![
            Ingredient {
                name: "apple".to_string(),
                amounts: vec![
                    IngredientAmount {
                        amount: 1.0,
                        unit: "each".to_string(),
                    },
                    IngredientAmount {
                        amount: 3.0,
                        unit: "each".to_string(),
                    },
                ],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
            Ingredient {
                name: "banana".to_string(),
                amounts: vec![
                    IngredientAmount {
                        amount: 1.0,
                        unit: "each".to_string(),
                    },
                    IngredientAmount {
                        amount: 3.0,
                        unit: "each".to_string(),
                    },
                ],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
            Ingredient {
                name: "orange".to_string(),
                amounts: vec![
                    IngredientAmount {
                        amount: 1.0,
                        unit: "each".to_string(),
                    },
                    IngredientAmount {
                        amount: 3.0,
                        unit: "each".to_string(),
                    },
                ],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
            Ingredient {
                name: "grapes".to_string(),
                amounts: vec![
                    IngredientAmount {
                        amount: 1.0,
                        unit: "cup".to_string(),
                    },
                    IngredientAmount {
                        amount: 3.0,
                        unit: "cup".to_string(),
                    },
                ],
                notes: None,
                usda_num: None,
                processing: None,
                substitutions: None,
            },
        ],
        steps: None,
        notes: None,
        oven_fan: None,
        oven_temp: None,
        source_url: None,
        source_authors: None,
        source_book: None,
    };
    let deserialized_recipe: OpenRecipe = OpenRecipe::parse(&recipe_text).unwrap();
    assert_eq!(deserialized_recipe, recipe);
    let dumped_recipe = deserialized_recipe.dump().unwrap();
    println!("{}", dumped_recipe);
    let deserialized_recipe = OpenRecipe::parse(&dumped_recipe).unwrap();
    assert_eq!(deserialized_recipe, recipe)
}

#[test]
fn test_deserialize_with_notes() {
    let recipe_text = r#"
        recipe_name: Giving an Apple to a Friend
        ingredients:
            - apple:
                amounts:
                    - amount: 1
                      unit: each
                notes:
                    - Use whole apples
                    - Pears may be substituted, but produce a different flavor and mouthfeel
        steps:
            - step: Give an apple to a friend.
              notes:
                - You can also give an apple to an enemy.
        notes:
            - This is a friendly recipe; giving, rather than throwing, is recommended.
        "#;
    let recipe = OpenRecipe {
        recipe_uuid: None,
        recipe_name: "Giving an Apple to a Friend".to_string(),
        yields: None,
        ingredients: vec![Ingredient {
            name: "apple".to_string(),
            amounts: vec![IngredientAmount {
                amount: 1.0,
                unit: "each".to_string(),
            }],
            notes: Some(vec![
                "Use whole apples".to_string(),
                "Pears may be substituted, but produce a different flavor and mouthfeel"
                    .to_string(),
            ]),
            usda_num: None,
            processing: None,
            substitutions: None,
        }],
        steps: Some(vec![OpenRecipeStep {
            step: "Give an apple to a friend.".to_string(),
            notes: Some(vec!["You can also give an apple to an enemy.".to_string()]),
            haccp: None,
        }]),
        notes: Some(vec![
            "This is a friendly recipe; giving, rather than throwing, is recommended.".to_string(),
        ]),
        oven_fan: None,
        oven_temp: None,
        source_url: None,
        source_authors: None,
        source_book: None,
    };
    let deserialized_recipe: OpenRecipe = OpenRecipe::parse(&recipe_text).unwrap();
    assert_eq!(deserialized_recipe, recipe);
    let dumped_recipe = deserialized_recipe.dump().unwrap();
    println!("{}", dumped_recipe);
    let deserialized_recipe = OpenRecipe::parse(&dumped_recipe).unwrap();
    assert_eq!(deserialized_recipe, recipe)
}

#[test]
fn test_deserialize_with_haccp() {
    let recipe_text = r#"
        recipe_name: Giving an Apple to a Friend
        ingredients:
            - apple:
                usda_num: 09003
                amounts:
                    - amount: 1
                      unit: each
                processing:
                    - whole
                    - raw
                substitutions:
                    - pears:
                        usda_num: 09252
                        amounts:
                            - amount: 1
                              unit: each
                notes:
                    - Use whole apples
                    - Pears may be substituted, but produce a different flavor and mouthfeel
        steps:
            - step: Gather the apples.
              haccp:
                control_point: The apples must be clean
              notes:
                - Some people like green
                - Some people like red
            - step: Hand out the apples.
              haccp:
                critical_control_point: Wash hands with soap and warm water before distributing.
        "#;
    let recipe = OpenRecipe {
        recipe_uuid: None,
        recipe_name: "Giving an Apple to a Friend".to_string(),
        yields: None,
        ingredients: vec![Ingredient {
            name: "apple".to_string(),
            amounts: vec![IngredientAmount {
                amount: 1.0,
                unit: "each".to_string(),
            }],
            notes: Some(vec![
                "Use whole apples".to_string(),
                "Pears may be substituted, but produce a different flavor and mouthfeel"
                    .to_string(),
            ]),
            usda_num: Some("09003".to_string()),
            processing: Some(vec!["whole".to_string(), "raw".to_string()]),
            substitutions: Some(vec![Ingredient {
                name: "pears".to_string(),
                usda_num: Some("09252".to_string()),
                amounts: vec![IngredientAmount {
                    amount: 1.0,
                    unit: "each".to_string(),
                }],
                notes: None,
                processing: None,
                substitutions: None,
            }]),
        }],
        steps: Some(vec![
            OpenRecipeStep {
                step: "Gather the apples.".to_string(),
                notes: Some(vec![
                    "Some people like green".to_string(),
                    "Some people like red".to_string(),
                ]),
                haccp: Some(Haccp {
                    control_point: Some("The apples must be clean".to_string()),
                    critical_control_point: None,
                }),
            },
            OpenRecipeStep {
                step: "Hand out the apples.".to_string(),
                notes: None,
                haccp: Some(Haccp {
                    control_point: None,
                    critical_control_point: Some(
                        "Wash hands with soap and warm water before distributing.".to_string(),
                    ),
                }),
            },
        ]),
        notes: None,
        oven_fan: None,
        oven_temp: None,
        source_url: None,
        source_authors: None,
        source_book: None,
    };
    let deserialized_recipe: OpenRecipe = OpenRecipe::parse(&recipe_text).unwrap();
    assert_eq!(deserialized_recipe, recipe);
    let dumped_recipe = deserialized_recipe.dump().unwrap();
    println!("{}", dumped_recipe);
    let deserialized_recipe = OpenRecipe::parse(&dumped_recipe).unwrap();
    assert_eq!(deserialized_recipe, recipe)
}
