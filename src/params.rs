#[derive(Clone)]

pub(crate) struct Params {
    pub(crate) pop_size: i32,
    pub(crate) max_depth: i32,
    pub(crate) max_generations: i32,
    pub(crate) tourney_size: i32,
    pub(crate) p_crossover: f64,
    pub(crate) p_mutate: f64,
    pub(crate) p_copy: f64
}
