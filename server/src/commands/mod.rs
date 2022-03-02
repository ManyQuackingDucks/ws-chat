use std::collections::HashMap;
mod example;
mod say;

use crate::db::ConnType;
trait Command<'a>{
    fn execute(&self, dbconn: &ConnType, args: &[String]) -> anyhow::Result<String>;
    fn help(&self) -> &'a str;
    fn permission(&self) -> bool;
}

pub struct Commands<'a>{
    commands: HashMap<String, Box<dyn Command<'a>>>,
}

//Should be safe to send across threads since the mutex is only modified on creation
#[allow(clippy::non_send_fields_in_send_ty)] 
unsafe impl<'a> Send for Commands<'a> {}

impl<'a> Commands<'a>{
    pub fn new() -> Self{
        let mut commands: HashMap<String, Box<dyn Command<'a>>> = HashMap::new();
        commands.insert("example".to_string(), Box::new(example::Example::new()));
        commands.insert("".to_string(), Box::new(say::Say::new()));
        Self{commands}
    }
    pub fn exec_command(&self,  dbconn: &ConnType, mes: &crate::types::FromClient, perm_level: bool) -> anyhow::Result<String>{
        match self.commands.get(&mes.command){
            Some(comm) => {
                if perm_level == comm.permission() || !comm.permission() {
                    comm.execute(dbconn, &mes.args)
                } else {
                    Err(anyhow::Error::msg("Requires admin privleges"))
                }
            },
            None => Err(anyhow::Error::msg("Command not found")),
        }
    }
}