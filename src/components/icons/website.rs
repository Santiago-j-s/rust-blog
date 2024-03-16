use leptos::{component, view, IntoAttribute, IntoView};

#[component]
pub fn WebsiteIcon(
    #[prop(default = 40)] size: u32,
    #[prop(into, default = "fill-black".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 24 24"
            class=class
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d="M10.17 13.83a1.264 1.264 0 010 1.844 1.309 1.309 0 01-1.844 0 6.496 6.496 0 010-9.179l4.596-4.596a6.496 6.496 0 019.18 0 6.496 6.496 0 010 9.18l-1.935 1.934a8.969 8.969 0 00-.52-3.142l.61-.624a3.871 3.871 0 000-5.505 3.871 3.871 0 00-5.504 0l-4.584 4.584a3.871 3.871 0 000 5.505m3.662-5.505a1.309 1.309 0 011.843 0 6.496 6.496 0 010 9.179l-4.596 4.596a6.496 6.496 0 01-9.18 0 6.496 6.496 0 010-9.18l1.935-1.934a9.09 9.09 0 00.52 3.155l-.61.61a3.871 3.871 0 000 5.506 3.871 3.871 0 005.504 0l4.584-4.584a3.871 3.871 0 000-5.505 1.264 1.264 0 010-1.843z"></path>
        </svg>
    }
}
