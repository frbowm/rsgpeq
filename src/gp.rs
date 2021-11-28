use rand::Rng;

use crate::ind::{Ind, random_ind};
use crate::point::Point;
use crate::params::*;
use std::vec::Vec;
use std::sync::mpsc::Receiver;

pub(crate) fn run_gp<'a>(p: &Params, fit_data: &Vec<Point>, rx: Receiver<&str>) -> Box<Ind> {
    let mut current_gen: i32 = 0;
    let mut current_pop = init_pop(p, fit_data);
    let mut best_of_gen_index = find_best(p, &current_pop);
    println!("current_gen: {}\nbest_of_gen: {}", current_gen, current_pop[best_of_gen_index as usize]);

    while !finished(p, current_gen, &current_pop[best_of_gen_index as usize], &rx) {
        current_pop = next_gen(p, &mut current_pop, &fit_data);
        current_gen = current_gen + 1;
        best_of_gen_index = find_best(p, &current_pop);
        println!("current_gen: {}\nbest_of_gen: {}", current_gen, current_pop[best_of_gen_index as usize]);
    }

    current_pop[best_of_gen_index as usize].clone()
}

fn finished(p: &Params, current_gen: i32, best_of_gen: &Ind,  rx: &std::sync::mpsc::Receiver<&str>) -> bool {
    let user_stop: bool = rx.try_recv().is_err();
    let check_max_gens: bool = p.max_generations > 0;
    best_of_gen.fitness < 0.01 || (check_max_gens && current_gen >= p.max_generations) || !user_stop
}

fn init_pop<'a>(p: &Params, fit_data: &Vec<Point>) -> Vec<Box<Ind>>{
    let mut current_pop: Vec<Box<Ind>> = Vec::new();
    for _ in 0..p.pop_size {
        current_pop.push(random_ind(p.max_depth, &fit_data));
    }
    current_pop.to_vec()
}

fn find_best<'a>(p: &Params, current_pop: &'a Vec<Box<Ind>>) -> i32 {
    let mut best_index: i32 = 0;
    for i in 1..p.pop_size {
        if first_is_better(&current_pop[i as usize],&current_pop[best_index as usize]) {
            best_index = i;
        }
    }
    best_index
}

fn first_is_better(ind1: &Box<Ind>, ind2: &Box<Ind>) -> bool {
    ind1.fitness < ind2.fitness
}

fn next_gen<'a>(p: &Params, current_pop: &'a mut Vec<Box<Ind>>, fit_data: &Vec<Point>) -> Vec<Box<Ind>> {
    let mut rng = rand::thread_rng();
    let best_index = find_best(p, current_pop);
    current_pop.push(current_pop[best_index as usize].copy());

    while (current_pop.len() as i32) < p.pop_size * 2 {
        let op_rng: f64 = rng.gen();
        let op: i32 = get_op(p, op_rng);
        
        let new_ind: Box<Ind> = match op {
            0 => {
                let ind1 = &current_pop[tourney_pick(p, &current_pop) as usize];
                let ind2 = &current_pop[tourney_pick(p, &current_pop) as usize];
                let new_ind = ind1.crossover(ind2, p.max_depth, &fit_data);
                new_ind
            },
            1 => {
                current_pop[tourney_pick(p, &current_pop) as usize].mutate(p.max_depth, &fit_data)
            },
            _ => {
                current_pop[tourney_pick(p, &current_pop) as usize].copy()
            }
        };
        current_pop.push(new_ind);
    }
    current_pop.drain(0..p.pop_size.try_into().unwrap());
    current_pop.to_vec()
}

fn get_op(p: &Params, op_rng: f64) -> i32 {
    match op_rng {
        op_val if op_val < p.p_crossover => { 0 },
        op_val if op_val < p.p_crossover + p.p_mutate => { 1 },
        _ => { 2 }
    }
}

fn tourney_pick(p: &Params, current_pop: &Vec<Box<Ind>>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut best_index: i32 = rng.gen_range(0..current_pop.len()).try_into().unwrap();
    for _ in 1..p.tourney_size {
        let ind_index: i32 = rng.gen_range(0..current_pop.len()).try_into().unwrap();
        if first_is_better(&current_pop[ind_index as usize],&current_pop[best_index as usize]) {
            best_index = ind_index;
        }
    }
    best_index
}