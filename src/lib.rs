use std::{error::Error, iter::FromIterator};
use std::fs;
use std::path::Path;
use colorful::{Color, Colorful};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use libc;

pub struct Config {
    pub all: bool,
    pub long: bool,
    pub path: String,
}

impl Config {
    pub fn new(all: bool, long: bool, path: Option<&str>) -> Config {
        let path: String = match path {
            Some(a) => a.parse().unwrap(),
            None => ".".to_string(),
        };
        Config {
            all,
            long,
            path,
        }
    }
}

#[allow(bare_trait_objects)]
pub fn run(config: &Config) -> Result<(), Box<Error>>{
    let dir = Path::new(&config.path);

    let file_icon = "  ";
    let dir_icon = "  ";

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let filename = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            if filename.starts_with(".") {
                continue;
            }

            if entry.path().is_dir() {
                println!("{} {}{}", dir_icon.color(Color::DeepSkyBlue1), filename.color(Color::DeepSkyBlue1), "/".color(Color::DeepSkyBlue1));
            } else {
                println!("{} {}", file_icon.color(Color::Yellow1), filename.color(Color::Yellow1));
            }
        }
        println!("");
    } else {
        println!("{}", format!("lrs: Specified path '{}' doesn't exist.", config.path).color(Color::Red))
    }
    Ok(())
}

#[allow(bare_trait_objects)]
pub fn run_all(config: &Config) -> Result<(), Box<Error>>{
    let dir = Path::new(&config.path);

    let file_icon = "  ";
    let dir_icon = "  ";

    if dir.is_dir() {
    println!("{} {}", dir_icon.color(Color::DeepSkyBlue1), "./".color(Color::DeepSkyBlue1));
    println!("{} {}",dir_icon.color(Color::DeepSkyBlue1), "../".color(Color::DeepSkyBlue1));
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let filename = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            if entry.path().is_dir() {
                println!("{} {}{}", dir_icon.color(Color::DeepSkyBlue1), filename.color(Color::DeepSkyBlue1), "/".color(Color::DeepSkyBlue1));
            } else {
                println!("{} {}", file_icon.color(Color::Yellow1), filename.color(Color::Yellow1));
            }
        }
        println!("");
    } else {
        println!("{}", format!("lrs: Specified path '{}' doesn't exist.", config.path).color(Color::Red))
    }
    Ok(())
}

#[allow(bare_trait_objects)]
pub fn run_list(config: &Config) -> Result<(), Box<Error>>{
    let dir = Path::new(&config.path);

    let file_icon = "  ";
    let dir_icon = "  ";

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let filename = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            if filename.starts_with(".") {
                continue;
            }

            let metadata = entry.metadata().unwrap().permissions();

            let uid = entry.metadata().unwrap().uid();
            let gid = entry.metadata().unwrap().gid();
            let username = get_unix_username(uid).unwrap().white();
            let group = get_unix_group(gid).unwrap().color(Color::SeaGreen1a);
            let permissions = show_permissions(format!("{:o}", metadata.mode()));

            if entry.path().is_dir() {
                println!("{} {} {} {} {}{}", permissions, username, group,dir_icon.color(Color::DeepSkyBlue1), filename.color(Color::DeepSkyBlue1), "/".color(Color::DeepSkyBlue1));
            } else {
                println!("{} {} {} {} {}", permissions, username, group,file_icon.color(Color::Yellow1), filename.color(Color::Yellow1));
            }
        }
        println!("");
    } else {
        println!("{}", format!("lrs: Specified path '{}' doesn't exist.", config.path).color(Color::Red))
    }
    Ok(())
}

