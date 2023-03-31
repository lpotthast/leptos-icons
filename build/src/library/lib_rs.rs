use std::path::PathBuf;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};
use xml::attribute::OwnedAttribute;

use crate::icon::SvgIcon;

#[derive(Debug)]
pub(crate) struct LibRs {
    pub path: PathBuf,
}

impl LibRs {
    pub async fn init(&self) -> Result<()> {
        info!(path = ?self.path, "Creating new lib.rs file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await?;
        Ok(())
    }

    /// Opens the file for appending thereby creating it if non-existent.
    pub async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(?err, "Could not open lib.rs file to append modules.");
                    err
                })?,
        ))
    }

    pub async fn write_enum(&mut self, icons: &[SvgIcon]) -> Result<()> {
        let variants = icons.iter().map(|icon| {
            let feature_name = &icon.feature.name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            quote! {
                #[cfg(feature = #feature_name)]
                #feature_ident
            }
        });

        let icon_enum = quote! {
            pub enum Icon {
                #(#variants),*
            }
        };

        let tokens_file: syn::File = syn::parse2(icon_enum).map_err(|err| {
            error!(?err, "Error parsing enum to token stream");
            err
        })?;
        let icon_enum_code = prettyplease::unparse(&tokens_file);

        let mut writer = self.append().await?;
        writer.write_all("\n".as_bytes()).await?;
        writer.write_all(icon_enum_code.as_bytes()).await?;
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }

    pub async fn write_leptos_icon_component(&mut self, icons: &[SvgIcon]) -> Result<()> {
        let match_arms = icons.iter().map(|icon| {
            let feature_name = &icon.feature.name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());

            let svg_content: TokenStream = icon
                .svg
                .content
                .parse()
                .map_err(|err| anyhow::anyhow!("Error parsing svg content into TokenStream: {err}"))
                .unwrap();

            let x_attribute = attribute_token_stream(&icon.svg.svg_attributes.x).unwrap();
            let y_attribute = attribute_token_stream(&icon.svg.svg_attributes.y).unwrap();
            let view_box_attribute =
                attribute_token_stream(&icon.svg.svg_attributes.view_box).unwrap();
            let stroke_linecap_attribute =
                attribute_token_stream(&icon.svg.svg_attributes.stroke_linecap).unwrap();
            let stroke_linejoin_attribute =
                attribute_token_stream(&icon.svg.svg_attributes.stroke_linejoin).unwrap();
            let stroke_width_attribute =
                attribute_token_stream(&icon.svg.svg_attributes.stroke_width).unwrap();
            // We are fine is stroke is not set for the svg.
            let stroke_attribute = attribute_token_stream(&icon.svg.svg_attributes.stroke).unwrap();
            // Fill should most likely always default to use the "currentColor".
            let fill_attribute = attribute_token_stream_opt(&icon.svg.svg_attributes.fill)
                .unwrap()
                .unwrap_or_else(|| quote!(fill = "currentColor"));
            let style_attribute = icon
                .svg
                .svg_attributes
                .style
                .clone()
                .map(|it| it.value)
                .unwrap_or_default();
            // role="graphics-symbol" should be used for icons.
            let role_attribute = attribute_token_stream_opt(&icon.svg.svg_attributes.role)
                .unwrap()
                .unwrap_or_else(|| quote!(role = "graphics-symbol"));

            let style_format_string = format!("{style_attribute} {{}}");

            quote! {
                #[cfg(feature = #feature_name)]
                Icon::#feature_ident => view! {cx,
                    // <#feature_ident width=width height=height class=class style=style title=title/>

                    <svg
                        class=class
                        style=format!(#style_format_string, style)
                        #x_attribute
                        #y_attribute
                        width=width
                        height=height
                        #view_box_attribute
                        #stroke_linecap_attribute
                        #stroke_linejoin_attribute
                        #stroke_width_attribute
                        #stroke_attribute
                        #fill_attribute
                        #role_attribute
                    >
                        // Title should be the first child!
                        <title>{title.unwrap_or_else(|| #feature_name.to_owned())}</title>
                        #svg_content
                    </svg>
                }.into_view(cx)
            }
        });

        let tokens = quote! {
            #[component]
            pub fn LeptosIcon(
                cx: Scope,
                /// Variant of the icon to display.
                icon: Icon,
                /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional, default = String::from("1em"))]
                width: String,
                /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
                #[prop(into, optional, default = String::from("1em"))]
                height: String,
                /// HTML class attribute.
                #[prop(into, optional)]
                class: String,
                /// HTML style attribute.
                #[prop(into, optional)]
                style: String,
                /// ARIA accessibility title.
                #[prop(into, optional_no_strip)]
                title: Option<String>,
            ) -> impl IntoView {
                match icon {
                    #(#match_arms),*
                }
            }
        };

        let tokens_file: syn::File = syn::parse2(tokens)?;
        let icon_component_code = prettyplease::unparse(&tokens_file);

        let mut writer = self.append().await?;
        writer.write_all("\n".as_bytes()).await?;
        // TODO: Once https://github.com/leptos-rs/leptos/pull/748 is merged, this write can be removed. In component generation use `::leptos::...` wherever possible.
        writer
            .write_all("use leptos::*;\n\n".as_bytes())
            .await
            .unwrap();
        writer.write_all(icon_component_code.as_bytes()).await?;
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }
}

fn attribute_token_stream_opt(attribute: &Option<OwnedAttribute>) -> Result<Option<TokenStream>> {
    if let Some(attribute) = attribute {
        let attribute_val = &attribute.value;
        let attr_ident: TokenStream = attribute
            .name
            .local_name
            .parse()
            .map_err(|_| anyhow::anyhow!("could not convert attributes to token stream"))?;
        Ok(Some(quote!(#attr_ident=#attribute_val)))
    } else {
        Ok(None)
    }
}

fn attribute_token_stream(attribute: &Option<OwnedAttribute>) -> Result<TokenStream> {
    attribute
        .iter()
        .map(|attribute| {
            let attribute_val = &attribute.value;
            let attr_ident: TokenStream = attribute
                .name
                .local_name
                .parse()
                .map_err(|_| anyhow::anyhow!("could not convert attributes to token stream"))?;
            Ok(quote!(#attr_ident=#attribute_val))
        })
        .collect::<Result<TokenStream>>()
}
