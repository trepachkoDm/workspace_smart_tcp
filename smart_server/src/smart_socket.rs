#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    On,
    Off,
}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    pub state: DeviceState,
    pub power: f32,
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        }
    }
}

impl SmartSocket {
    pub fn switch_on(&mut self) {
        self.state = DeviceState::On;
    }

    pub fn switch_off(&mut self) {
        self.state = DeviceState::Off;
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.clone()
    }

    pub fn get_power(&self) -> f32 {
        self.power
    }

    pub fn get_report(&self) -> String {
        format!(
            "SmartSocket state: {:?}, current power: {:?}",
            self.get_state(),
            self.get_power()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_device() {
        let _smart_socket = SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        };
    }

    #[test]
    fn test_get_power() {
        let smart_socket = SmartSocket {
            state: DeviceState::Off,
            power: 1.0,
        };

        assert_eq!(smart_socket.get_power(), 1.0)
    }

    #[test]
    fn test_get_state() {
        let smart_socket = SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        };

        assert_eq!(smart_socket.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_get_report() {
        let smart_socket = SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        };

        assert_eq!(
            smart_socket.get_report(),
            "SmartSocket state: Off, current power: 0.0".to_string()
        )
    }

    #[test]
    fn test_swith_on() {
        let mut smart_socket = SmartSocket {
            state: DeviceState::Off,
            power: 1.0,
        };

        smart_socket.switch_on();

        assert_eq!(smart_socket.get_state(), DeviceState::On)
    }

    #[test]
    fn test_switch_off() {
        let mut smart_socket = SmartSocket {
            state: DeviceState::On,
            power: 1.0,
        };

        smart_socket.switch_off();

        assert_eq!(smart_socket.get_state(), DeviceState::Off)
    }
}
