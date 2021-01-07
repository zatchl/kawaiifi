cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::{default_interface, interfaces, Interface, ScanError};
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::{default_interface, interfaces, Interface};
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::{default_interface, interfaces, Interface};
    }
}
