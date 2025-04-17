use sysinfo::{Pid, System};

pub struct SystemData {
    pub sys: System,
}

impl SystemData {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn cpu_usage(&self) -> f64 {
        self.sys.global_cpu_info().cpu_usage() as f64
    }

    pub fn memory(&self) -> (u64, u64) {
        (self.sys.used_memory(), self.sys.total_memory())
    }
}
