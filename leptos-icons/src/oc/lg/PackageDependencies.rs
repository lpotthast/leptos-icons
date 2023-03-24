#[cfg(feature = "OcLgPackageDependencies")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgPackageDependencies")]
/// *This icon requires the feature* `OcLgPackageDependencies` *to be enabled*.
#[component]
pub fn PackageDependencies(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9.126.64a1.748 1.748 0 0 1 1.75 0l8.25 4.762c.103.06.199.128.286.206a.75.75 0 0 1 .554.96c.023.113.035.23.035.35v3.332a.75.75 0 0 1-1.5 0V7.64l-7.75 4.474V22.36a.75.75 0 0 1-1.125.65l-8.75-5.052a1.75 1.75 0 0 1-.875-1.515V6.917c0-.119.012-.236.035-.35a.749.749 0 0 1 .554-.96c.088-.078.184-.146.286-.205L9.126.639Zm.875 10.173v.001l7.75-4.474-7.625-4.402a.248.248 0 0 0-.25 0L2.251 6.34Zm-8.5-3.175v8.803c0 .09.048.172.125.216l7.625 4.402v-8.947Z" /><path d="m16.617 17.5 2.895-2.702a.75.75 0 0 0-1.024-1.096l-4.285 4a.75.75 0 0 0 0 1.096l4.285 4a.75.75 0 1 0 1.024-1.096L16.617 19h6.633a.75.75 0 0 0 0-1.5h-6.633Z" /></svg>
   }
}