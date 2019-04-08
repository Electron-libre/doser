extern crate num_rational;

mod ingredient;
mod parser;

const RECIPE: &str = "
1/2 gousse d'ail dégermée
1/8 petit piment rouge
1/8 jaune d'oeuf
1/8 tranche de pain
1/32 L d'huile d'olive
";

use crate::ingredient::Ingredient;

fn scale_ingredients(ins: Vec<Ingredient>, mult: u32) -> Vec<Ingredient> {
    ins.iter().map(|e| e.scale(mult)).collect()
}

fn main() {
    let scale: u32 = 2;
    let base_ingredients = parser::parse(RECIPE);
    let scaled = scale_ingredients(base_ingredients, scale);
    for i in scaled.iter() {
        println!("{}", i)
    }
}
