#[cfg(feature = "SiBose")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBose")]
/// *This icon requires the feature* `SiBose` *to be enabled*.
#[component]
pub fn Bose(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M14.051 10.555a.686.686 0 00-.588.34c-.11.194-.426.742-.54.936a.27.27 0 00.236.409h1.873l-.336.582a.271.271 0 01-.24.142h-.29c-.097 0-.137-.105-.103-.168.035-.063.112-.194.112-.194h-1.698l-.246.426c-.115.2.038.416.233.416h3.173a.69.69 0 00.598-.349c.121-.21.405-.706.528-.916a.27.27 0 00-.228-.42h-1.873l.346-.6a.274.274 0 01.231-.125h.292c.096 0 .136.104.106.159a14.05 14.05 0 01-.118.204h1.696l.255-.44a.273.273 0 00-.24-.402h-3.179m-2.94.65a1307.162 1307.217 0 00-.936 1.622.275.275 0 01-.236.137h-.295c-.095 0-.138-.104-.102-.168l.94-1.629a.275.275 0 01.236-.133h.295a.113.113 0 01.098.171m1.597-.65h-3.17a.695.695 0 00-.593.337l-1.238 2.145c-.11.19.04.407.236.407h3.176c.256 0 .48-.145.593-.338l1.236-2.143a.272.272 0 00-.24-.408m-5.723.65l-.243.42a.266.266 0 01-.233.134h-.9l.419-.725h.858c.089 0 .14.096.099.17M6.29 12.41l-.243.42a.266.266 0 01-.233.134h-.9l.42-.724h.857c.089 0 .14.095.099.17m1.902-1.855H4.61l-1.392 2.41H0v.48l6.599-.001c.24 0 .468-.125.595-.344l.41-.713c.086-.148-.005-.338-.163-.387a.698.698 0 00.583-.337l.402-.698a.272.272 0 00-.234-.41m9.986 0l-1.667 2.889h4.042l.277-.48h-2.346l.418-.724h2.346l.277-.48H19.18l.418-.726H24v-.479h-5.82z" /></svg>
   }
}