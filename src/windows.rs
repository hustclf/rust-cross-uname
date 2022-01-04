use crate::Uname;
use std::io;

use windows::Win32::System::Diagnostics::Debug::{
    PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM, PROCESSOR_ARCHITECTURE_ARM64,
    PROCESSOR_ARCHITECTURE_INTEL,
};
use windows::Win32::System::SystemInformation::{GetNativeSystemInfo, SYSTEM_INFO};

impl Uname {
    pub fn new() -> io::Result<Uname> {
        let machine;
        unsafe {
            let mut lpsysteminfo: SYSTEM_INFO = Default::default();
            GetNativeSystemInfo(&mut lpsysteminfo);
            machine = match lpsysteminfo.Anonymous.Anonymous.wProcessorArchitecture {
                PROCESSOR_ARCHITECTURE_AMD64 | PROCESSOR_ARCHITECTURE_ARM64 => "x86_64",
                PROCESSOR_ARCHITECTURE_INTEL | PROCESSOR_ARCHITECTURE_ARM => "i686",
                _ => "unknown",
            };
        }

        let uname = Uname {
            sys_name: String::from("Windows"),
            node_name: String::from("unknown"),
            release: String::from("unknown"),
            version: String::from("unknown"),
            machine: machine.to_string(),
        };
        return Ok(uname);
    }
}
