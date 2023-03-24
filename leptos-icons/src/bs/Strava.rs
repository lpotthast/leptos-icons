#[cfg(feature = "BsStrava")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsStrava")]
/// *This icon requires the feature* `BsStrava` *to be enabled*.
#[component]
pub fn Strava(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-strava" viewBox="0 0 16 16"><path d="M6.731 0 2 9.125h2.788L6.73 5.497l1.93 3.628h2.766L6.731 0zm4.694 9.125-1.372 2.756L8.66 9.125H6.547L10.053 16l3.484-6.875h-2.112z" /></svg>
   }
}