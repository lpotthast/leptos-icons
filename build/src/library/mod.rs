use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{package::Package, sub_crate::Crate};

pub mod cargo_toml;
pub mod icons_md;
pub mod lib_rs;
pub mod readme_md;
pub mod src_dir;

#[derive(Debug)]
pub(crate) struct Library {
    #[allow(unused)]
    path: PathBuf,
}

impl Library {
    pub fn new(root: PathBuf) -> Self {
        Self { path: root.clone() }
    }

    pub async fn generate(self, clean: bool) -> Result<()> {
        let all_crates = Arc::new(RwLock::new(Vec::<Crate>::new()));

        info!("Downloading icon packages");
        let handles = Package::all()
            .into_iter()
            .map(|package| {
                let package_type = package.ty;
                let all_crates = all_crates.clone();
                let src_path = self.path.clone();
                tokio::spawn(async move {
                    if clean {
                        package.remove().await?;
                    }

                    // Download the package.
                    let package = package.download().map_err(|err| {
                        error!(
                            ?package_type,
                            ?err,
                            "Downloading the package failed unexpectedly."
                        );
                        err
                    })?;

                    // Extract icon information from that package.
                    // Sorting the resulting Vec is necessary, as we want to reduce churn in the later generated output as much as possible.
                    info!(?package_type, "Collecting icons.");
                    let package = package.read_icons().await.map_err(|err| {
                        error!(?package_type, ?err, "Could not get icons.");
                        err
                    })?;

                    let icon_crate = Crate::new(&src_path, package);

                    {
                        let mut lock = all_crates.write().await;
                        lock.push(icon_crate);
                    }

                    Ok::<(), anyhow::Error>(())
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            if let Err(err) = handle.await.unwrap() {
                error!(?err, "Could not process package successfully.");
            };
        }

        // Should we sort the crates?
        // let all_crates = {
        //     let mut lock = all_crates.write().await;
        //     let num_crates = lock.len();
        //     info!(num_crates, "Sorting features to avoid churn.");
        //     lock.sort_by(|a, b| a.package.meta.package_name.cmp(&b.package.meta.package_name));
        //     std::mem::take(&mut *lock)
        // };

        for crate_ in all_crates.write().await.iter_mut() {
            crate_.remove().await?;
            crate_.init().await?;
            crate_.generate().await?;
        }

        // self.src_dir.lib_rs.write_enum(&all_icons).await?;
        // self.src_dir
        //     .lib_rs
        //     .write_leptos_icon_component(&all_icons)
        //     .await?;
        // self.cargo_toml.append_features(&all_icons).await?;

        // info!("Writing README.md.");
        // self.readme_md.write_usage().await?;
        // self.readme_md.write_package_table().await?;
        // self.readme_md.write_contribution().await?;

        // info!("Writing ICONS.md.");
        // let mut package_icon_metadata: Vec<(PackageType, Vec<IconMetadata>)> =
        //     Package::all().into_iter().map(|p| (p.ty, vec![])).collect();
        // for package in Package::all() {
        //     let meta = all_icons
        //         .iter()
        //         .filter(|icon| icon.source == package.ty)
        //         .map(|icon| IconMetadata {
        //             name: icon.feature.name.clone(),
        //             categories: icon.categories.clone(),
        //         })
        //         .collect::<Vec<_>>();

        //     package_icon_metadata
        //         .iter_mut()
        //         .find(|(p, _vec)| *p == package.ty)
        //         .expect("should have been initialized")
        //         .1 = meta;
        // }
        // self.icons_md
        //     .write_icon_table(package_icon_metadata)
        //     .await?;

        Ok(())
    }
}
