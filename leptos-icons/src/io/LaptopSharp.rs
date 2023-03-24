#[cfg(feature = "IoLaptopSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLaptopSharp")]
/// *This icon requires the feature* `IoLaptopSharp` *to be enabled*.
#[component]
pub fn LaptopSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M477.29,400A27.75,27.75,0,0,0,480,388V108a28,28,0,0,0-28-28H60a28,28,0,0,0-28,28V388a27.75,27.75,0,0,0,2.71,12H0v32H512V400Z" /></svg>
   }
}