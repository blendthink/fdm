use std::process::ExitCode;

pub trait FdmCommand: Sized {
    fn run(self) -> ExitCode;
}
