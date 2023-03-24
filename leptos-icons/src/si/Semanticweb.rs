#[cfg(feature = "SiSemanticweb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSemanticweb")]
/// *This icon requires the feature* `SiSemanticweb` *to be enabled*.
#[component]
pub fn Semanticweb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M21.602 0s-1.524 5.809-8.516 2.658c-.776-.35-.954-.444-.982-.469L2.074 6.301l10.043 4.896s.776-.326 2.026-.933C20.273 7.287 21.602 0 21.602 0zM1.59 8.486v10.448L10.947 24V13.242L1.59 8.486zm20.82 0l-9.357 4.756V24l9.357-5.066V8.486Z" /></svg>
   }
}