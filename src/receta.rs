use time::Duration;
use crate::info_nutricional::InfoNutricional;
use crate::ingrediente::Ingrediente;

#[derive(Default)]
pub(crate) struct Receta {
    pub(crate) nombre: String,
    pub(crate) tiempo: Duration,
    pub(crate) tipo: Vec<String>,
    pub(crate) info_nutricional: InfoNutricional,
    pub(crate) ingredientes: Vec<(u16, Ingrediente)>,
    pub(crate) pasos: Vec<String>,
}
