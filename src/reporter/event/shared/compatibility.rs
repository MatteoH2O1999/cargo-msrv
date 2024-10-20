use crate::rust::Toolchain;

/// Reports whether a crate is compatible with a certain toolchain, or not.
/// If it's not compatible, it may specify a reason why it is not compatible.

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Compatibility {
    toolchain: Toolchain,
    is_compatible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl Compatibility {
    pub fn compatible(toolchain: impl Into<Toolchain>) -> Self {
        Self {
            toolchain: toolchain.into(),
            is_compatible: true,
            error: None,
        }
    }

    pub fn incompatible(toolchain: impl Into<Toolchain>, error: Option<String>) -> Self {
        Self {
            toolchain: toolchain.into(),
            is_compatible: false,
            error,
        }
    }

    pub fn toolchain(&self) -> &Toolchain {
        &self.toolchain
    }

    pub fn is_compatible(&self) -> bool {
        self.is_compatible
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
