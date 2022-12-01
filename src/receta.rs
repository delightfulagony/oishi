use std::time::Duration;
use crate::info_nutricional::InfoNutricional;
use crate::ingrediente::Ingrediente;

pub(crate) struct Receta {
    pub(crate) nombre: String,
    pub(crate) tiempo: Duration,
    pub(crate) tipo: Vec<String>,
    pub(crate) info_nutricional: InfoNutricional,
    pub(crate) ingredientes: Vec<(f32, Ingrediente)>,
    pub(crate) pasos: Vec<String>,
}

impl Receta {
    fn new(nombre: String,
            tiempo: Duration,
            tipo: Vec<String>,
            ingredientes: Vec<(f32, Ingrediente)>,
            pasos: Vec<String>) -> Self {
        return Receta {
            nombre,
            tiempo,
            tipo,
            info_nutricional: ingredientes.iter()
                .map(|x| x.1.info_nutricional*x.0)
                .collect::<Vec<InfoNutricional>>()
                .iter().sum(),
            ingredientes,
            pasos,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_ingredientes() -> (Ingrediente, Ingrediente) {
        let pan = Ingrediente {
            nombre: String::from("Pan"),
            info_nutricional: InfoNutricional {
                calorias: 266,
                proteinas: 7.64,
                hidratos: 50.61,
                grasas: 3.29,
            },
        };
        let aceite = Ingrediente {
            nombre: String::from("Aceite de oliva"),
            info_nutricional: InfoNutricional {
                calorias: 884,
                proteinas: 0.0,
                hidratos: 0.0,
                grasas: 100.0,
            },
        };
        return (pan, aceite);
    }

    fn setup_receta() -> Receta {
        let (pan, aceite) = setup_ingredientes();
        return Receta::new(
            String::from("Tostadas con aceite"),
            Duration::new(60, 0),
            vec![String::from("Tradicional"),
            String::from("Vegetariana"),
            String::from("Vegana"),
            String::from("Halal"),
            String::from("Kosher")],
            // Medidas de una rebanada de pan y un aceite monodosis
            vec![(30.0, pan), (8.5, aceite)],
            vec![String::from("1. Corta una rebanada de pan"),
            String::from("2. Tuesta el pan"),
            String::from("3. Añade aceite"),]);
    }

    #[test]
    fn calcula_info_nutricional_receta() {
        let (pan, aceite) = setup_ingredientes();
        let tostada = setup_receta();
        assert_eq!(tostada.info_nutricional.calorias,
            (pan.info_nutricional.calorias*(30/100))
            +(aceite.info_nutricional.calorias*(8/100)));
        assert_eq!(tostada.info_nutricional.grasas,
            (pan.info_nutricional.grasas*(30.0/100.0))
            +(aceite.info_nutricional.grasas*(8.5/100.0)));
        assert_eq!(tostada.info_nutricional.hidratos,
            (pan.info_nutricional.hidratos*(30.0/100.0))
            +(aceite.info_nutricional.hidratos*(8.5/100.0)));
        assert_eq!(tostada.info_nutricional.proteinas,
            (pan.info_nutricional.proteinas*(30.0/100.0))
            +(aceite.info_nutricional.proteinas*(8.5/100.0)));
    }
}
