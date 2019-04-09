mod ingredient;
mod parser;

const RECIPE: &str = "
1/2 gousse d'ail dégermée
1/8 petit piment rouge
1/8 jaune d'oeuf
1/8 tranche de pain
1/32 L d'huile d'olive
";

fn main() {
    let scale: u32 = 2;
    let base_ingredients = parser::parse(RECIPE);
    for i in base_ingredients.iter() {
        println!("{}", i.scale(scale))
    }
}
