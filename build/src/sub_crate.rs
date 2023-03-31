use std::path::{PathBuf, Path};

use crate::{package::{Package, Downloaded}, library::{lib_rs::LibRs, cargo_toml::CargoToml}};

pub struct Crate {
    package: Package<Downloaded>,
    path: PathBuf,
    lib_rs: LibRs,
    cargo_toml: CargoToml,
}

impl Crate {
    pub fn new(src_path: &Path, package: Package<Downloaded>) -> Self {
        let path = src_path.join(format!("leptos_icons_{}", package.meta.short_name));
        let lib_rs = LibRs {
            path: path.join("lib.rs"),
        };
        let cargo_toml = CargoToml {
            path: path.join("Cargo.toml"),
        };

        Self { package, path, lib_rs, cargo_toml }
    }

    pub fn init(&mut self) -> Result<()> {
    }
}
