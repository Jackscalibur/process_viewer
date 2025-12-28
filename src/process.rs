use std::{ffi::OsString, path::PathBuf, time::Instant};
use sysinfo::{self, ProcessesToUpdate};

pub trait ProcessSampler {
    fn refresh(&mut self);
    fn snapshot(&self) -> &[ProcessInfo];
}

pub struct SysInfoSampler {
    system: sysinfo::System,
    snapshot: Vec<ProcessInfo>,
    last_sample: Instant,
}

impl SysInfoSampler {
    pub fn new() -> Self {
        Self {
            system: sysinfo::System::new(),
            snapshot: Vec::new(),
            last_sample: Instant::now(),
        }
    }
}

impl ProcessSampler for SysInfoSampler {
    fn refresh(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        self.snapshot.clear();

        for (pid, proc_) in self.system.processes() {
            self.snapshot.push(ProcessInfo::from_sysinfo(*pid, proc_));
        }
    }

    fn snapshot(&self) -> &[ProcessInfo] {
        &self.snapshot
    }
}

pub struct ProcessInfo {
    pub pid: u32,
    pub start_time: u64,
    pub name: OsString,
    exe: Option<PathBuf>,
    cpu_percent: f32,
    memory_bytes: u64,
    parent_pid: Option<u32>,
}

impl ProcessInfo {
    /// Takes types from the sysinfo crate and converts them,
    /// takes ownership, and normalizes semantics for the UI layer to use.
    pub fn from_sysinfo(pid: sysinfo::Pid, p: &sysinfo::Process) -> Self {
        Self {
            pid: pid.as_u32(),
            start_time: p.start_time(),
            name: p.name().to_owned(),
            exe: p.exe().map(PathBuf::from),
            cpu_percent: p.cpu_usage(),
            memory_bytes: p.memory(),
            parent_pid: p.parent().map(|pp| pp.as_u32()),
        }
    }
}