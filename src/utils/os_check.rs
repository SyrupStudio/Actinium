// I DO NOT WANT SOME GUY ON WINDOWS 7 COMPLAING TAHT IT DOESNT WORK!!!!! >:(
// No cheks for linux becuase there isnt a way to do it
// An old debian or arch version would say Linux not very useful to js block linux
mod minimums {
    pub const WINDOWS_MIN_MAJOR: u64 = 10; // blocks 7, 8, 8.1
    pub const MACOS_MIN_MAJOR: u64 = 11;
}

pub enum OsCheckResult {
    Supported,
    Unsupported { detected: String, message: String },
}

pub fn check_os_supported() -> OsCheckResult {
    let info = os_info::get();

    match info.os_type() {
        os_info::Type::Windows => {
            if let os_info::Version::Semantic(major, _, _) = info.version() {
                if *major < minimums::WINDOWS_MIN_MAJOR {
                    return OsCheckResult::Unsupported {
                        detected: format!("Windows (build reports major version {major})"),
                        message: format!(
                            "Actinium requires Windows 10 or later.\n\nYour system appears to be running an older version of Windows that isn't supported."
                        ),
                    };
                }
            }
            OsCheckResult::Supported
        }

        os_info::Type::Macos => {
            if let os_info::Version::Semantic(major, _, _) = info.version() {
                if *major < minimums::MACOS_MIN_MAJOR {
                    return OsCheckResult::Unsupported {
                        detected: format!("macOS {}", info.version()),
                        message: format!(
                            "Actinium requires macOS 11 (Big Sur) or later.\n\nYour system is running an older version of macOS that isn't supported."
                        ),
                    };
                }
            }
            OsCheckResult::Supported
        }
        _ => OsCheckResult::Supported,
    }
}

pub fn block_and_exit(detected: &str, message: &str) -> ! {
    rfd::MessageDialog::new()
        .set_title("Unsupported Operating System")
        .set_description(&format!("{message}\n\nDetected: {detected}"))
        .set_level(rfd::MessageLevel::Error)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();

    std::process::exit(1);
}
