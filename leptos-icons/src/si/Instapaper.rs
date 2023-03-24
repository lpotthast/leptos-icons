#[cfg(feature = "SiInstapaper")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiInstapaper")]
/// *This icon requires the feature* `SiInstapaper` *to be enabled*.
#[component]
pub fn Instapaper(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M14.766 20.259c0 1.819.271 2.089 2.934 2.292V24H6.301v-1.449c2.666-.203 2.934-.473 2.934-2.292V3.708c0-1.784-.27-2.089-2.934-2.292V0h11.398v1.416c-2.662.203-2.934.506-2.934 2.292v16.551z" /></svg>
   }
}