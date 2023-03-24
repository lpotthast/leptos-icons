#[cfg(feature = "SiHedera")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiHedera")]
/// *This icon requires the feature* `SiHedera` *to be enabled*.
#[component]
pub fn Hedera(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0a12 12 0 1 0 0 24 12 12 0 0 0 0-24Zm4.9571 17.3963h-1.5812V14.01H8.6224v3.3777H7.0498V6.6037H8.631v3.3845h6.7535V6.6037h1.5812zm-1.5812-6.2592H8.6224v1.7241h6.7535Z" /></svg>
   }
}