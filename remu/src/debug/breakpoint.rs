use crate::isas::isa::ISA;
use super::eval::eval;

struct Breakpoint {
    valid: bool,
    exp: String,
    original_value: u64,
}

impl Breakpoint {
    fn new(exp: String, value: u64) -> Breakpoint {
        Breakpoint {
            valid: true,
            exp,
            original_value: value,
        }
    }

    fn disable(&mut self) {
        self.valid = false;
    }
}

pub(super) struct Breakpoints {
    breakpoints: Vec<Breakpoint>,
}

impl Breakpoints {
    pub(super) fn new() -> Breakpoints {
        let breakpoint = Breakpoint {
            valid: true,
            exp: String::from("0"),
            original_value: 0,
        };
        Breakpoints {
            breakpoints: vec![breakpoint],
        }
    }

    pub(super) fn make_breakpoint(&mut self, cpu: &mut impl ISA, exp: &str) -> Option<u8> {
        match eval(cpu, exp) {
            Some(value) => {
                // check disabled breakpoint
                for (i, bp) in self.breakpoints.iter_mut().enumerate() {
                    if !bp.valid && bp.exp == exp {
                        bp.valid = true;
                        bp.original_value = value;
                        return Some(i as u8);
                    }
                }
                self.breakpoints.push(Breakpoint::new(exp.to_string(), value));
                Some((self.breakpoints.len()-1) as u8)
            }
            None => None,
        }
    }

    pub(super) fn check_breakpoint(&self, cpu: &mut impl ISA) -> bool {
        for bp in &self.breakpoints {
            if bp.valid {
                let value = eval(cpu, &bp.exp).unwrap();
                if bp.original_value != value {
                    println!("Breakpoint hit: {} = {}", bp.exp, value);
                    return true;
                }
            }
        }
        false
    }

    pub(super) fn delete_breakpoint(&mut self, index: usize) {
        if index >= self.breakpoints.len() {
            return;
        }
        self.breakpoints[index].disable();
    }

    pub(super) fn show(&self) {
        for (i, bp) in self.breakpoints.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if bp.valid {
                println!("{}: {}", i, bp.exp);
            }
        }
    }

    pub(super) fn exists(&self) -> bool {
        for bp in &self.breakpoints {
            if bp.valid {
                return true;
            }
        }
        false
    }

}