use super::Command;
use super::ConnType;
pub struct Example{ }

impl Example{
    pub const fn new() -> Self{
        Self {}
    }
}

impl<'a> Command<'a> for Example{
    fn execute(&self, _: &ConnType, _: &[String]) -> anyhow::Result<String> {
        Ok("An example function".to_string())
    }

    fn help(&self) -> &'a str{
        "example"
    }

    fn permission(&self) -> bool {
        true
    }
}