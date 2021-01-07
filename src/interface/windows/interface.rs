use crate::interface::{Interface, ScanError};
use std::collections::HashSet;

pub struct Interface;

impl Interface {
    pub fn scan(&self) -> Result<HashSet<Bss>, ScanError> {
        todo!()
    }

    pub fn scan_for_ssid(&self, _: &str) {
        todo!()
    }

    pub fn cached_scan_results(&self) -> Result<HashSet<Bss>, ScanError> {
        todo!()
    }
}
