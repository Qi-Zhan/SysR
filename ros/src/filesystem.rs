use rconfig::layout::USER_APP_BASE;

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

/// load shell, simple
unsafe fn load_app(fs: &mut FileSystem) {
    copy_app!("shell", USER_APP_BASE);
    // add shell to fs
    fs.add_file(Finfo::new("shell", 0, USER_APP_BASE));
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
