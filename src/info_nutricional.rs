use std::ops::{Add,AddAssign,Mul};
use std::iter::Sum;

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

/// El operador de suma `+`
///
/// Permite sumar dos objetos de InfoNutricional y obtiene el resultado de
/// sumar cada una de sus componentes individualmente.
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

/// El operador de suma con asignación `+=`
///
/// Permite sumar dos objetos de InfoNutricional y asigna el resultado de
/// sumar cada una de sus componentes individualmente al primer objeto.
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

/// El operador de multiplicación `*`
///
/// Permite multiplicar un objeto InfoNutricional por un número de gramos y
/// devuelve la información nutricional para esa cantidad concreta.
impl Mul<u16> for InfoNutricional {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        Self {
            calorias: self.calorias*(rhs/100),
            grasas: self.grasas*(rhs as f32/100.0),
            hidratos: self.hidratos*(rhs as f32/100.0),
            proteinas: self.proteinas*(rhs as f32/100.0),
        }
    }
}

/// La función sumatoria
///
/// Permite realizar una sumatoria a partir de un iterador de InfoNutricional
impl<'a> Sum<&'a Self> for InfoNutricional {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(
            Self {
                calorias:0,
                grasas:0.0,
                hidratos:0.0,
                proteinas:0.0,
            },
            |a, b| Self {
                calorias: a.calorias + b.calorias,
                grasas: a.grasas + b.grasas,
                hidratos: a.hidratos + b.hidratos,
                proteinas: a.proteinas + b.proteinas,
        })
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

    #[test]
    fn multiplica_nutrientes() {
        let (a, _) = setup_objects();
        let result = a * 200;
        assert_eq!(result.calorias, a.calorias*2);
        assert_eq!(result.grasas, a.grasas*2.0);
        assert_eq!(result.hidratos, a.hidratos*2.0);
        assert_eq!(result.proteinas, a.proteinas*2.0);
    }

    #[test]
    fn sumatoria_nutrientes() {
        let (a, b) = setup_objects();
        let vector = vec![a, b];
        let result: InfoNutricional = vector.iter().sum();
        assert_eq!(result.calorias, 2);
        assert_eq!(result.grasas, 2.2);
        assert_eq!(result.hidratos, 4.5);
        assert_eq!(result.proteinas, 1.0);
    }

    #[test]
    fn sumatoria_1_elemento_nutrientes() {
        let (a, _) = setup_objects();
        let vector = vec![a];
        let result: InfoNutricional = vector.iter().sum();
        assert_eq!(result.calorias, 1);
        assert_eq!(result.grasas, 1.1);
        assert_eq!(result.hidratos, 2.2);
        assert_eq!(result.proteinas, 0.5);
    }
}