#[allow(bare_trait_objects)]
pub fn run_all_list(config: &Config) -> Result<(), Box<Error>>{
    let dir = Path::new(&config.path);

    let file_icon = "  ";
    let dir_icon = "  ";

    let current_dir = Path::new(&config.path);
    let current_dir_perm = current_dir.metadata().unwrap().permissions();
    let current_dir_perm = show_permissions(format!("{:o}", current_dir_perm.mode()));
    let current_uid = current_dir.metadata().unwrap().uid();
    let current_gid = current_dir.metadata().unwrap().gid();
    let current_username = get_unix_username(current_uid).unwrap().white();
    let current_group = get_unix_group(current_gid).unwrap().color(Color::SeaGreen1a);


    let parent_path = format!("{}{}", config.path, "/..");
    let parent_path = &parent_path[..];
    let parrent_dir = Path::new(parent_path);
    let parrent_dir_perm = parrent_dir.metadata().unwrap().permissions();
    let parrent_dir_perm = show_permissions(format!("{:o}", parrent_dir_perm.mode()));
    let parrent_uid = parrent_dir.metadata().unwrap().uid();
    let parrent_gid = parrent_dir.metadata().unwrap().gid();
    let parrent_username = get_unix_username(parrent_uid).unwrap().white();
    let parrent_group = get_unix_group(parrent_gid).unwrap().color(Color::SeaGreen1a);

    if dir.is_dir() {
    println!("{} {} {} {} {}", current_dir_perm, current_username, current_group ,dir_icon.color(Color::DeepSkyBlue1), "./".color(Color::DeepSkyBlue1));
    println!("{} {} {} {} {}", parrent_dir_perm, parrent_username, parrent_group ,dir_icon.color(Color::DeepSkyBlue1), "../".color(Color::DeepSkyBlue1));
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let filename = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

            let metadata = entry.metadata().unwrap().permissions();

            let uid = entry.metadata().unwrap().uid();
            let gid = entry.metadata().unwrap().gid();
            let username = get_unix_username(uid).unwrap().white();
            let group = get_unix_group(gid).unwrap().color(Color::SeaGreen1a);

            let permissions = show_permissions(format!("{:o}", metadata.mode()));

            if entry.path().is_dir() {
                println!("{} {} {} {} {}{}", permissions, username, group,dir_icon.color(Color::DeepSkyBlue1), filename.color(Color::DeepSkyBlue1), "/".color(Color::DeepSkyBlue1));
            } else {
                println!("{} {} {} {} {}", permissions, username, group,file_icon.color(Color::Yellow1), filename.color(Color::Yellow1));
            }
        }
        println!("");
    } else {
        println!("{}", format!("lrs: Specified path '{}' doesn't exist.", config.path).color(Color::Red))
    }
    Ok(())
}

fn show_permissions(permissions: String) -> String {
    let permissions = &permissions[permissions.len()-3..];

    let user = rwx(&permissions[0..1]);
    let group = rwx(&permissions[1..2]);
    let other = rwx(&permissions[2..3]);
    format!("{}{}{}", user, group, other)
}

fn rwx(permissions: &str) -> String {
    match permissions.parse().unwrap() {
        0 => String::from(format!("{}{}{}", "-".light_magenta(), "-".light_magenta(), "-".light_magenta())),
        1 => String::from(format!("{}{}{}", "-".light_magenta(), "-".light_magenta(), "x".light_red())),
        2 => String::from(format!("{}{}{}", "-".light_magenta(), "w".light_yellow(), "-".light_magenta())),
        3 => String::from(format!("{}{}{}", "-".light_magenta(), "w".light_yellow(), "x".light_red())),
        4 => String::from(format!("{}{}{}", "r".light_green(), "-".light_magenta(), "-".light_magenta())),
        5 => String::from(format!("{}{}{}", "r".light_green(), "-".light_magenta(), "x".light_red())),
        6 => String::from(format!("{}{}{}", "r".light_green(), "w".light_yellow(), "-".light_magenta())),
        7 => String::from(format!("{}{}{}", "r".light_green(), "w".light_yellow(), "x".light_red())),
        _ => String::from("How?"),
    }
}

fn get_unix_username(uid: u32) -> Option<String> {

    unsafe {
        let mut result = std::ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = std::mem::zeroed();

        match libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(),
                              buf.capacity() as libc::size_t,
                              &mut result) {
           0 if !result.is_null() => {
               let ptr = passwd.pw_name as *const _;
               let username = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned();
               Some(username)
           },
           _ => None
        }
    }
}

fn get_unix_group(gid: u32) -> Option<String> {

    unsafe {
        let mut result = std::ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut group: libc::group = std::mem::zeroed();

        match libc::getgrgid_r(gid, &mut group, buf.as_mut_ptr(),
                              buf.capacity() as libc::size_t,
                              &mut result) {
           0 if !result.is_null() => {
               let ptr = group.gr_name as *const _;
               let username = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned();
               Some(username)
           },
           _ => None
        }
    }
}