use std::fmt::{Display, Formatter};

pub enum Architecture {
    Ia32,
    X64,
    Arm,
    Arm64,
}

const TRY_FROM_NAME_IA32: &str = "x86";
const TRY_FROM_NAME_X64: &str = "x86_64";
const TRY_FROM_NAME_ARM: &str = "arm";
const TRY_FROM_NAME_ARM64: &str = "aarch64";

impl TryFrom<&str> for Architecture {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Architecture::*;
        let arch = match value {
            TRY_FROM_NAME_IA32 => Ia32,
            TRY_FROM_NAME_X64 => X64,
            TRY_FROM_NAME_ARM => Arm,
            TRY_FROM_NAME_ARM64 => Arm64,
            &_ => {
                let message = format!("{} is not supported.", value);
                return Err(message);
            }
        };
        Ok(arch)
    }
}

const DISPLAY_NAME_IA32: &str = "ia32";
const DISPLAY_NAME_X64: &str = "x64";
const DISPLAY_NAME_ARM: &str = "arm";
const DISPLAY_NAME_ARM64: &str = "arm64";

impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Architecture::*;
        let arch_name = match self {
            Ia32 => DISPLAY_NAME_IA32,
            X64 => DISPLAY_NAME_X64,
            Arm => DISPLAY_NAME_ARM,
            Arm64 => DISPLAY_NAME_ARM64,
        };
        write!(f, "{}", arch_name)
    }
}
