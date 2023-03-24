#[cfg(feature = "SiManjaro")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiManjaro")]
/// *This icon requires the feature* `SiManjaro` *to be enabled*.
#[component]
pub fn Manjaro(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v24h6.75V6.75h8.625V0H0zm8.625 8.625V24h6.75V8.625h-6.75zM17.25 0v24H24V0h-6.75z" /></svg>
   }
}