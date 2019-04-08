use num_rational::Ratio;

#[derive(Debug)]
pub struct Ingredient {
    pub label: String,
    pub quantity: Ratio<u32>,
}

impl Ingredient {
    pub fn scale(&self, mult: u32) -> Self {
        Self {
            label: self.label.clone(),
            quantity: self.quantity * mult,
        }
    }
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.label)
    }
}

#[cfg(test)]
mod tests {
    use crate::Ingredient;
    use num_rational::Ratio;

    #[test]
    fn it_scale_quantity() {
        let quantity = Ratio::new(1, 2);
        let ingredient = Ingredient {
            label: "".to_string(),
            quantity,
        };

        assert_eq!(ingredient.scale(2).quantity, Ratio::from_integer(1))
    }
}
