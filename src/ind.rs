use crate::node::Node::{self};
use crate::node::random_node;
use crate::point::Point;
use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::vec::Vec;
use rand::Rng;

#[derive(Clone)]
pub(crate) struct Ind {
    pub(crate) tree: Box<Node>,
    pub(crate) fitness: f64
}

impl Ind {
    pub(crate) fn crossover<'a>(&'a self, provider: &Box<Ind>, max_depth: i32, fit_data: &Vec<Point>) -> Box<Ind> {
        let p_count = provider.tree.count();
        let mut rng = rand::thread_rng();
        let p_point = rng.gen_range(0..p_count);
        let (_, p_tree) = provider.tree.get_node(p_point, (p_point, &provider.tree));
        let r_count = self.tree.count();
        let r_point = rng.gen_range(0..r_count);
        let (_, r_point_depth) = self.tree.depth_at_node(r_point, (r_point, 0));
        let p_trimmed_tree = p_tree.trim(max_depth - r_point_depth);
        let (_, new_tree) = self.tree.replace_node(r_point, &p_trimmed_tree);
        let fitness = calc_fitness(&new_tree, fit_data);

        Box::new(Ind { tree: Box::new(new_tree), fitness })
    }

    pub(crate) fn mutate<'a>(&'a self, max_depth: i32, fit_data: &Vec<Point>) -> Box<Ind> {
        let n_count = self.tree.count();
        let mut rng = rand::thread_rng();
        let n_point = rng.gen_range(0..n_count);
        let (_, n_depth) = self.tree.depth_at_node(n_point, (n_point, 0));
        let new_subtree = random_node(max_depth - n_depth);
        let (_, new_tree) = self.tree.replace_node(n_point, &new_subtree);
        let fitness = calc_fitness(&new_tree, fit_data);

        Box::new(Ind { tree: Box::new(new_tree), fitness: fitness })
    }

    pub(crate) fn copy<'a>(&'a self) -> Box<Ind> {
        Box::new(self.clone())
    }
}

fn calc_fitness(tree: &Node, fit_data: &Vec<Point>) -> f64 {
    fit_data.iter().fold(0.0, |acc, Point { x, y }| acc + (*y - tree.exec(*x)).abs())
}

pub(crate) fn random_ind<'a>(max_depth: i32, fit_data: &Vec<Point>) -> Box<Ind> {
    let rand_node  = random_node(max_depth);
    let fitness: f64 = calc_fitness(&rand_node, fit_data);
    Box::new(Ind { tree: rand_node, fitness })
}

impl Display for Ind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Ind\n\ttree: {}\n\tfitness: {}", self.tree, self.fitness)
    }
}
