#[cfg(feature = "SiBetfair")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBetfair")]
/// *This icon requires the feature* `SiBetfair` *to be enabled*.
#[component]
pub fn Betfair(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M20.218 3.14h-7.083v3.6H9.352l7.359 8.582L24 6.67h-3.782zM0 17.26h3.782v3.6h7.083v-3.6h3.783l-7.29-8.583z" /></svg>
   }
}