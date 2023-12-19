use crate::request::RESPData;

pub trait Command {
    fn execute();
}

pub enum CommandKey {
    Ping,
    Set,
    Get,
    Echo,
}

pub struct CommandFactory {}

impl CommandFactory {
    pub fn create_command(&self, data: RESPData) {}
}
