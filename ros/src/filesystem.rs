use rconfig::layout::USER_APP_BASE;

const APPS: [&str; 4] = ["shell", "cat", "ls", "echo"];

macro_rules! copy_app {
    ($app: literal, $base: expr) => {
        let app = include_bytes!(concat!(
            "../../target/riscv32i-unknown-none-elf/release/",
            $app
        ));
        let app_len = app.len();
        let mut p = $base as *mut u8;
        for i in 0..app_len {
            *p = app[i];
            p = p.offset(1);
        }
    };
}

// shell, cat, ls, etc.
unsafe fn load_apps(fs: &mut FileSystem) {
    copy_app!("shell", USER_APP_BASE);
    // add shell to fs
    fs.add_file(Finfo::new("shell", 0, USER_APP_BASE));

    // copy_app!("cat", USER_APP_BASE + USER_APP_SIZE);
    // copy_app!("ls", USER_APP_BASE + USER_APP_SIZE * 2);
    // copy_app!("echo", USER_APP_BASE + USER_APP_SIZE * 3);
}

#[derive(Copy, Clone)]
pub struct Finfo {
    pub name: &'static str,
    pub size: usize,
    pub offset: usize,
}

impl Finfo {
    pub fn new(name: &'static str, size: usize, offset: usize) -> Self {
        Self { name, size, offset }
    }
}

pub struct FileSystem {
    pub files: [Finfo; 16], // 16 files at most currently
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            files: [Finfo::new("", 0, 0); 16],
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        let mut fs = Self::default();
        unsafe {
            load_apps(&mut fs);
        }
        fs
    }

    pub fn add_file(&mut self, file: Finfo) {
        for i in 0..16 {
            if self.files[i].name == "" {
                self.files[i] = file;
                return;
            }
        }
        panic!("too many files");
    }

    pub fn get_file(&self, name: &str) -> Option<&Finfo> {
        for i in 0..16 {
            if self.files[i].name == name {
                return Some(&self.files[i]);
            }
        }
        None
    }
}
