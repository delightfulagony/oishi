use crate::receta::Receta;

pub(crate) struct PlanDia {
    pub(crate) desayuno: Vec<Receta>,
    pub(crate) almuerzo: Vec<Receta>,
    pub(crate) cena: Vec<Receta>,
}
