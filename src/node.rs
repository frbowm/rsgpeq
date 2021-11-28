extern crate rand;

use crate::node::Node::*;

use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use rand::Rng;

#[derive(Clone)]
pub(crate) enum Node {
    Constant(f64),
    Var,
    Plus(Box<Node>, Box<Node>),
    Minus(Box<Node>, Box<Node>),
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>)
}

impl Node {
    pub(crate) fn count(&self) -> i32 {
        match self {
            Constant(_) => 1,
            Var => 1,
            Plus(lhs, rhs)
            | Minus(lhs, rhs)
            | Mult(lhs, rhs)
            | Div(lhs, rhs) => 1 + lhs.count() + rhs.count(),
        }
    }

    pub(crate) fn get_node<'a>(&'a self, i: i32, pair: (i32, &'a Node)) -> (i32, &'a Node) {
        match i {
            0 => (0, &*self),
            _ => {
                match self {
                    Constant(_) => (i, pair.1),
                    Var => (i, pair.1),
                    Plus(lhs, rhs)
                    | Minus(lhs, rhs)
                    | Mult(lhs, rhs)
                    | Div(lhs, rhs) => {
                        let res = lhs.get_node(i - 1, pair);
                        rhs.get_node(res.0 - 1, res)
                    }
                }
            }
        }
    }

    pub(crate) fn replace_node<'a>(&'a self, i: i32, new_node: &'a Node) -> (i32, Node) {
        match i {
            0 => {
                (0, new_node.clone())
            }
            _ => {
                match self {
                    Constant(c) => (i, Constant(*c)),
                    Var => (i, Var),
                    Plus(lhs, rhs) => {
                        let res= lhs.replace_node(i - 1, new_node);
                        let res2 = rhs.replace_node(res.0 - 1, new_node);
                        (res2.0, Plus(Box::new(res.1), Box::new(res2.1)))
                    },
                    Minus(lhs, rhs) => {
                        let res= lhs.replace_node(i - 1, new_node);
                        let res2 = rhs.replace_node(res.0 - 1, new_node);
                        (res2.0, Minus(Box::new(res.1), Box::new(res2.1)))
                    },
                    Mult(lhs, rhs) => {
                        let res= lhs.replace_node(i - 1, new_node);
                        let res2 = rhs.replace_node(res.0 - 1, new_node);
                        (res2.0, Mult(Box::new(res.1), Box::new(res2.1)))
                    },
                    Div(lhs, rhs) => {
                        let res= lhs.replace_node(i - 1, new_node);
                        let res2 = rhs.replace_node(res.0 - 1, new_node);
                        (res2.0, Div(Box::new(res.1), Box::new(res2.1)))
                    }
                }
            }
        }
    }

    pub(crate) fn exec<'a>(&'a self, x_val: f64) -> f64 {
        match self {
            Constant(c) => *c,
            Var => x_val,
            Plus(lhs, rhs) => {
                lhs.exec(x_val) + rhs.exec(x_val)
            },
            Minus(lhs, rhs) => {
                lhs.exec(x_val) - rhs.exec(x_val)
            },
            Mult(lhs, rhs) => {
                lhs.exec(x_val) * rhs.exec(x_val)
            },
            Div(lhs, rhs) => {
                lhs.exec(x_val) / rhs.exec(x_val)
            }
        }
    }

    pub(crate) fn depth_at_node<'a>(&'a self, i: i32, (n_index, depth): (i32, i32)) -> (i32, i32) {
        match i {
            0 => (0, depth),
            _ => {
                match self {
                    Constant(_) => (i, depth),
                    Var => (i, depth),
                    Plus(lhs, rhs)
                    | Minus(lhs, rhs)
                    | Mult(lhs, rhs)
                    | Div(lhs, rhs) => {
                        let pair = lhs.depth_at_node(i - 1, (n_index, depth + 1));
                        rhs.depth_at_node(pair.0 - 1, (pair.0, depth + 1))
                    }
                }
            }
        }
    }

    pub(crate) fn trim<'a>(&'a self, curr_depth: i32) -> Box<Node> {
        match curr_depth {
            0 => {
                match self {
                    Constant(c) => Box::new(Constant(*c)),
                    Var => Box::new(Var),
                    _ => random_node(0),
                }
            },
            _ => {
                match self {
                    Constant(c) => Box::new(Constant(*c)),
                    Var => Box::new(Var),
                    Plus(lhs, rhs) => {
                        let lhs_node = lhs.trim(curr_depth - 1);
                        let rhs_node = rhs.trim(curr_depth - 1);
                        Box::new(Plus(lhs_node, rhs_node))
                    },
                    Minus(lhs, rhs) => {
                        let lhs_node = lhs.trim(curr_depth - 1);
                        let rhs_node = rhs.trim(curr_depth - 1);
                        Box::new(Minus(lhs_node, rhs_node))
                    },
                    Mult(lhs, rhs) => {
                        let lhs_node = lhs.trim(curr_depth - 1);
                        let rhs_node = rhs.trim(curr_depth - 1);
                        Box::new(Mult(lhs_node, rhs_node))
                    },
                    Div(lhs, rhs) => {
                        let lhs_node = lhs.trim(curr_depth - 1);
                        let rhs_node = rhs.trim(curr_depth - 1);
                        Box::new(Div(lhs_node, rhs_node))
                    }
                }
            }
        }
    }
}

pub(crate) fn random_node<'a>(curr_depth: i32) -> Box<Node> {
    match curr_depth {
        0 | 1 => {
            let mut rng = rand::thread_rng();
            let leaf_node = rng.gen_range(0..2);
            if leaf_node == 0 {
                return Box::new(Constant(rng.gen::<f64>()))
            } else {
                return Box::new(Var)
            }
        }
        _ => {
            let mut rng = rand::thread_rng();
            let node_index = rng.gen_range(0..6);
            match node_index {
                0 => {
                    let new_c = rng.gen::<f64>();
                    return Box::new(Constant(new_c))
                },
                1 => return Box::new(Var),
                2 => {
                    let lhs = random_node(curr_depth - 1);
                    let rhs = random_node(curr_depth - 1);
                    return Box::new(Plus(lhs, rhs))
                },
                3 => {
                    let lhs = random_node(curr_depth - 1);
                    let rhs = random_node(curr_depth - 1);
                    return Box::new(Minus(lhs, rhs))
                },
                4 => {
                    let lhs = random_node(curr_depth - 1);
                    let rhs = random_node(curr_depth - 1);
                    return Box::new(Mult(lhs, rhs))
                },
                _ => {
                    let lhs = random_node(curr_depth - 1);
                    let rhs = random_node(curr_depth - 1);
                    return Box::new(Div(lhs, rhs))
                }
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Constant(c) => write!(f, "{}", c),
            Var => write!(f, "X"),
            Plus(lhs, rhs)
            | Minus(lhs, rhs)
            | Mult(lhs, rhs)
            | Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs)
        }
    }
}
