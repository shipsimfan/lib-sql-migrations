use crate::Command;

impl Command {
    /// Execute the requested command
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Applied(applied) => applied.execute(),
            Command::Apply(apply) => apply.execute(),
            Command::ApplyDown(apply_down) => apply_down.execute(),
            Command::ApplyUp(apply_up) => apply_up.execute(),
            Command::Available(available) => available.execute(),
            Command::Create(create) => create.execute(),
            Command::Required(required) => required.execute(),
            Command::RequiredDown(required_down) => required_down.execute(),
            Command::RequiredUp(required_up) => required_up.execute(),
        }
    }
}
