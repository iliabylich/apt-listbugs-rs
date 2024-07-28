use crate::{log::log, AppError};
use std::{collections::HashSet, io::Read, os::fd::FromRawFd};

#[cfg(feature = "fake-package-list")]
pub(crate) fn get_input() -> Result<Vec<String>, AppError> {
    Ok(vec![(String::from("libxml2"))])
}

#[cfg(not(feature = "fake-package-list"))]
pub(crate) fn get_input() -> Result<Vec<String>, AppError> {
    let fd = std::env::var("APT_HOOK_INFO_FD").map_err(|_| AppError::NoAptHookInfoFdVariable)?;
    let fd = fd
        .parse::<i32>()
        .map_err(|_| AppError::InvalidAptHookInfoVariable)?;

    log!("APT_HOOK_INFO_FD: {fd}");

    let mut f = unsafe { std::fs::File::from_raw_fd(fd) };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|_| AppError::CantReadFromAptHook)?;

    let (_configurations, packages) = contents
        .split_once("\n\n")
        .ok_or(AppError::NoNewlineBetweenAptHookParts)?;

    let mut set = HashSet::new();

    for line in packages.trim().lines() {
        if line.ends_with("**") {
            // **REMOVE** or **CONFIGURE**
            continue;
        }
        let parts = line.split(" ").collect::<Vec<_>>();
        if parts.len() != 9 {
            return Err(AppError::InvalidAptHookLine {
                line: line.to_string(),
            });
        }

        let package_name = parts[0];

        set.insert(package_name.to_string());
    }

    Ok(set.into_iter().collect())
}
