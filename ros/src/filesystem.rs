use rconfig::layout::{USER_APP_BASE, USER_APP_SIZE};

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

/// load shell, simple1, simple2
unsafe fn load_app(fs: &mut FileSystem) {
    let mut index = 0;
    copy_app!("shell", USER_APP_BASE + USER_APP_SIZE * index);
    fs.add_file(Finfo::new(
        "shell",
        0,
        USER_APP_BASE + USER_APP_SIZE * index,
    ));
    index += 1;
    copy_app!("simple1", USER_APP_BASE + USER_APP_SIZE * index);
    fs.add_file(Finfo::new(
        "simple1",
        0,
        USER_APP_BASE + USER_APP_SIZE * index,
    ));
    index += 1;
    copy_app!("simple2", USER_APP_BASE + USER_APP_SIZE * index);
    fs.add_file(Finfo::new(
        "simple2",
        0,
        USER_APP_BASE + USER_APP_SIZE * index,
    ));
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
            load_app(&mut fs);
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
}
