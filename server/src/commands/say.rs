use super::Command;
use super::ConnType;
pub(super) struct Say {}

impl Say {
    pub(super) fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Command for Say {
    fn execute(&self, _: &ConnType, args: &[String]) -> anyhow::Result<String> {
        let resp: String = args.join(" ");
        Ok(resp)
    }

    fn help(&self) -> &'static str {
        "" //say doesnt need help
    }

    fn permission(&self) -> bool {
        false
    }
}
