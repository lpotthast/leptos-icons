#[cfg(feature = "SiServerless")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiServerless")]
/// *This icon requires the feature* `SiServerless` *to be enabled*.
#[component]
pub fn Serverless(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 17.57h4.248l-1.316 3.974H0zm0-7.557h6.75l-1.316 3.974H0zm0-7.557h9.252L7.936 6.431H0zm13.44 0H24v3.975H12.123zM9.62 13.987l1.317-3.974H24v3.974zM8.436 17.57H24v3.975H7.119Z" /></svg>
   }
}