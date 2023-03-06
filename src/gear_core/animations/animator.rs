use std::{collections::HashMap};

use crate::{Animation, Skeleton};



pub struct Animator {
    // basically a state machine between animations
    state: usize, // index in the animation vec
    next_state: usize,
    current_anim_time: f32,
    animations: Vec<Animation>,
    variables: AnimatorVariables,
    transitions: Vec<(usize, AnimatorTransitionCondition, usize, bool)>,
}

impl Animator {
    
    pub fn set_bool(&mut self, var_name: &str, value: bool) {
        self.variables.set_bool(var_name, value);
        self.check_transitions();
    }

    pub fn set_float(&mut self, var_name: &str, value: f32) {
        self.variables.set_float(var_name, value);
        self.check_transitions();
    }

    pub fn set_int(&mut self, var_name: &str, value: i32) {
        self.variables.set_int(var_name, value);
        self.check_transitions();
    }

    fn check_transitions(&mut self) {
        let mut result = None;
        for (from, cond, to, immediate_switch) in self.transitions.iter() {
            if self.state == *from {
                if cond.evaluate(&self.variables) {
                    // we need to switch to animation !
                    result = Some((*to, *immediate_switch));
                }
            }
        }
        match result {
            Some((to, immediate)) => if immediate {
                self.transit_to(to);
            } else {
                self.next_state = to;
            }
            None => {/* no transitions */},
        }
    }

    fn transit_to(&mut self, to: usize) {
        self.state = to;
        self.next_state = to;
        self.current_anim_time = 0.;
        self.check_transitions();
    }

    fn evaluate(&self) -> Skeleton {
        self.animations.get(self.state)
            .expect("Unable to evaluate animator : animation out of range.")
            .evaluate(self.current_anim_time)
    }

    fn update(&mut self, delta: f32) {
        self.current_anim_time += delta;
        if self.current_anim_time >= self.animations.get(self.state).unwrap().duration() {
            self.transit_to(self.next_state); // loop back animation
        }
    }
}

struct AnimatorVariables {
    bool_vars: HashMap<String, bool>,
    int_vars: HashMap<String, i32>,
    float_vars: HashMap<String, f32>,
}

impl AnimatorVariables {
    pub fn new() -> AnimatorVariables {
        AnimatorVariables { 
            bool_vars: HashMap::new(),
            int_vars: HashMap::new(),
            float_vars: HashMap::new(),
        }
    }

    pub fn with_bool(mut self, var_name: String) -> AnimatorVariables {
        self.bool_vars.insert(var_name, false);
        self
    }

    pub fn with_float(mut self, var_name: String) -> AnimatorVariables {
        self.float_vars.insert(var_name, 0.);
        self
    }

    pub fn with_int(mut self, var_name: String) -> AnimatorVariables {
        self.int_vars.insert(var_name, 0);
        self
    }

    pub fn set_bool(&mut self, var_name: &str, value: bool) {
        match self.bool_vars.get_mut(var_name) {
            Some(previous_val) => *previous_val = value,
            None => println!("[GEAR] -> [ANIMATOR] -> Unable to set value to variable {var_name} : no such variable."),
        }
    }

    pub fn set_float(&mut self, var_name: &str, value: f32) {
        match self.float_vars.get_mut(var_name) {
            Some(previous_val) => *previous_val = value,
            None => println!("[GEAR] -> [ANIMATOR] -> Unable to set value to variable {var_name} : no such variable."),
        }
    }

    pub fn set_int(&mut self, var_name: &str, value: i32) {
        match self.int_vars.get_mut(var_name) {
            Some(previous_val) => *previous_val = value,
            None => println!("[GEAR] -> [ANIMATOR] -> Unable to set value to variable {var_name} : no such variable."),
        }
    }

    pub fn get_bool(&self, var_name: &str) -> Option<bool> {
        match self.bool_vars.get(var_name) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn get_float(&self, var_name: &str) -> Option<f32> {
        match self.float_vars.get(var_name) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn get_int(&self, var_name: &str) -> Option<i32> {
        match self.int_vars.get(var_name) {
            Some(value) => Some(*value),
            None => None,
        }
    }

}

#[derive(Clone, Copy, Debug)]
enum AnimatorComparaisons {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    GreaterOrEqual,
    Greater,
}

#[derive(Clone, Debug)]
enum AnimatorConditionVars {
    ImmediateBool(bool),
    ImmediateFloat(f32),
    ImmediateInt(i32),
    Bool(String),
    Float(String),
    Int(String),
}

struct AnimatorTransitionCondition {
    // todo : multiple conditions, and lazy eval on expressions
    conditions: (AnimatorConditionVars, AnimatorComparaisons, AnimatorConditionVars),
}

impl AnimatorTransitionCondition {
    pub fn evaluate(&self, variables: &AnimatorVariables) -> bool {
        match &self.conditions {
            (AnimatorConditionVars::Bool(v1), _, AnimatorConditionVars::Bool(v2)) => {
                self.evaluate_bool(
                    self.conditions.1,
                    variables.get_bool(&v1).expect("[GEAR ENGINE] -> [ANIMATOR] -> Unable to evaluate variable {v1} : Not declared."),
                    variables.get_bool(&v2).expect("[GEAR ENGINE] -> [ANIMATOR] -> Unable to evaluate variable {v2} : Not declared.")
                )
            }
            (AnimatorConditionVars::Bool(v), _, AnimatorConditionVars::ImmediateBool(b)) => {
                self.evaluate_bool(
                    self.conditions.1,
                    variables.get_bool(&v).expect("[GEAR ENGINE] -> [ANIMATOR] -> Unable to evaluate variable {v} : Not declared."),
                    *b
                )
            }
            (AnimatorConditionVars::ImmediateBool(b), _, AnimatorConditionVars::Bool(v)) => {
                self.evaluate_bool(
                    self.conditions.1,
                    *b,
                    variables.get_bool(&v).expect("[GEAR ENGINE] -> [ANIMATOR] -> Unable to evaluate variable {v} : Not declared.")
                )
            }
            _ => false
        }
    }

    fn evaluate_bool(&self, comparaison: AnimatorComparaisons, b1: bool, b2: bool) -> bool {
        unimplemented!();
    }

    fn evaluate_float(&self, f1: f32, f2: f32) -> bool {
        unimplemented!();
    }

    fn evaluate_int(&self, i1: i32, i2: i32) -> bool {
        unimplemented!();
    }
}