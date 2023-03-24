#[cfg(feature = "SiNorco")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiNorco")]
/// *This icon requires the feature* `SiNorco` *to be enabled*.
#[component]
pub fn Norco(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.055 2.707a.971.971 0 00-.688.387L0 16.78h4.049l7.27-9.597 1.927 5.74 1.42-1.875-2.578-7.676a.983.983 0 00-1.033-.666zM19.95 7.22l-7.27 9.597-1.927-5.74-1.42 1.875 2.578 7.676a.987.987 0 001.72.28L24 7.218z" /></svg>
   }
}