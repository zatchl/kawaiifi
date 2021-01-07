mod capability_info;
pub use capability_info::CapabilityInfo;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::Bss;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::Bss;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::Bss;
    }
}
