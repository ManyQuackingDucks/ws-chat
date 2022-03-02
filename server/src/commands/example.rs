use super::Command;
use super::ConnType;
pub(super) struct Example {}

impl Example {
    pub(super) fn new() -> Box<Self> {
        Box::new(Self {})//Technically doesnt allocate because Example is zero
    }
}

impl Command for Example {
    fn execute(&self, _: &ConnType, _: &[String]) -> anyhow::Result<String> {
        Ok("An example function".to_string())
    }

    fn help(&self) -> &'static str {
        "example"
    }

    fn permission(&self) -> bool {
        true
    }
}
