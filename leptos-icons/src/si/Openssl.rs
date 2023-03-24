#[cfg(feature = "SiOpenssl")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiOpenssl")]
/// *This icon requires the feature* `SiOpenssl` *to be enabled*.
#[component]
pub fn Openssl(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.961,13.532c-0.303,0-0.575-0.052-0.818-0.157c-0.243-0.105-0.448-0.25-0.616-0.437 c-0.168-0.187-0.298-0.408-0.389-0.664C0.046,12.018,0,11.741,0,11.442c0-0.299,0.046-0.576,0.137-0.832 c0.091-0.256,0.221-0.477,0.389-0.664C0.695,9.759,0.9,9.613,1.143,9.508c0.243-0.105,0.516-0.157,0.818-0.157 c0.303,0,0.575,0.052,0.818,0.157c0.243,0.105,0.448,0.25,0.616,0.437c0.168,0.187,0.298,0.408,0.389,0.664 c0.091,0.256,0.137,0.533,0.137,0.832c0,0.299-0.046,0.576-0.137,0.832c-0.092,0.256-0.221,0.477-0.389,0.664 c-0.168,0.187-0.374,0.333-0.616,0.437C2.536,13.479,2.264,13.532,1.961,13.532z M1.961,13.089c0.235,0,0.443-0.042,0.622-0.126 s0.331-0.199,0.454-0.345c0.123-0.146,0.216-0.319,0.277-0.521c0.062-0.202,0.092-0.42,0.092-0.656 c0-0.235-0.031-0.454-0.092-0.656c-0.062-0.202-0.154-0.376-0.277-0.524c-0.123-0.148-0.275-0.263-0.454-0.347 S2.197,9.789,1.961,9.789c-0.235,0-0.443,0.042-0.622,0.126c-0.179,0.084-0.331,0.2-0.454,0.347 c-0.123,0.148-0.216,0.322-0.277,0.524c-0.062,0.202-0.092,0.42-0.092,0.656c0,0.235,0.031,0.454,0.092,0.656 c0.062,0.202,0.154,0.375,0.277,0.521c0.123,0.146,0.275,0.261,0.454,0.345C1.519,13.047,1.726,13.089,1.961,13.089z M4.455,10.551 h0.454v0.409H4.92c0.108-0.149,0.237-0.266,0.387-0.35c0.149-0.084,0.331-0.126,0.544-0.126c0.183,0,0.351,0.035,0.504,0.104 c0.153,0.069,0.284,0.168,0.392,0.297c0.108,0.129,0.193,0.289,0.255,0.479c0.062,0.191,0.092,0.405,0.092,0.644 c0,0.232-0.029,0.442-0.087,0.63c-0.058,0.189-0.141,0.349-0.249,0.482c-0.108,0.133-0.241,0.234-0.398,0.305 c-0.157,0.071-0.333,0.106-0.527,0.106c-0.198,0-0.37-0.032-0.516-0.095c-0.146-0.063-0.278-0.174-0.398-0.331H4.909v1.557H4.455 V10.551z M5.8,13.151c0.138,0,0.26-0.029,0.364-0.087c0.105-0.058,0.191-0.137,0.261-0.238c0.069-0.101,0.12-0.22,0.154-0.359 c0.034-0.138,0.05-0.289,0.05-0.454c0-0.161-0.017-0.311-0.05-0.451c-0.034-0.14-0.085-0.261-0.154-0.364 c-0.069-0.103-0.155-0.183-0.258-0.241c-0.103-0.058-0.223-0.087-0.361-0.087c-0.157,0-0.294,0.03-0.412,0.09 c-0.118,0.06-0.214,0.142-0.289,0.247c-0.075,0.105-0.131,0.226-0.168,0.364c-0.037,0.138-0.056,0.286-0.056,0.443 c0,0.157,0.017,0.305,0.05,0.443c0.034,0.138,0.087,0.259,0.16,0.361c0.073,0.103,0.167,0.184,0.283,0.244 C5.49,13.121,5.632,13.151,5.8,13.151z M8.837,13.526c-0.22,0-0.417-0.04-0.591-0.12c-0.174-0.08-0.32-0.19-0.44-0.328 c-0.12-0.138-0.211-0.3-0.275-0.485c-0.064-0.185-0.095-0.382-0.095-0.591c0-0.209,0.033-0.406,0.098-0.591 c0.065-0.185,0.159-0.346,0.28-0.485c0.121-0.138,0.266-0.247,0.434-0.328c0.168-0.08,0.355-0.12,0.56-0.12 c0.217,0,0.408,0.04,0.574,0.12c0.166,0.08,0.305,0.192,0.417,0.336c0.112,0.144,0.197,0.316,0.255,0.516 c0.058,0.2,0.087,0.419,0.087,0.658H7.901c0.007,0.146,0.034,0.282,0.078,0.409c0.045,0.127,0.106,0.237,0.185,0.331 c0.078,0.093,0.174,0.166,0.286,0.219c0.112,0.052,0.241,0.078,0.387,0.078c0.213,0,0.387-0.049,0.521-0.148 c0.134-0.099,0.228-0.247,0.28-0.445h0.443c-0.067,0.314-0.209,0.555-0.426,0.723C9.438,13.442,9.165,13.526,8.837,13.526z M8.809,10.837c-0.134,0-0.254,0.023-0.359,0.07c-0.105,0.047-0.193,0.112-0.266,0.196c-0.073,0.084-0.133,0.183-0.179,0.297 c-0.047,0.114-0.078,0.236-0.092,0.367h1.743c-0.015-0.291-0.094-0.519-0.238-0.684C9.273,10.919,9.07,10.837,8.809,10.837z M11.12,10.932h0.011c0.105-0.138,0.231-0.247,0.378-0.328c0.148-0.08,0.328-0.12,0.541-0.12c0.303,0,0.541,0.079,0.714,0.238 c0.174,0.159,0.261,0.389,0.261,0.692v2.034h-0.454v-1.995c0-0.191-0.058-0.336-0.174-0.437c-0.116-0.101-0.276-0.151-0.482-0.151 c-0.116,0-0.222,0.02-0.319,0.059c-0.097,0.039-0.181,0.093-0.252,0.163c-0.071,0.069-0.126,0.153-0.165,0.252 c-0.039,0.099-0.059,0.208-0.059,0.328v1.782h-0.454v-2.897h0.454V10.932z M15.409,13.53c-0.549,0-0.972-0.116-1.271-0.347 c-0.299-0.231-0.456-0.561-0.47-0.989h0.8c0.03,0.244,0.116,0.418,0.259,0.521c0.143,0.103,0.359,0.155,0.648,0.155 c0.105,0,0.205-0.009,0.299-0.028c0.094-0.019,0.177-0.049,0.248-0.09c0.071-0.041,0.129-0.095,0.172-0.161 c0.043-0.066,0.065-0.146,0.065-0.24c0-0.098-0.024-0.178-0.07-0.242c-0.047-0.064-0.114-0.117-0.2-0.161 c-0.086-0.043-0.191-0.081-0.313-0.113c-0.122-0.032-0.26-0.065-0.414-0.099c-0.18-0.041-0.354-0.087-0.521-0.138 c-0.167-0.051-0.314-0.119-0.44-0.206c-0.126-0.086-0.227-0.197-0.304-0.332c-0.077-0.135-0.116-0.308-0.116-0.518 c0-0.199,0.038-0.374,0.116-0.524c0.077-0.15,0.184-0.276,0.321-0.378c0.137-0.101,0.3-0.178,0.487-0.228 c0.188-0.051,0.394-0.076,0.62-0.076c0.454,0,0.82,0.107,1.096,0.321c0.276,0.214,0.429,0.526,0.459,0.935h-0.783 c-0.023-0.203-0.105-0.355-0.248-0.456c-0.143-0.101-0.319-0.152-0.53-0.152c-0.222,0-0.398,0.043-0.53,0.13 c-0.132,0.086-0.197,0.201-0.197,0.344c0,0.083,0.018,0.151,0.054,0.206c0.036,0.055,0.09,0.102,0.163,0.144 c0.073,0.041,0.164,0.077,0.273,0.107c0.109,0.03,0.238,0.062,0.389,0.096c0.207,0.045,0.401,0.095,0.583,0.149 c0.182,0.055,0.342,0.127,0.479,0.217c0.137,0.09,0.245,0.205,0.324,0.344c0.079,0.139,0.118,0.318,0.118,0.535 c0,0.203-0.039,0.384-0.118,0.544c-0.079,0.16-0.188,0.293-0.327,0.4c-0.139,0.107-0.304,0.189-0.496,0.245 C15.843,13.501,15.635,13.53,15.409,13.53z M19.168,13.53c-0.549,0-0.972-0.116-1.271-0.347s-0.456-0.561-0.47-0.989h0.8 c0.03,0.244,0.116,0.418,0.259,0.521c0.143,0.103,0.359,0.155,0.648,0.155c0.105,0,0.205-0.009,0.299-0.028 c0.094-0.019,0.177-0.049,0.248-0.09c0.071-0.041,0.129-0.095,0.172-0.161c0.043-0.066,0.065-0.146,0.065-0.24 c0-0.098-0.024-0.178-0.07-0.242c-0.047-0.064-0.114-0.117-0.2-0.161c-0.086-0.043-0.191-0.081-0.313-0.113 c-0.122-0.032-0.26-0.065-0.414-0.099c-0.18-0.041-0.354-0.087-0.521-0.138c-0.167-0.051-0.314-0.119-0.44-0.206 c-0.126-0.086-0.227-0.197-0.304-0.332c-0.077-0.135-0.116-0.308-0.116-0.518c0-0.199,0.038-0.374,0.116-0.524 c0.077-0.15,0.184-0.276,0.321-0.378c0.137-0.101,0.3-0.178,0.487-0.228c0.188-0.051,0.394-0.076,0.62-0.076 c0.454,0,0.82,0.107,1.096,0.321c0.276,0.214,0.429,0.526,0.459,0.935h-0.783c-0.023-0.203-0.105-0.355-0.248-0.456 c-0.143-0.101-0.319-0.152-0.53-0.152c-0.222,0-0.398,0.043-0.53,0.13c-0.132,0.086-0.197,0.201-0.197,0.344 c0,0.083,0.018,0.151,0.054,0.206c0.036,0.055,0.09,0.102,0.163,0.144c0.073,0.041,0.164,0.077,0.273,0.107 c0.109,0.03,0.238,0.062,0.389,0.096c0.207,0.045,0.401,0.095,0.583,0.149c0.182,0.055,0.342,0.127,0.479,0.217 c0.137,0.09,0.245,0.205,0.324,0.344c0.079,0.139,0.118,0.318,0.118,0.535c0,0.203-0.039,0.384-0.118,0.544 c-0.079,0.16-0.188,0.293-0.327,0.4c-0.139,0.107-0.304,0.189-0.496,0.245C19.602,13.501,19.393,13.53,19.168,13.53z M21.29,9.41 h0.817v3.347H24v0.682h-2.71V9.41z" /></svg>
   }
}