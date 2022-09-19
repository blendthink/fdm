#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Channel {
    Dev,
    Beta,
    Stable,
}

impl Channel {
    pub fn name(&self) -> &'static str {
        match self {
            Channel::Dev => "dev",
            Channel::Beta => "beta",
            Channel::Stable => "stable",
        }
    }
}
