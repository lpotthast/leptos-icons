#[cfg(feature = "CgTrending")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgTrending")]
/// *This icon requires the feature* `CgTrending` *to be enabled*.
#[component]
pub fn Trending(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M1.41421 16.4322L0 15.018L7.07107 7.94693L13.435 14.3109L17.6777 10.0682L15.9353 8.32587L22.6274 6.53271L20.8343 13.2248L19.0919 11.4825L13.435 17.1393L7.07107 10.7754L1.41421 16.4322Z" fill="currentColor" /></svg>
   }
}