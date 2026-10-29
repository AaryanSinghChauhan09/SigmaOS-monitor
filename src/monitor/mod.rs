// src/monitor/mod.rs
// Display monitor management and system telemetry interface for SigmaOS.
//
// Principles: Command-Query Separation, Defensive Programming, Interface Segregation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    On,
    Standby,
    Suspend,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorConfig {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub scale_factor: u32,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            refresh_rate: 60,
            scale_factor: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monitor {
    pub id: u32,
    pub name: &'static str,
    pub config: MonitorConfig,
    pub power_state: PowerState,
    pub brightness: u8,
}

impl Monitor {
    pub fn new(id: u32, name: &'static str) -> Self {
        Self {
            id,
            name,
            config: MonitorConfig::default(),
            power_state: PowerState::On,
            brightness: 100,
        }
    }

    pub fn with_config(id: u32, name: &'static str, config: MonitorConfig) -> Self {
        Self {
            id,
            name,
            config,
            power_state: PowerState::On,
            brightness: 100,
        }
    }

    // Commands (CQS)
    pub fn set_resolution(&mut self, width: u32, height: u32) -> Result<(), &'static str> {
        if width == 0 || height == 0 || width > 7680 || height > 4320 {
            return Err("Resolution out of supported bounds (max 8K)");
        }
        self.config.width = width;
        self.config.height = height;
        Ok(())
    }

    pub fn set_refresh_rate(&mut self, rate: u32) -> Result<(), &'static str> {
        if rate < 24 || rate > 360 {
            return Err("Refresh rate must be between 24Hz and 360Hz");
        }
        self.config.refresh_rate = rate;
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness.min(100);
    }

    pub fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;
    }

    // Queries (CQS)
    pub fn is_active(&self) -> bool {
        self.power_state == PowerState::On
    }

    pub fn resolution(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    pub fn refresh_rate(&self) -> u32 {
        self.config.refresh_rate
    }
}

pub fn is_monitor_running() -> bool {
    false
}

pub fn launch_monitor() -> Result<(), ()> {
    Ok(())
}

pub fn stop_monitor() -> Result<(), ()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation_and_defaults() {
        // Arrange & Act
        let mon = Monitor::new(1, "eDP-1");

        // Assert
        assert_eq!(mon.id, 1);
        assert_eq!(mon.name, "eDP-1");
        assert_eq!(mon.resolution(), (1920, 1080));
        assert_eq!(mon.refresh_rate(), 60);
        assert!(mon.is_active());
        assert_eq!(mon.brightness, 100);
    }

    #[test]
    fn test_set_resolution_valid_and_invalid() {
        // Arrange
        let mut mon = Monitor::new(2, "HDMI-1");

        // Act: Valid resolution
        let res_ok = mon.set_resolution(2560, 1440);
        // Assert
        assert!(res_ok.is_ok());
        assert_eq!(mon.resolution(), (2560, 1440));

        // Act: Invalid resolution (zero)
        let res_err = mon.set_resolution(0, 1080);
        // Assert
        assert!(res_err.is_err());
    }

    #[test]
    fn test_set_refresh_rate() {
        // Arrange
        let mut mon = Monitor::new(3, "DP-1");

        // Act & Assert: Valid
        assert!(mon.set_refresh_rate(144).is_ok());
        assert_eq!(mon.refresh_rate(), 144);

        // Act & Assert: Below minimum
        assert!(mon.set_refresh_rate(10).is_err());
    }

    #[test]
    fn test_power_state_and_brightness() {
        // Arrange
        let mut mon = Monitor::new(4, "DP-2");

        // Act
        mon.set_power_state(PowerState::Standby);
        mon.set_brightness(80);

        // Assert
        assert!(!mon.is_active());
        assert_eq!(mon.brightness, 80);

        // Act: clamping
        mon.set_brightness(150);
        assert_eq!(mon.brightness, 100);
    }
}
