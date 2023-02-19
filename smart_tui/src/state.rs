use async_trait::async_trait;
use smart_client::SocketClient;
use std::io;

#[async_trait]
pub trait State {
    async fn update(&mut self, client: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct Main;

#[async_trait]
impl State for Main {
    async fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
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

#[async_trait]
impl State for SwithOnState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.switch_on().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}
struct SwithOffState;

#[async_trait]
impl State for SwithOffState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.switch_off().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}

struct GetStateState;

#[async_trait]
impl State for GetStateState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_state().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}
struct GetReportState;

#[async_trait]
impl State for GetReportState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_report().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(Main))
    }
}

struct Exit;

#[async_trait]
impl State for Exit {
    async fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}
