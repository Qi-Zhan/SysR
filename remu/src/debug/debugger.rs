use std::io::Write;

use super::breakpoint::Breakpoints;
use super::eval::eval;
use crate::color::{ERROR, grey};
use crate::info;
use crate::isas::ISA;
use crate::error::RError;

#[derive(Debug, PartialEq)]
enum DebuggerState {
    Running,
    Paused,
    Exit,
    Init,
}

#[derive(Debug, PartialEq)]
enum DebuggerCommand {
    Run,
    Continue,
    Step(u64),
    Print(String),
    Breakpoint(String),
    Delete(u8), // delete breakpoint
    Blank,      // blank line
    Show(String),
    Help,
    Quit,
    Clear,
}

impl DebuggerCommand {
    fn parse(input: &str) -> Option<DebuggerCommand> {
        let mut tokens = input.split_whitespace();
        let command = tokens.next();
        match command {
            Some("clear") | Some("cls") => Some(DebuggerCommand::Clear),
            Some("c") | Some("continue") => Some(DebuggerCommand::Continue),
            Some("s") | Some("step") => {
                let count = tokens.next();
                match count {
                    Some(count) => {
                        let count = count.parse::<u64>();
                        match count {
                            Ok(count) => Some(DebuggerCommand::Step(count)),
                            Err(_) => None,
                        }
                    }
                    None => Some(DebuggerCommand::Step(1)),
                }
            }
            Some("p") | Some("print") => {
                let reg = tokens.next();
                reg.map(|reg| DebuggerCommand::Print(reg.to_string()))
            }
            Some("b") | Some("breakpoint") => {
                let addr = tokens.next();
                addr.map(|addr| DebuggerCommand::Breakpoint(addr.to_string()))
            }
            Some("h") | Some("help") => Some(DebuggerCommand::Help),
            Some("q") | Some("quit") => Some(DebuggerCommand::Quit),
            Some("r") | Some("run") => Some(DebuggerCommand::Run),
            Some("d") | Some("delete") => {
                let number = tokens.next();
                match number {
                    Some(number) => {
                        let count = number.parse::<u8>();
                        match count {
                            Ok(count) => Some(DebuggerCommand::Delete(count)),
                            Err(_) => None,
                        }
                    }
                    None => Some(DebuggerCommand::Step(1)),
                }
            }
            Some("show") | Some("layout") => {
                let layout = tokens.next();
                layout.map(|layout| DebuggerCommand::Show(layout.to_string()))
            }
            None => Some(DebuggerCommand::Blank),
            _ => None,
        }
    }
}

