use std::ops::{Add,AddAssign};

/* Default permite inicializar la estructura a los valores por defecto
 * de los tipos de dato implicados.
 *
 * Clone permite hacer una copia de la estructura por medio de clonar cada
 * elemento, también permite sobrecarga.
 *
 * Copy permite hacer una copia a nivel de bits de la estructura, esto es
 * posible porque los tipos de datos implicados son sencillos y no contienen
 * punteros ni nada que vaya a causar problemas al hacer una copia de bits.
 * Copy depende de clone, por tanto para implementar copy es necesario
 * implementar clone.
 * ["The Copy trait is a subtrait of Clone, so you always need to implement
 * Clone if you implement Copy"](https://stackoverflow.com/questions/30782836/the-trait-clone-is-is-not-implemented-when-deriving-the-trait-copy-for-enum)
 */
#[derive(Default, Clone, Copy)]
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

    fn setup_objects() -> (InfoNutricional, InfoNutricional) {
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
        return (a, b)
    }

    #[test]
    fn suma_nutrientes() {
        let (a, b) = setup_objects();
        let result = a + b;
        assert_eq!(result.calorias, 2);
        assert_eq!(result.grasas, 2.2);
        assert_eq!(result.hidratos, 4.5);
        assert_eq!(result.proteinas, 1.0);
    }

    #[test]
    fn suma_iguala_nutrientes() {
        let (mut a, b) = setup_objects();
        a += b;
        assert_eq!(a.calorias, 2);
        assert_eq!(a.grasas, 2.2);
        assert_eq!(a.hidratos, 4.5);
        assert_eq!(a.proteinas, 1.0);
    }
}
