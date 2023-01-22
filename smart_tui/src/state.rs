use smart_client::SocketClient;
use std::io;

pub trait State {
    fn update(&mut self, client: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct Main;

impl State for Main {
    fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Swith on
    2) Swith off
    3) Get state
    4) Get report
    Other) Exit"
        );
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let selected = buf.trim();
        println!("Selected: {}", selected);

        match selected {
            "1" => Ok(Box::new(SwithOnState)),
            "2" => Ok(Box::new(SwithOffState)),
            "3" => Ok(Box::new(GetStateState)),
            "4" => Ok(Box::new(GetReportState)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct SwithOnState;

impl State for SwithOnState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.switch_on()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}
struct SwithOffState;

impl State for SwithOffState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.switch_off()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}

struct GetStateState;

impl State for GetStateState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_state()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}
struct GetReportState;

impl State for GetReportState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_report()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}

struct Exit;

impl State for Exit {
    fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}
