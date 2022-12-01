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
impl Mul<f32> for InfoNutricional {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            calorias: self.calorias*(rhs as u16/100),
            grasas: self.grasas*(rhs/100.0),
            hidratos: self.hidratos*(rhs/100.0),
            proteinas: self.proteinas*(rhs/100.0),
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

    const CALORIAS_A:  u16 = 1;
    const CALORIAS_B:  u16 = 1;
    const GRASAS_A:    f32 = 1.1;
    const GRASAS_B:    f32 = 1.1;
    const HIDRATOS_A:  f32 = 2.2;
    const HIDRATOS_B:  f32 = 2.3;
    const PROTEINAS_A: f32 = 0.5;
    const PROTEINAS_B: f32 = 0.5;

    fn setup_objects() -> (InfoNutricional, InfoNutricional) {
        let a = InfoNutricional {
            calorias: CALORIAS_A,
            grasas: GRASAS_A,
            hidratos: HIDRATOS_A,
            proteinas: PROTEINAS_A,
        };
        let b = InfoNutricional {
            calorias: CALORIAS_B,
            grasas: GRASAS_B,
            hidratos: HIDRATOS_B,
            proteinas: PROTEINAS_B,
        };
        return (a, b)
    }

    #[test]
    fn suma_nutrientes() {
        let (a, b) = setup_objects();
        let result = a + b;
        assert_eq!(result.calorias, CALORIAS_A+CALORIAS_B);
        assert_eq!(result.grasas, GRASAS_A+GRASAS_B);
        assert_eq!(result.hidratos, HIDRATOS_A+HIDRATOS_B);
        assert_eq!(result.proteinas, PROTEINAS_A+PROTEINAS_B);
    }

    #[test]
    fn suma_iguala_nutrientes() {
        let (mut a, b) = setup_objects();
        a += b;
        assert_eq!(a.calorias, CALORIAS_A+CALORIAS_B);
        assert_eq!(a.grasas, GRASAS_A+GRASAS_B);
        assert_eq!(a.hidratos, HIDRATOS_A+HIDRATOS_B);
        assert_eq!(a.proteinas, PROTEINAS_A+PROTEINAS_B);
    }

    #[test]
    fn multiplica_nutrientes() {
        let (a, _) = setup_objects();
        let result = a * 200.0;
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
        assert_eq!(result.calorias, CALORIAS_A+CALORIAS_B);
        assert_eq!(result.grasas, GRASAS_A+GRASAS_B);
        assert_eq!(result.hidratos, HIDRATOS_A+HIDRATOS_B);
        assert_eq!(result.proteinas, PROTEINAS_A+PROTEINAS_B);
    }

    #[test]
    fn sumatoria_1_elemento_nutrientes() {
        let (a, _) = setup_objects();
        let vector = vec![a];
        let result: InfoNutricional = vector.iter().sum();
        assert_eq!(result.calorias, CALORIAS_A);
        assert_eq!(result.grasas, GRASAS_A);
        assert_eq!(result.hidratos, HIDRATOS_A);
        assert_eq!(result.proteinas, PROTEINAS_A);
    }
}
