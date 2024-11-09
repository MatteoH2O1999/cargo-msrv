use crate::rust::Toolchain;

mod rustup_toolchain_check;
#[cfg(test)]
mod testing;

use crate::{Outcome, TResult};
pub use rustup_toolchain_check::{RunCommand, RustupToolchainCheck};

#[cfg(test)]
pub use testing::TestRunner;

pub trait Check {
    fn before(&self, _toolchain: &Toolchain) -> TResult<()> {
        Ok(())
    }

    fn check(&self, toolchain: &Toolchain) -> TResult<Outcome>;

    fn after(&self, _toolchain: &Toolchain) -> TResult<()> {
        Ok(())
    }
}
