use std::{collections::HashMap, fs, time::Duration};

pub struct Cpu {
    pub model: String,
    pub freq: f64,
}

pub struct Memory {
    pub total: u64,
    pub avail: u64,
    pub used: u64,
    pub cached: u64,
    pub buffers: u64,
}

pub struct System {
    pub uptime: Duration,
}

pub struct User {
    pub name: String,
    pub shell: String,
}

pub struct Host {
    pub arch: String,
    pub kernel: String,
}

pub struct Distro {
    pub name: String,
    pub colour: String,
}

pub struct Fs {
    pub total_size: u64,
    pub free: u64,
    pub used: u64,
}

/// Will return data from `/proc/cpuinfo`
pub fn cpu_info() -> Option<Vec<Cpu>> {
    let data = fs::read_to_string("/proc/cpuinfo").ok()?;

    let blocks = data
        .lines()
        .filter(|e| e.starts_with("model name") || e.starts_with("cpu MHz"))
        .map(|e| e.split(": ").nth(1))
        .collect::<Vec<Option<&str>>>();

    blocks
        .chunks(2)
        .map(|v| -> Option<Cpu> {
            Some(Cpu {
                model: String::from(v[0]?),
                freq: v[1]?.parse::<f64>().ok()?,
            })
        })
        .collect()
}

/// Will return data from `/proc/meminfo`
pub fn mem_info() -> Option<Memory> {
    let data = fs::read_to_string("/proc/meminfo").ok()?;

    let mem = data
        .lines()
        .map(|v| v.split_whitespace().take(2).collect::<Vec<&str>>())
        .filter(|e| !e.is_empty())
        .map(|e| -> (String, Option<u64>) {
            let mut key = e[0].to_string();
            dbg!("key: {&key}");
            key.pop();
            let val = e[1].parse::<u64>().ok();
            (key, val)
        })
        .collect::<HashMap<String, Option<u64>>>();

    Some(Memory {
        total: mem["MemTotal"]?,
        avail: mem["MemAvailable"]?,
        used: mem["Total"]? - mem["Available"]?,
        cached: mem["Cached"]?,
        buffers: mem["Buffers"]?,
    })
}

pub fn sys_info() -> Option<System> {
    todo!()
}

pub fn user_info() -> Option<User> {
    todo!()
}

pub fn host_info() -> Option<Host> {
    todo!()
}

pub fn distro_info() -> Option<Distro> {
    todo!()
}

pub fn fs_info() -> Option<Fs> {
    todo!()
}
