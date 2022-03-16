use fnv::FnvHashMap; //Hashmap is never inserted into after creation so dos attacks dont work

mod example;
mod say;

use crate::db::ConnType;
trait Command {
    fn execute(&self, dbconn: &ConnType, args: &[String], username: String) -> anyhow::Result<crate::types::ChannelMes>;
    fn help(&self) -> &'static str;
    fn permission(&self) -> bool;
}

pub struct Commands {
    commands: FnvHashMap<&'static str, Box<dyn Command>>,
}

//Should be safe to send across threads since the mutex is only modified on creation
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Commands {}

impl Commands {
    pub fn new() -> Self {
        let mut commands: FnvHashMap<&'static str, Box<dyn Command>> = FnvHashMap::default();
        commands.insert("example", example::Example::new());
        commands.insert("say", say::Say::new());
        Self { commands }
    }
    pub fn exec_command(
        &self,
        dbconn: &ConnType,
        mes: &crate::types::FromClient,
        perm_level: bool,
        username: String,
    ) -> anyhow::Result<crate::types::ChannelMes> {
        match self.commands.get(&mes.command as &str) {
            Some(comm) if perm_level == comm.permission() || !comm.permission() => {
                comm.execute(dbconn, &mes.args, username)
            }
            Some(_) => Err(anyhow::Error::msg("Requires admin privleges")),
            None => Err(anyhow::Error::msg("Command not found")),
        }
    }
}
