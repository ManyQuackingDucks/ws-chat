use super::Command;
use super::ConnType;
pub struct Say{ }

impl Say{
    pub const fn new() -> Self{
        Self {}
    }
}

impl<'a> Command<'a> for Say{
    fn execute(&self, _: &ConnType, args: &[String]) -> anyhow::Result<String> {
        let resp: String = args.join("");
        Ok(resp)
    }

    fn help(&self) -> &'a str{
        " " //say doesnt need help
    }
    
    fn permission(&self) -> bool {
        false
    }
}