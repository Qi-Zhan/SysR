pub const ENABLE_DEBUG: bool = true;

pub const ENABLE_SERIAL: bool = true;
pub const ENABLE_KBD: bool = true;
pub const ENABLE_VGA: bool = true;
pub const ENABLE_AUDIO: bool = false;
pub const ENABLE_DISK: bool = false;
pub const ENABLE_FB: bool = false;
pub const ENABLE_TIMER: bool = true;

#[macro_export]
macro_rules! add_device {
    ($flag:ident, $device:ty, $devices:ident) => {
        if $flag {
            $devices.push(Box::<$device>::default());
            info!("{} enabled", stringify!($device));
        }
    };
}
