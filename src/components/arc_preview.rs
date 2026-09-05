#![allow(non_snake_case)]

use crate::common::icons;
use crate::i18n::loader;
use crate::models::arc::Arc;
use topcoat::{
    Result,
    icon::icon,
    view::{View, component, view},
};

#[component]
pub async fn ArcPreview(locale: String, arc: Arc) -> Result<impl View> {
    let type_icon = match arc.arc_type.as_str() {
        "video" => icons::VIDEO,
        "photo" => icons::CAMERA,
        _ => icons::PENCIL,
    };
    let created_date: String = arc.created_at.chars().take(10).collect();
    // view! 生成异步闭包：借用与移动不可共存，统一提取 owned 值
    let href = format!("/{locale}/arc/{}", arc.id);
    let title = arc.title.clone();
    let author = arc.author_name.clone().unwrap_or_default();
    let type_label = match arc.arc_type.as_str() {
        "video" => "post_type_video",
        "photo" => "post_type_photo",
        _ => "post_type_article",
    };
    let thumbnail = arc.thumbnail.clone();
    Ok(view! {
        <a
            href=(href)
            class="bg-surface border border-border rounded-lg shadow-xs overflow-hidden no-underline hover:shadow-md transition-shadow block"
        >
            <div
                class="aspect-video bg-muted flex items-center justify-center relative"
            >
                if let Some(ref thumb) = thumbnail {
                    <img
                        src=(thumb.clone())
                        alt=(title.clone())
                        class="w-full h-full object-cover"
                    >
                } else {
                    icon(data: type_icon, size: 48)
                }
                <span
                    class="absolute top-2 left-2 text-xs px-1.5 py-0.5 rounded bg-black/60 text-white"
                >
                    (loader::t(&locale, type_label))
                </span>
            </div>
            <div class="p-3">
                <h3 class="font-medium text-sm text-foreground truncate">
                    (title.clone())
                </h3>
                <div class="flex items-center gap-2 text-xs text-muted-foreground mt-1">
                    <span>(author)</span>
                    <span>(created_date)</span>
                </div>
            </div>
        </a>
    })
}
