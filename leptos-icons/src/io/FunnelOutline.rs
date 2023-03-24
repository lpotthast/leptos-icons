#[cfg(feature = "IoFunnelOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFunnelOutline")]
/// *This icon requires the feature* `IoFunnelOutline` *to be enabled*.
#[component]
pub fn FunnelOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M35.4,87.12,204.05,283.56A16.07,16.07,0,0,1,208,294V413.32a7.93,7.93,0,0,0,5.39,7.59l80.15,26.67A7.94,7.94,0,0,0,304,440V294A16.07,16.07,0,0,1,308,283.56L476.6,87.12A14,14,0,0,0,466,64H46.05A14,14,0,0,0,35.4,87.12Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}