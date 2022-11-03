use crate::receta::Receta;

pub(crate) struct MenuDia {
    pub(crate) desayuno: Vec<Receta>,
    pub(crate) almuerzo: Vec<Receta>,
    pub(crate) cena: Vec<Receta>,
}
