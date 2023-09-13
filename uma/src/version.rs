// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use crate::uma;

const MAJOR_VERSION: i32 = 0;
const MINOR_VERSION: i32 = 1;

pub fn uma_protocol_version() -> String {
    format!("{}.{}", MAJOR_VERSION, MINOR_VERSION)
}

pub struct ParsedVersion {
    pub major: i32,
    pub minor: i32,
}

impl ParsedVersion {
    pub fn new(version: &str) -> Result<Self, uma::Error> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 2 {
            Err(uma::Error::InvalidVersion)
        } else {
            let major = parts[0]
                .parse::<i32>()
                .map_err(|_| uma::Error::InvalidVersion)?;
            let minor = parts[1]
                .parse::<i32>()
                .map_err(|_| uma::Error::InvalidVersion)?;
            Ok(Self { major, minor })
        }
    }

    pub fn string_value(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

pub fn get_supported_major_version() -> Vec<i32> {
    // NOTE: In the future, we may want to support multiple major versions in the same SDK, but for
    // now, this keeps things simple.
    vec![MAJOR_VERSION]
}

pub fn get_highest_supported_version_for_major_version(
    major_version: &i32,
) -> Option<ParsedVersion> {
    if *major_version != MAJOR_VERSION {
        None
    } else {
        ParsedVersion::new(&uma_protocol_version()).ok()
    }
}

pub fn select_highest_supported_version(
    other_vasp_supported_major_versions: &[i32],
) -> Option<String> {
    let supported_version = get_supported_major_version();
    let mut highest_version: Option<ParsedVersion> = None;
    for other_vasp_major_version in other_vasp_supported_major_versions {
        if !supported_version.contains(other_vasp_major_version) {
            continue;
        }

        match highest_version {
            Some(ref v) => {
                if *other_vasp_major_version > v.major {
                    highest_version = Some(
                        get_highest_supported_version_for_major_version(other_vasp_major_version)
                            .unwrap(),
                    );
                }
            }
            None => {
                highest_version = Some(
                    get_highest_supported_version_for_major_version(other_vasp_major_version)
                        .unwrap(),
                );
            }
        }
    }

    highest_version.map(|v| v.string_value())
}

pub fn select_lower_version(version1: &str, version2: &str) -> Result<String, uma::Error> {
    let v1 = ParsedVersion::new(version1)?;
    let v2 = ParsedVersion::new(version2)?;

    if v1.major > v2.major || (v1.major == v2.major && v1.minor > v2.minor) {
        Ok(version2.to_string())
    } else {
        Ok(version1.to_string())
    }
}

pub fn is_version_supported(version: &str) -> bool {
    let parsed_version = match ParsedVersion::new(version) {
        Ok(parsed_version) => parsed_version,
        Err(_) => return false,
    };
    get_supported_major_version().contains(&parsed_version.major)
}
