#[cfg(feature = "SiXstate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiXstate")]
/// *This icon requires the feature* `SiXstate` *to be enabled*.
#[component]
pub fn Xstate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M15.891 0h6.023l-6.085 10.563c-1.853-3.305-1.822-7.32.062-10.563zm6.055 23.999L8.078.001H2.055l6.919 12.015L2.055 24h6.023L12 17.236 15.892 24z" /></svg>
   }
}