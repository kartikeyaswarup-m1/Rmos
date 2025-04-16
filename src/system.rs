use sysinfo::{
    System, RefreshKind, CpuRefreshKind, MemoryRefreshKind,
};

pub struct SystemData {
    pub sys: System,
}

impl SystemData {
    pub fn new() -> Self {
        let refresh = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::new());
        let mut sys = System::new_with_specifics(refresh);
        sys.refresh_all();
        Self { sys }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
    }

    pub fn cpu_usage(&self) -> f32 {
        self.sys.global_cpu_info().cpu_usage()
    }

    pub fn total_memory(&self) -> u64 {
        self.sys.total_memory()
    }

    pub fn used_memory(&self) -> u64 {
        self.sys.used_memory()
    }
}
