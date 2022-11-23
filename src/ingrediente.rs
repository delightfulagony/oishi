use crate::info_nutricional::InfoNutricional;

#[derive(Clone)]
pub(crate) struct Ingrediente {
    pub(crate) nombre: String,
    pub(crate) info_nutricional: InfoNutricional,
}
