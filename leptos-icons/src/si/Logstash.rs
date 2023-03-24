#[cfg(feature = "SiLogstash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiLogstash")]
/// *This icon requires the feature* `SiLogstash` *to be enabled*.
#[component]
pub fn Logstash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M13.745 24h8.291v-9.164h-8.29zm-2.618 0h.437v-9.164h-9.6A9.163 9.163 0 0011.127 24m.438-9.164h-9.6V0h.873a8.727 8.727 0 018.727 8.727z" /></svg>
   }
}