use alloc::vec::Vec;
use ram::{cte::Context, println, tm::halt};

/// dirty hack to use TaskManager in global
pub static mut TM: Option<TaskManager> = None;

pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub current: usize,
}

#[derive(PartialEq)]
pub enum TaskState {
    Running,
    Exit,
    Ready,
}

#[rustfmt::skip]
pub struct Task {
    pub name:       &'static str,
    pub state:      TaskState,
    pub context:   Context,
}

impl Task {
    pub fn new(name: &'static str, entry: u32, id: usize) -> Self {
        Self {
            name,
            state: TaskState::Ready,
            context: Context::new(entry, id),
        }
    }
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current: 0,
        }
    }

    pub fn run(&self) {
        unsafe {
            core::arch::asm!(
                "csrw mepc, {0}",
                "mret",
                // "jr {0}",
                in(reg) self.tasks[self.current].context.mepc,
            )
        }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn schedule(&mut self, context: &mut Context) {
        if self.tasks[self.current].state == TaskState::Exit {
            println!(
                "Task {}(pid {}) exited",
                self.tasks[self.current].name, self.current
            );
        }
        let mut cur = (self.current + 1) % self.tasks.len();
        while cur != self.current {
            if self.tasks[cur].state != TaskState::Exit {
                self.tasks[cur].state = TaskState::Running;
                self.tasks[cur].context.assign_to(context);
                self.current = cur;
                return;
            }
            cur = (cur + 1) % self.tasks.len();
        }
        if self.tasks[cur].state == TaskState::Ready {
            self.tasks[cur].state = TaskState::Running;
            self.tasks[cur].context.assign_to(context);
            self.current = cur;
            return;
        }
        println!("All tasks are exited");
        halt(0);
    }
}
