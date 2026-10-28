//! Capability-based access control

use bitflags::bitflags;

bitflags! {
    /// Process capabilities (Linux-inspired)
    pub struct Capability: u64 {
        const CAP_CHOWN = 1 << 0;
        const CAP_DAC_OVERRIDE = 1 << 1;
        const CAP_DAC_READ_SEARCH = 1 << 2;
        const CAP_FOWNER = 1 << 3;
        const CAP_FSETID = 1 << 4;
        const CAP_KILL = 1 << 5;
        const CAP_SETGID = 1 << 6;
        const CAP_SETUID = 1 << 7;
        const CAP_SETPCAP = 1 << 8;
        const CAP_LINUX_IMMUTABLE = 1 << 9;
        const CAP_NET_BIND_SERVICE = 1 << 10;
        const CAP_NET_BROADCAST = 1 << 11;
        const CAP_NET_ADMIN = 1 << 12;
        const CAP_NET_RAW = 1 << 13;
        const CAP_IPC_LOCK = 1 << 14;
        const CAP_IPC_OWNER = 1 << 15;
        const CAP_SYS_MODULE = 1 << 16;
        const CAP_SYS_RAWIO = 1 << 17;
        const CAP_SYS_CHROOT = 1 << 18;
        const CAP_SYS_PTRACE = 1 << 19;
        const CAP_SYS_PACCT = 1 << 20;
        const CAP_SYS_ADMIN = 1 << 21;
        const CAP_SYS_BOOT = 1 << 22;
        const CAP_SYS_NICE = 1 << 23;
        const CAP_SYS_RESOURCE = 1 << 24;
        const CAP_SYS_TIME = 1 << 25;
        const CAP_SYS_TTY_CONFIG = 1 << 26;
        const CAP_MKNOD = 1 << 27;
        const CAP_LEASE = 1 << 28;
        const CAP_AUDIT_WRITE = 1 << 29;
        const CAP_AUDIT_CONTROL = 1 << 30;
        const CAP_MAC_OVERRIDE = 1 << 31;
        const CAP_MAC_ADMIN = 1 << 32;
        const CAP_SYSLOG = 1 << 33;
        const CAP_WAKE_ALARM = 1 << 34;
        const CAP_BLOCK_SUSPEND = 1 << 35;
        const CAP_AUDIT_READ = 1 << 36;
        const CAP_PERFMON = 1 << 37;
        const CAP_BPF = 1 << 38;
        const CAP_CHECKPOINT_RESTORE = 1 << 39;
    }
}

/// Capability set for a process
#[derive(Debug, Clone, Copy)]
pub struct Capabilities {
    /// Effective capabilities (currently active)
    pub effective: Capability,
    /// Permitted capabilities (can be made effective)
    pub permitted: Capability,
    /// Inheritable capabilities (passed to children)
    pub inheritable: Capability,
}

impl Capabilities {
    /// Create empty capability set
    pub const fn empty() -> Self {
        Self {
            effective: Capability::empty(),
            permitted: Capability::empty(),
            inheritable: Capability::empty(),
        }
    }
    
    /// Create full capability set (root)
    pub const fn all() -> Self {
        Self {
            effective: Capability::all(),
            permitted: Capability::all(),
            inheritable: Capability::all(),
        }
    }
    
    /// Check if a capability is present in effective set
    pub fn has(&self, cap: Capability) -> bool {
        self.effective.contains(cap)
    }
    
    /// Add a capability to permitted set
    pub fn permit(&mut self, cap: Capability) {
        self.permitted.insert(cap);
    }
    
    /// Make a permitted capability effective
    pub fn enable(&mut self, cap: Capability) -> bool {
        if self.permitted.contains(cap) {
            self.effective.insert(cap);
            true
        } else {
            false
        }
    }
    
    /// Disable an effective capability
    pub fn disable(&mut self, cap: Capability) {
        self.effective.remove(cap);
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::empty()
    }
}
