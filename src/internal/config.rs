use crate::functions::*;
use crate::internal::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    partition: Partition,
    bootloader: Bootloader,
    locale: Locale,
    networking: Networking,
    users: Vec<Users>,
    rootpass: String,
    desktop: String,
    timeshift: bool,
    extra_packages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Partition {
    device: String,
    mode: String,
    efi: bool,
}

#[derive(Serialize, Deserialize)]
struct Bootloader {
    r#type: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
struct Locale {
    locale: Vec<String>,
    keymap: String,
    timezone: String,
}

#[derive(Serialize, Deserialize)]
struct Networking {
    hostname: String,
    ipv6: bool,
}

#[derive(Serialize, Deserialize)]
struct Users {
    name: String,
    password: String,
    hasroot: bool,
}

pub fn read_config(configpath: &str) {
    let data = std::fs::read_to_string(configpath);
    match &data {
        Ok(_) => {
            log(format!(
                "[ \x1b[2;1;32mOK\x1b[0m ] {}",
                format!("Read config file {}", configpath).as_str()
            ));
        }
        Err(e) => {
            crash(
                format!(
                    "{}  ERROR: {}",
                    format!("Read config file {}", configpath).as_str(),
                    e
                ),
                e.raw_os_error().unwrap(),
            );
        }
    }
    let config: std::result::Result<Config, serde_json::Error> =
        serde_json::from_str(&data.unwrap());
    match &config {
        Ok(_) => {
            log(format!(
                "[ \x1b[2;1;32mOK\x1b[0m ] {}",
                format!("Parse config file {}", configpath).as_str()
            ));
        }
        Err(e) => {
            crash(
                format!(
                    "{}  ERROR: {}",
                    format!("Parse config file {}", configpath).as_str(),
                    e
                ),
                1,
            );
        }
    }
    let config: Config = config.unwrap();

    println!("---------Setup Partitions---------");
    println!("{}", config.partition.device);
    println!("{}", config.partition.mode);
    println!("{}", config.partition.efi);
    partition::partition(
        config.partition.device.as_str(),
        config.partition.mode.as_str(),
        config.partition.efi,
    );
    base::install_base_packages();
    base::genfstab();
    println!("---------Install Bootloader---------");
    println!("{}", config.bootloader.r#type);
    println!("{}", config.bootloader.location);
    if config.bootloader.r#type == "grub-efi" {
        base::install_bootloader_efi(config.bootloader.location.as_str());
    } else if config.bootloader.r#type == "grub-legacy" {
        base::install_bootloader_legacy(config.bootloader.location.as_str());
    }
    println!("---------Set Locale---------");
    println!("{:?}", config.locale.locale);
    println!("{}", config.locale.keymap);
    println!("{}", config.locale.timezone);
    locale::set_locale(config.locale.locale.join(" "));
    locale::set_keyboard(config.locale.keymap.as_str());
    locale::set_timezone(config.locale.timezone.as_str());
    println!("---------Set Networking---------");
    println!("{}", config.networking.hostname);
    println!("{}", config.networking.ipv6);
    network::set_hostname(config.networking.hostname.as_str());
    network::create_hosts();
    if config.networking.ipv6 {
        network::enable_ipv6();
    }
    println!("---------Create Users---------");
    println!("---------");
    for i in 0..config.users.len() {
        println!("{}", config.users[i].name);
        println!("{}", config.users[i].password);
        println!("{}", config.users[i].hasroot);
        users::new_user(
            config.users[i].name.as_str(),
            config.users[i].hasroot,
            config.users[i].password.as_str(),
        );
        println!("---------");
    }
    println!("---------Set Rootpass---------");
    println!("{}", config.rootpass);
    users::root_pass(config.rootpass.as_str());
    println!("---------Install Desktop---------");
    println!("{}", config.desktop);
    desktops::choose_pkgs(config.desktop.as_str());
    println!("---------Setup Timeshift---------");
    println!("{}", config.timeshift);
    if config.timeshift {
        base::setup_timeshift();
    }
    println!("---------Install Extra packages---------");
    println!("{:?}", config.extra_packages);
    let mut extra_packages: Vec<&str> = Vec::new();
    for i in 0..config.extra_packages.len() {
        extra_packages.push(config.extra_packages[i].as_str());
    }
    install(extra_packages);
}