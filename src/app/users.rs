#![allow(non_snake_case)]

use crate::common::config;
use crate::components;
use crate::components::badge::{BadgeVariant, badge};
use crate::components::card::{card, card_content, card_description, card_footer, card_header, card_title};
use crate::db::{arcs, users};
use crate::i18n::loader;
use crate::models::page::PageInfo;
use topcoat::{
    Result,
    context::Cx,
    router::{page, path_param_segment, query_params},
    view::{View, attributes, view},
};

#[query_params]
pub struct UserQuery {
    pub page: Option<u64>,
}

/// 用户主页 — 显示用户信息及其发布的内容
#[page("/{locale}/users/{username}")]
pub async fn user_profile(cx: &Cx) -> Result<impl View> {
    let locale = path_param_segment(cx, "locale");
    let username = path_param_segment(cx, "username");
    let params = query_params::<UserQuery>(cx).ok();
    let page = params.as_ref().and_then(|p| p.page).unwrap_or(1).max(1);
    let user = users::get_user_profile(username).await.ok().flatten();
    let page_size = config::config().page_size as u64;
    let total = arcs::count_arcs_by_author(username)
        .await
        .unwrap_or(0);
    let page_info = PageInfo::new(total, page, page_size);
    let arc_list = arcs::get_arcs_by_author(username, page_info.current_page, page_size)
        .await
        .unwrap_or_default();
    let loc = locale.to_string();
    let n = arc_list.len();
    let locales: Vec<String> = std::iter::repeat(loc.clone()).take(n).collect();
    let base_url = format!("/{locale}/users/{username}");
    Ok(view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            if let Some(ref u) = user {
                // 用户信息卡片
                card(
                    attrs: attributes! { class="p-6 mb-8" },
                    card_header(
                        card_title((u.username.clone()))
                        if !u.introduction.is_empty() {
                            card_description(
                                (topcoat::view::Unescaped::new_unchecked(
                                    crate::common::markdown::render_md(&u.introduction),
                                ))
                            )
                        }
                    )
                    if !u.topics.is_empty() {
                        card_content(
                            attrs: attributes! { class="pt-0" },
                            <div class="flex flex-wrap gap-2">
                                for topic in u
                                    .topics
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect::<Vec<_>>() {
                                    badge(
                                        variant: BadgeVariant::Primary,
                                        attrs: attributes! {},
                                        (topic)
                                    )
                                }
                            </div>
                        )
                    }
                    card_footer(
                        (loader::t(&locale, "my_arcs"))
                        ": "
                        (total)
                    )
                )
                if arc_list.is_empty() {
                    <div class="text-center py-16">
                        <p class="text-base text-muted-foreground">
                            (loader::t(&locale, "no_data"))
                        </p>
                    </div>
                } else {
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                        for (item, lc) in arc_list.into_iter().zip(locales) {
                            components::arc_preview::ArcPreview(locale: lc, arc: item)
                        }
                    </div>
                    components::pagination::Pagination(
                        locale: loc,
                        page_info: page_info,
                        base_url: base_url
                    )
                }
            } else {
                (topcoat::router::StatusCode::NOT_FOUND)
                <div class="text-center py-16">
                    <h1
                        class="text-7xl font-bold text-blue-600 dark:text-blue-400 mb-4"
                    >
                        "404"
                    </h1>
                    <p class="text-muted-foreground mb-4">
                        (loader::t(&locale, "page_error_404"))
                    </p>
                    <a
                        href=(format!("/{locale}"))
                        class="text-blue-600 dark:text-blue-400 hover:underline"
                    >
                        (loader::t(&locale, "go_home"))
                    </a>
                </div>
            }
        </div>
    })
}
