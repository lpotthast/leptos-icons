#[cfg(feature = "TbDice3Filled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDice3Filled")]
/// *This icon requires the feature* `TbDice3Filled` *to be enabled*.
#[component]
pub fn Dice3Filled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-dice-3-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18.333 2c1.96 0 3.56 1.537 3.662 3.472l.005 .195v12.666c0 1.96 -1.537 3.56 -3.472 3.662l-.195 .005h-12.666a3.667 3.667 0 0 1 -3.662 -3.472l-.005 -.195v-12.666c0 -1.96 1.537 -3.56 3.472 -3.662l.195 -.005h12.666zm-2.833 12a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm-3.5 -3.5a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm-3.5 -3.5a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3z" stroke-width="0" fill="currentColor" /></svg>
   }
}