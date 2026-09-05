#![allow(non_snake_case)]

use crate::i18n::loader;
use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
pub async fn Sidebar(locale: String, signed_in: bool, username: Option<String>) -> Result<impl View> {
    let link = "block px-3 py-1.5 text-sm rounded text-muted-foreground hover:bg-muted hover:text-foreground no-underline";
    let user = username.unwrap_or_default();
    Ok(view! {
        <aside
            class="bg-surface w-48 shrink-0 border-r border-border"
            style="min-height:calc(100vh-3rem)"
        >
            <div class="py-3 flex flex-col gap-1">
                <a href=(format!("/{locale}/arcs/article")) class=(link)>
                    (loader::t(&locale, "nav_article"))
                </a>
                <a href=(format!("/{locale}/arcs/video")) class=(link)>
                    (loader::t(&locale, "nav_video"))
                </a>
                <a href=(format!("/{locale}/arcs/photo")) class=(link)>
                    (loader::t(&locale, "nav_photo"))
                </a>
                <div class="border-t border-border my-2 mx-3"></div>
                if signed_in {
                    <a href=(format!("/{locale}/users/{user}")) class=(link)>
                        (loader::t(&locale, "nav_my"))
                    </a>
                }
                <a href=(format!("/{locale}/upload")) class=(link)>
                    (loader::t(&locale, "nav_upload"))
                </a>
            </div>
        </aside>
    })
}
