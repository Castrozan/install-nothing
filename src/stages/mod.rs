mod bios;
mod boot;
mod bootloader;
mod compilation;
mod database;
mod deno;
mod drivers;
mod filesystem;
mod initramfs;
mod kernel;
mod locale;
mod network;
mod optimization;
mod packages;
mod retro;
mod services;
mod system;
mod xorg;

use crate::cli::Stage;
use std::io;

pub use bios::BiosStage;
pub use boot::BootStage;
pub use bootloader::BootloaderStage;
pub use compilation::CompilationStage;
pub use database::DatabaseStage;
pub use deno::DenoStage;
pub use drivers::DriversStage;
pub use filesystem::FilesystemStage;
pub use initramfs::InitramfsStage;
pub use kernel::KernelStage;
pub use locale::LocaleStage;
pub use network::NetworkStage;
pub use optimization::OptimizationStage;
pub use packages::PackagesStage;
pub use retro::RetroSoftwareStage;
pub use services::ServicesStage;
pub use system::SystemStage;
pub use xorg::XorgStage;

/// Common trait for all installation stages
pub trait InstallationStage {
    fn name(&self) -> &'static str;
    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()>;
}

/// Get selected installation stages in order
pub fn selected_stages(stages: &[Stage]) -> Vec<Box<dyn InstallationStage>> {
    let mut result = Vec::new();

    for stage in stages {
        let stage_impl: Box<dyn InstallationStage> = match stage {
            Stage::Bios => Box::new(BiosStage),
            Stage::Boot => Box::new(BootStage::new()),
            Stage::Bootloader => Box::new(BootloaderStage),
            Stage::Filesystem => Box::new(FilesystemStage),
            Stage::System => Box::new(SystemStage),
            Stage::Network => Box::new(NetworkStage),
            Stage::Drivers => Box::new(DriversStage),
            Stage::Initramfs => Box::new(InitramfsStage),
            Stage::Packages => Box::new(PackagesStage),
            Stage::Kernel => Box::new(KernelStage::new()),
            Stage::Compilation => Box::new(CompilationStage::new()),
            Stage::Deno => Box::new(DenoStage::new()),
            Stage::Database => Box::new(DatabaseStage),
            Stage::Xorg => Box::new(XorgStage),
            Stage::Services => Box::new(ServicesStage),
            Stage::Retro => Box::new(RetroSoftwareStage),
            Stage::Locale => Box::new(LocaleStage),
            Stage::Optimization => Box::new(OptimizationStage),
        };
        result.push(stage_impl);
    }

    result
}
