pub mod config;

use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use sysinfo::{Disks, Motherboard, System, Users};

use crate::config::ConfigError;

pub const LARPFETCH_VERSION: &str = "0.1.0";
// Load all build-in logo;
pub const LOGO_WINDOWS: &str = include_str!("../assets/Windows.txt");
pub const LOGO_ARCHLINUX: &str = include_str!("../assets/ArchLinux.txt");
pub const LOGO_DEBIAN: &str = include_str!("../assets/Debian.txt");
pub const LOGO_UBUNTU: &str = include_str!("../assets/Ubuntu.txt");
pub const LOGO_FEDORA: &str = include_str!("../assets/Fedora.txt");
pub const LOGO_MINT: &str = include_str!("../assets/Mint.txt");

#[derive(Clone, PartialEq, PartialOrd, Eq, Serialize, Deserialize, Debug)]
pub struct Larp {
    logo: IconType,
    options: Vec<FetchOption>,
}
impl Larp {
    pub fn new(logo: IconType, options: Vec<FetchOption>) -> Self {
        Self { logo, options }
    }
    pub fn logo(&self) -> IconType {
        self.logo
    }
    pub fn options(&self) -> &Vec<FetchOption> {
        &self.options
    }
    pub fn logo_str(&self) -> &str {
        to_icon_content(&self.logo)
    }
    pub fn print(&self) -> bool {
        let options = self.options();
        let mut logo_biggest_len = 0;
        let logo = self.logo_str();
        for line in logo.lines() {
            let len = line.len();
            if len > logo_biggest_len {
                logo_biggest_len = len;
            }
        }
        println!("");
        for (i, line) in logo.lines().enumerate() {
            let op = options.get(i);
            let backspace = logo_biggest_len - line.len();
            if op.is_some() {
                print!("{line}{}", backspaces_to_str(backspace.clone()));
            } else {
                println!("{line}{}", backspaces_to_str(backspace.clone()));
            }
            if op.is_some() {
                let option = op.unwrap();
                println!("{}", option.what.text());
            }
        }
        true
    }
}
#[derive(Clone, PartialEq, PartialOrd, Eq, Serialize, Deserialize, Debug)]
pub struct FetchOption {
    pub what: FetchType,
}
impl FetchOption {
    pub fn new(what: FetchType) -> Self {
        Self { what }
    }
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Deserialize, Serialize, Debug)]
pub enum FetchType {
    RAM,
    CPU,
    CPUName,
    OsName,
    KernelVersion,
    DiskName,
    DiskKind,
    DiskFileSystem,
    DiskTotalSpace,
    DiskMountPoint,
    Disk,
    UserID,
    UserName,
    MotherBoardName,
    MotherBoardVersion,
}
impl FetchType {
    pub fn text(&self) -> String {
        let sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        let users = Users::new_with_refreshed_list();
        match self {
            Self::RAM => {
                let ram_total = sys.total_memory() / 1073741824;
                let ram_used = sys.used_memory() / 1073741824;

                format!("Memory: {ram_used}GiB/{ram_total}GiB")
            }
            Self::OsName => {
                format!("Os: {}", System::name().unwrap())
            }
            Self::KernelVersion => {
                format!("Kernel Version: {}", System::kernel_version().unwrap())
            }
            Self::DiskName => {
                let disk = disks.first();
                if disk.is_some() {
                    format!("Disk Name: {}", disk.unwrap().name().to_str().unwrap())
                } else {
                    format!("")
                }
            }
            Self::DiskKind => {
                let disk = disks.first();
                if disk.is_some() {
                    format!("Disk Kind: {}", disk.unwrap().kind().to_string())
                } else {
                    format!("")
                }
            }
            Self::DiskFileSystem => {
                let disk = disks.first();
                if disk.is_some() {
                    format!(
                        "Disk File System: {}",
                        disk.unwrap().file_system().to_str().unwrap()
                    )
                } else {
                    format!("")
                }
            }
            Self::DiskTotalSpace => {
                let disk = disks.first();
                if disk.is_some() {
                    let size = disk.unwrap().total_space() / 1073741824;
                    format!("Disk total space: {}GiB", size)
                } else {
                    format!("")
                }
            }
            Self::DiskMountPoint => {
                let disk = disks.first();
                if disk.is_some() {
                    format!(
                        "Disk Mount Point: {}",
                        disk.unwrap().mount_point().to_str().unwrap()
                    )
                } else {
                    format!("")
                }
            }
            Self::Disk => {
                let disk = disks.first();
                if disk.is_some() {
                    let disk1 = disk.unwrap();
                    let total = disk1.total_space() / 1073741824;
                    let used: i64 = (total - (disk1.available_space()) / 1073741824) as i64;

                    let name = disk1.name().to_str().unwrap();
                    format!("Disk: {used}GiB/{total}GiB ({name})")
                } else {
                    format!("")
                }
            }
            Self::UserID => {
                let user = users.first();
                if user.is_some() {
                    format!("User ID: {}", user.unwrap().id().to_string())
                } else {
                    format!("")
                }
            }
            Self::UserName => {
                let user = users.first();
                if user.is_some() {
                    format!("User Name: {}", user.unwrap().name())
                } else {
                    format!("")
                }
            }
            Self::MotherBoardName => {
                if let Some(m) = Motherboard::new() {
                    format!("Motherboard Name: {}", m.name().unwrap())
                } else {
                    format!("")
                }
            }
            Self::MotherBoardVersion => {
                if let Some(m) = Motherboard::new() {
                    format!("Motherboard Version: {}", m.version().unwrap())
                } else {
                    format!("")
                }
            }
            _ => String::new(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug, Eq)]
pub enum IconType {
    Windows,
    ArchLinux,
    Debian,
    Fedora,
    Mint,
    Ubuntu,
    Unknown,
}
impl IconType {
    pub fn from_string(from: String) -> Self {
        match from.as_str() {
            "windows" | "Windows" => return IconType::Windows,
            "archlinux" | "ArchLinux" => return IconType::ArchLinux,
            "debian" | "Debian" => return IconType::Debian,
            "ubuntu" | "Ubuntu" => return IconType::Ubuntu,
            "mint" | "Mint" => return IconType::Mint,
            "fedora" | "Fedora" => return IconType::Fedora,
            _ => return IconType::Unknown,
        }
    }
}

// Later will replace IconType in Config
pub struct Icon {
    pub icon_type: IconType,
}
#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct Config {
    pub elements: Vec<FetchOption>,
    pub icon: IconType,
}
impl Config {
    pub fn new(elements: Vec<FetchOption>, icon: IconType) -> Self {
        Self { elements, icon }
    }
    pub fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::ConfigFileNotFound);
        }
        let mut file = File::open(&path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let new = serde_json::from_str(buffer.as_str()).unwrap();
        Ok(new)
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            icon: IconType::ArchLinux,
            elements: vec![
                FetchOption::new(FetchType::RAM),
                FetchOption::new(FetchType::KernelVersion),
                FetchOption::new(FetchType::OsName),
                FetchOption::new(FetchType::DiskName),
                FetchOption::new(FetchType::DiskKind),
                FetchOption::new(FetchType::DiskFileSystem),
                FetchOption::new(FetchType::DiskTotalSpace),
                FetchOption::new(FetchType::DiskMountPoint),
                FetchOption::new(FetchType::Disk),
                FetchOption::new(FetchType::UserID),
                FetchOption::new(FetchType::UserName),
                FetchOption::new(FetchType::MotherBoardName),
                FetchOption::new(FetchType::MotherBoardVersion),
            ],
        }
    }
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
    Unknown,
}
pub fn string(src: &str) -> String {
    String::from(src)
}
pub fn get_os() -> TargetOS {
    if cfg!(target_os = "windows") {
        return TargetOS::Windows;
    } else if cfg!(target_os = "linux") {
        return TargetOS::Linux;
    } else if cfg!(target_os = "macos") {
        return TargetOS::MacOS;
    } else {
        return TargetOS::Unknown;
    }
}
pub fn to_icon_content(icon: &IconType) -> &str {
    match icon {
        IconType::Windows => {
            return LOGO_WINDOWS;
        }
        IconType::ArchLinux => {
            return LOGO_ARCHLINUX;
        }
        IconType::Debian => {
            return LOGO_DEBIAN;
        }
        IconType::Fedora => {
            return LOGO_FEDORA;
        }
        IconType::Mint => {
            return LOGO_MINT;
        }
        IconType::Ubuntu => {
            return LOGO_UBUNTU;
        }
        _ => {
            return "";
        }
    }
}
pub fn backspaces_to_str(backspaces: usize) -> String {
    let mut string = String::new();
    let mut index = backspaces;
    while 0 < index {
        string.push_str(" ");
        index -= 1;
    }

    return string;
}
