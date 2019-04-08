use crate::Ingredient;
use num_rational::Ratio;

pub fn parse(input: &str) -> Vec<Ingredient> {
    input.lines().filter_map(&parse_line).collect()
}

fn parse_line(line: &str) -> Option<Ingredient> {
    let splits: Vec<&str> = line.splitn(2, ' ').collect();

    match splits[..] {
        [quantity, label] => Some(Ingredient {
            quantity: str_to_ratio(quantity),
            label: label.to_string(),
        }),
        _ => None,
    }
}

fn str_to_ratio(s: &str) -> Ratio<u32> {
    let numerics: Vec<u32> = s
        .splitn(2, '/')
        .map(str::parse::<u32>)
        .filter_map(Result::ok)
        .collect();

    match numerics[..] {
        [numer] => Ratio::from_integer(numer),
        [numer, denom] => Ratio::new(numer, denom),
        _ => panic!("Malformed recipe quantity: {:?}", numerics),
    }
}
