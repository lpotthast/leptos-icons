use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{error, info};

use crate::library::readme_md::Readme;
use crate::{
    icon::IconMetadata,
    library::{cargo_toml::CargoToml, icons_md::Icons, lib_rs::LibRs, src_dir::SrcDir},
    package::{Package, Parsed},
};

#[derive(Debug)]
pub(crate) struct Crate {
    pub package: Package<Parsed>,
    pub path: PathBuf,
    pub icons_md: Icons,
    pub readme_md: Readme,
    pub cargo_toml: CargoToml,
    pub src_dir: SrcDir,
}

impl Crate {
    pub fn new(src_path: &Path, package: Package<Parsed>) -> Self {
        let path = src_path.join(format!("leptos-icons-{}", package.meta.short_name));
        let src_dir = SrcDir {
            path: path.join("src"),
            lib_rs: LibRs {
                path: path.join("src").join("lib.rs"),
            },
        };
        let cargo_toml = CargoToml {
            path: path.join("Cargo.toml"),
        };

        let readme_md = Readme {
            path: path.join("README.md"),
        };

        let icons_md = Icons {
            path: path.join("ICONS.md"),
        };

        Self {
            package,
            path,
            icons_md,
            readme_md,
            src_dir,
            cargo_toml,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        info!("Creating crate directory");
        fs::create_dir(&self.path).await.map_err(|err| {
            error!(?err, "Could not create crate directory");
            err
        })?;
        self.readme_md.init(&self.package.meta).await?;
        self.icons_md.init().await?;
        self.src_dir.init().await?;
        self.cargo_toml.init(&self.package.meta).await?;

        Ok(())
    }

    pub async fn remove(&mut self) -> Result<()> {
        if self.path.exists() {
            info!("Removing crate.");
            tokio::fs::remove_dir_all(&self.path)
                .await
                .map_err(Into::into)
        } else {
            Ok(())
        }
    }

    pub async fn generate(&mut self) -> Result<()> {
        info!("Generating crate");
        self.src_dir.lib_rs.write_enum(&self.package.icons).await?;
        self.src_dir.lib_rs
            .write_leptos_icon_component(&self.package.icons)
            .await?;
        self.cargo_toml.append_features(&self.package.icons).await?;

        info!("Writing README.md");
        // self.readme_md.write_crate(self.package.meta).await?;

        info!("Writing ICONS.md.");
        let icon_metadata = self
            .package
            .icons
            .iter()
            .map(|icon| IconMetadata {
                name: icon.feature.name.clone(),
                categories: icon.categories.clone(),
            })
            .collect::<Vec<_>>();
        self.icons_md.write_crate_icon_table(icon_metadata).await?;

        Ok(())
    }
}
