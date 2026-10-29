//! SigmaOS Monitor
//! Display monitor management and configuration

pub mod monitor;

pub use monitor::{
    is_monitor_running, launch_monitor, stop_monitor, Monitor, MonitorConfig, PowerState,
};
