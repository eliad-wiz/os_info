mod file_release;
mod lsb_release;

use log::trace;

use crate::{architecture, bitness, Info, Type};

fn get_release_info(from_files_only: bool) -> Info {
    let info = match from_files_only {
        true => None,
        false => lsb_release::get(),
    };

    info.or_else(file_release::get)
        .unwrap_or_else(|| Info::with_type(Type::Linux))
}

fn get_info(from_files_only: bool) -> Info {
    trace!("linux::current_platform is called");

    let mut info = get_release_info(from_files_only);
    info.bitness = bitness::get();
    info.architecture = architecture::get();

    trace!("Returning {:?}", info);
    info
}

pub fn current_platform() -> Info {
    get_info(false)
}

/// extract os info from files only (without invoking external commands)
pub fn get_info_safe() -> Info {
    let from_files_only = true;
    get_release_info(from_files_only)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        match version.os_type() {
            Type::AlmaLinux
            | Type::Alpaquita
            | Type::Alpine
            | Type::Amazon
            | Type::AOSC
            | Type::Arch
            | Type::Artix
            | Type::Bluefin
            | Type::CachyOS
            | Type::CentOS
            | Type::Debian
            | Type::EndeavourOS
            | Type::Fedora
            | Type::Garuda
            | Type::Gentoo
            | Type::Kali
            | Type::Linux
            | Type::Mabox
            | Type::Manjaro
            | Type::Mariner
            | Type::NixOS
            | Type::Nobara
            | Type::Uos
            | Type::OpenCloudOS
            | Type::openEuler
            | Type::openSUSE
            | Type::OracleLinux
            | Type::Pop
            | Type::Raspbian
            | Type::Redhat
            | Type::RedHatEnterprise
            | Type::RockyLinux
            | Type::Solus
            | Type::SUSE
            | Type::Ubuntu
            | Type::Ultramarine
            | Type::Void
            | Type::Mint => (),
            os_type => {
                panic!("Unexpected OS type: {}", os_type);
            }
        }
    }
}