pub struct Debugger {
    state: DebuggerState,
    bps: Breakpoints,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Debugger {
    pub fn new() -> Debugger {
        info!("Debugger initialized");
        Debugger {
            state: DebuggerState::Init,
            bps: Breakpoints::new(),
        }
    }

    fn continue_(&mut self, cpu: &mut impl ISA) {
        self.state = DebuggerState::Running;
        while self.state == DebuggerState::Running {
            match self.step(cpu, 1) {
                Ok(_) => self.state = DebuggerState::Running,
                Err(e) => {
                    self.state = DebuggerState::Paused;
                    match e {
                        RError::Ebreak(_) => {
                            println!("{}", e);
                            self.state = DebuggerState::Exit;
                            return;
                        }
                        _ => println!("continue: {}", e),
                    }
                }
            }
            self.check_breakpoint(cpu);
        }
        
    }

    fn step(&mut self, cpu: &mut impl ISA, count: u64) -> Result<(), RError> {
        for _ in 0..count {
            let step_result = cpu.step();
            match step_result {
                Ok(_) => (),
                _ => return step_result,
            }
        }
        self.state = DebuggerState::Paused;
        Ok(())
    }

    fn print(&self, cpu: &mut impl ISA, exp: String) {
        match eval(cpu, &exp) {
            Some(value) => println!("{:#x}", value),
            None => println!("Invalid expression"),
        }
    }

    fn make_breakpoint(&mut self, cpu: &mut impl ISA, exp: String) {
        match self.bps.make_breakpoint(cpu, &exp) {
            Some(bp) => println!("Breakpoint {} at {}", bp, exp),
            None => {
                print!("{}", ERROR);
                println!(": Invalid expression");
            }
        }
    }

    fn check_breakpoint(&mut self, cpu: &mut impl ISA) {
        if self.bps.exists() && self.bps.check_breakpoint(cpu) {
            self.state = DebuggerState::Paused;
        }
    }

    fn delete_breakpoint(&mut self, index: u8) {
        self.bps.delete_breakpoint(index.into())
    }

    fn show_registers(&self, cpu: &impl ISA) {
        let mut i = 0;
        for (name, value) in cpu.iter() {
            print!("{:12} {:#10x}   ", name, value);
            i += 1;
            if i % 4 == 0 {
                println!();
            }
        }
        if i % 4 != 0 {
            println!();
        }
    }

    fn show_asm(&self, cpu: &mut impl ISA) {
        let pc = cpu.pc();
        let low = pc.saturating_sub(0x10);
        let high = pc + 0x20;
        let _xlen = cpu.xlen();
        fn draw_line() {
            print!(" │");
            for _ in 0..50 + 11 {
                print!("─");
            }
            println!("│");
        }
        draw_line();
        for addr in (low..=high).step_by(4) {
            if addr == pc {
                print!(">");
            } else {
                print!(" ");
            }
            let inst = cpu.disassemble(addr);
            match inst {
                Ok(inst) => {
                    print!("│{:#01$x} ", addr, 10);
                    print!("{:>50}", inst);
                }
                Err(_) => {
                    print!("│{:#01$x} ", addr, 10);
                    print!("{:>50}", "<???>");
                }
            }
            println!("│");
        }

        draw_line();
    }

    fn show_memory(&self, _cpu: &impl ISA) {
        todo!("show_memory")
    }

    pub fn debug(&mut self, cpu: &mut impl ISA) {
        loop {
            let mut input = String::new();
            print!("{} ", grey("(rdb)"));
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string();
            // parse user input
            match DebuggerCommand::parse(&input) {
                Some(DebuggerCommand::Continue) => {
                    if self.state != DebuggerState::Paused {
                        println!("The program is not paused.");
                    } else {
                        self.continue_(cpu)
                    }
                }
                Some(DebuggerCommand::Step(count)) => {
                    if self.state == DebuggerState::Exit {
                        println!("The program is exit.");
                    } else {
                        match self.step(cpu, count) {
                            Ok(_) => (),
                            Err(RError::Ebreak(_)) => {
                                self.state = DebuggerState::Exit;
                                println!("{}", RError::Ebreak(0));
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                }
                Some(DebuggerCommand::Print(expression)) => self.print(cpu, expression),
                Some(DebuggerCommand::Breakpoint(bp)) => self.make_breakpoint(cpu, bp),
                Some(DebuggerCommand::Quit) => {
                    self.state = DebuggerState::Exit;
                    return;
                }
                Some(DebuggerCommand::Run) => {
                    // TODO copy cpu state for reset
                    self.continue_(cpu);
                }
                Some(DebuggerCommand::Delete(number)) => {
                    self.delete_breakpoint(number);
                }
                Some(DebuggerCommand::Blank) => (),
                Some(DebuggerCommand::Show(layout)) => match layout.as_str() {
                    s if s.starts_with("asm") => {
                        self.show_asm(cpu);
                    }
                    s if s.starts_with("reg") => {
                        self.show_registers(cpu);
                    }
                    s if s.starts_with("mem") => {
                        self.show_memory(cpu);
                    }
                    s if s.starts_with("break") => {
                        self.bps.show();
                    }
                    _ => {
                        print!("{}", ERROR);
                        println!(": '{}' is not a valid layout argument", layout);
                    }
                },
                Some(DebuggerCommand::Clear) => print!("\x1B[2J\x1B[1;1H"),
                Some(DebuggerCommand::Help) => {
                    println!("Commands:");
                    println!("  c, continue\t\tContinue execution");
                    println!("  s, step [count]\tStep through [count] instructions");
                    println!("  show [layout]\t\tShow the current [layout]");
                    println!("  p, print [expression]\tPrint the value of [expression]");
                    println!("  b, breakpoint [expr]\tSet a breakpoint at [addr]");
                    println!("  d, delete [number]\tDelete breakpoint [number]");
                    println!("  r, run\t\tRun until breakpoint");
                    println!("  l, layout [layout]\tSet the layout to [layout]");
                    println!("  h, help\t\tShow this help message");
                    println!("  q, quit\t\tQuit the debugger");
                    println!("  clear, cls\t\tClear the screen");
                }
                None => {
                    print!("{}", ERROR);
                    println!(": '{}' is not a valid command", input);
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_command_parse() {
        use super::DebuggerCommand;
        assert_eq!(DebuggerCommand::parse("c"), Some(DebuggerCommand::Continue));
        assert_eq!(DebuggerCommand::parse("s"), Some(DebuggerCommand::Step(1)));
        assert_eq!(
            DebuggerCommand::parse("s 10"),
            Some(DebuggerCommand::Step(10))
        );
        assert_eq!(
            DebuggerCommand::parse("p x1"),
            Some(DebuggerCommand::Print("x1".to_string()))
        );
        assert_eq!(
            DebuggerCommand::parse("b 0x100"),
            Some(DebuggerCommand::Breakpoint("0x100".to_string()))
        );
        assert_eq!(DebuggerCommand::parse("q"), Some(DebuggerCommand::Quit));
        assert_eq!(DebuggerCommand::parse("r"), Some(DebuggerCommand::Run));
        assert_eq!(
            DebuggerCommand::parse("d 1"),
            Some(DebuggerCommand::Delete(1))
        );
        assert_eq!(DebuggerCommand::parse("invalid"), None);
    }
}
