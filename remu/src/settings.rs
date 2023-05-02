pub const ENABLE_DEBUG: bool = true;
pub const ENABLE_SERIAL: bool = true;
pub const ENABLE_KBD: bool = true;
pub const ENABLE_VGA: bool = true;
pub const ENABLE_AUDIO: bool = true;
pub const ENABLE_DISK: bool = true;
pub const ENABLE_FB: bool = true;
pub const ENABLE_TIMER: bool = true;

// TODO think again about this macro
#[macro_export]
macro_rules! enable_io {
    ($($io:ident),*) => {
        let mut devices: Vec<Box<dyn io::IO>> = Vec::new();
        $(
            if ENABLE_$io {
                devices.push(Box::new($io::new(io::$io_ADDR)));
            }
        )*
        devices
    }
}
