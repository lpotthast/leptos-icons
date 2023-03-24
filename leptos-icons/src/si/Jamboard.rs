#[cfg(feature = "SiJamboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiJamboard")]
/// *This icon requires the feature* `SiJamboard` *to be enabled*.
#[component]
pub fn Jamboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12.143 0v7.877h7.783V0zm0 8.155v7.784h7.783V8.155zm-.28.005a7.926 7.923 0 0 0-7.789 7.917A7.926 7.923 0 0 0 12 24a7.926 7.923 0 0 0 7.918-7.78h-8.056Z" /></svg>
   }
}