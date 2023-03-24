#[cfg(feature = "SiBeats")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBeats")]
/// *This icon requires the feature* `SiBeats` *to be enabled*.
#[component]
pub fn Beats(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M2.625 0v15h8.25a7.5 7.5 0 0 0 0-15zm17.016 11.705c-1.571 3.261-4.91 5.517-8.766 5.517h-8.25V24h11.25a7.5 7.5 0 0 0 5.766-12.295z" /></svg>
   }
}