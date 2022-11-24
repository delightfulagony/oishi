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
