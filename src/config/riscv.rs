#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
#[non_exhaustive]
pub struct RiscvConfig {
    pub core_interrupts: Vec<RiscvEnumItem>,
    pub exceptions: Vec<RiscvEnumItem>,
    pub priorities: Vec<RiscvEnumItem>,
    pub harts: Vec<RiscvEnumItem>,
    pub clint: Option<RiscvClintConfig>,
    pub plic: Option<RiscvPlicConfig>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
#[non_exhaustive]
pub struct RiscvEnumItem {
    pub name: String,
    pub value: usize,
    pub description: Option<String>,
}

impl RiscvEnumItem {
    pub fn description(&self) -> String {
        let description = match &self.description {
            Some(d) => d,
            None => &self.name,
        };
        format!("{} - {}", self.value, description)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
#[non_exhaustive]
pub struct RiscvClintConfig {
    pub name: String,
    pub freq: Option<usize>,
    pub async_delay: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
#[non_exhaustive]
pub struct RiscvPlicConfig {
    pub name: String,
    pub core_interrupt: Option<String>,
    pub hart_id: Option<String>,
}