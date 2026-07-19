use log::{debug, error, info};
use walkdir::WalkDir;

use crate::utils::get::*;

use std::path::*;

pub fn query_hosts() -> Vec<String> {
    WalkDir::new(get_path(Paths::Hosts))
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_dir())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

pub fn query_modules_dirs() -> Vec<String> {
    WalkDir::new(get_path(Paths::Modules))
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_dir())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

pub fn query_modules_all() -> Vec<String> {
    WalkDir::new(get_path(Paths::Modules))
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

pub fn query_modules_specific(dir: &str) -> Vec<String> {
    let path = PathBuf::from(get_path(Paths::Modules)).join(dir);
    WalkDir::new(path)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()

}

pub fn query_profiles() -> Vec<String> {
    WalkDir::new(get_path(Paths::Profiles))
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}
