use std::ops::{Add,AddAssign};

#[derive(Default, Copy, Clone)]
pub(crate) struct InfoNutricional {
    pub(crate) calorias: u16,
    pub(crate) grasas: f32,
    pub(crate) hidratos: f32,
    pub(crate) proteinas: f32,
}

impl Add for InfoNutricional {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            calorias: self.calorias + other.calorias,
            grasas: self.grasas + other.grasas,
            hidratos: self.hidratos + other.hidratos,
            proteinas: self.proteinas + other.proteinas,
        }
    }
}

impl AddAssign for InfoNutricional {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            calorias: self.calorias + other.calorias,
            grasas: self.grasas + other.grasas,
            hidratos: self.hidratos + other.hidratos,
            proteinas: self.proteinas + other.proteinas,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_nutrientes() {
        let a = InfoNutricional {
            calorias: 1,
            grasas: 1.1,
            hidratos: 2.2,
            proteinas: 0.5,
        };
        let b = InfoNutricional {
            calorias: 1,
            grasas: 1.1,
            hidratos: 2.3,
            proteinas: 0.5,
        };
        let result = a + b;
        assert_eq!(result.calorias, 2);
        assert_eq!(result.grasas, 2.2);
        assert_eq!(result.hidratos, 4.5);
        assert_eq!(result.proteinas, 1.0);
    }

    #[test]
    fn suma_iguala_nutrientes() {
        let mut a = InfoNutricional {
            calorias: 1,
            grasas: 1.1,
            hidratos: 2.2,
            proteinas: 0.5,
        };
        let b = InfoNutricional {
            calorias: 1,
            grasas: 1.1,
            hidratos: 2.3,
            proteinas: 0.5,
        };
        a += b;
        assert_eq!(a.calorias, 2);
        assert_eq!(a.grasas, 2.2);
        assert_eq!(a.hidratos, 4.5);
        assert_eq!(a.proteinas, 1.0);
    }
}
