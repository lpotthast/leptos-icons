#[cfg(feature = "SiHaskell")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiHaskell")]
/// *This icon requires the feature* `SiHaskell` *to be enabled*.
#[component]
pub fn Haskell(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 3.535L5.647 12 0 20.465h4.235L9.883 12 4.235 3.535zm5.647 0L11.294 12l-5.647 8.465h4.235l3.53-5.29 3.53 5.29h4.234L9.883 3.535zm8.941 4.938l1.883 2.822H24V8.473zm2.824 4.232l1.882 2.822H24v-2.822z" /></svg>
   }
}