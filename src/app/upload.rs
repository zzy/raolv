use crate::db;

use crate::common::auth;
use crate::common::media;
use crate::common::session;
use crate::components;
use crate::components::csrf::CsrfField;
use crate::components::button::{ButtonVariant, button};
use crate::components::card::card;
use crate::components::input::input;
use crate::components::label::label;
use crate::db::arcs;
use crate::i18n::loader;
use topcoat::{
    Result,
    context::Cx,
    router::{
        content::multipart::Multipart,
        error::{SeeOther, bad_request, forbidden, internal_server_error, see_other},
        path_param_segment,
    },
    view::{View, attributes, view},
};

#[topcoat::router::page("/{locale}/upload")]
pub async fn upload_page(cx: &Cx) -> Result<impl View> {
    let locale = path_param_segment(cx, "locale");
    let csrf = session::ensure_csrf_token(cx).await.unwrap_or_default();
    let loc = locale.to_string();
    Ok(view! {
        <main class="max-w-xl mx-auto px-4 py-8">
            card(
                attrs: attributes! { class="p-6" },
                <h1 class="text-xl font-bold text-foreground mb-6">
                    (loader::t(&locale, "upload_title"))
                </h1>
                <form
                    action=""
                    method="post"
                    enctype="multipart/form-data"
                    class="space-y-4"
                >
                    CsrfField(token: csrf.clone())
                    <div class="space-y-1">
                        label(
                            attrs: attributes! {},
                            (loader::t(&locale, "upload_label_title"))
                        )
                        input(
                            attrs: attributes! { type="text" name="title" required="" }
                        )
                    </div>
                    <div class="space-y-1">
                        label(
                            attrs: attributes! {},
                            (loader::t(&locale, "upload_label_type"))
                        )
                        <select
                            name="post_type"
                            class="h-9 w-full min-w-0 rounded-lg border border-border bg-background px-3 text-sm shadow-xs transition-colors outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:pointer-events-none disabled:opacity-50"
                            onchange=(format!(
                                "var b=document.getElementById('body-section');var f=document.getElementById('file-section');var t=this.value;b.style.display=t==='article'?'':'none';f.style.display=t==='article'?'none':''",
                            ))
                        >
                            <option value="video">
                                (loader::t(&locale, "post_type_video"))
                            </option>
                            <option value="article">
                                (loader::t(&locale, "post_type_article"))
                            </option>
                            <option value="photo">
                                (loader::t(&locale, "post_type_photo"))
                            </option>
                        </select>
                    </div>
                    <div id="body-section" style="display:none">
                        label(
                            attrs: attributes! {},
                            (loader::t(&locale, "upload_label_body"))
                        )
                        components::markdown_editor::MarkdownEditor(
                            locale: loc.clone(),
                            name: "body".to_string(),
                            rows: 12,
                            value: String::new(),
                            required: false,
                            csrf: csrf.clone()
                        )
                    </div>
                    <div>components::topic_input::TopicInput(locale: loc.clone())</div>
                    <div id="file-section" class="space-y-1">
                        label(
                            attrs: attributes! {},
                            (loader::t(&locale, "upload_select_file"))
                        )
                        <input
                            type="file"
                            name="file"
                            accept="video/*,image/*"
                            class="w-full text-sm file:mr-3 file:py-1.5 file:px-3 file:rounded file:border-0 file:bg-blue-50 file:text-blue-700"
                        >
                    </div>
                    button(
                        variant: ButtonVariant::Primary,
                        attrs: attributes! { type="submit" class="w-full justify-center" },
                        (loader::t(&locale, "upload_submit"))
                    )
                </form>
            )
        </main>
    })
}

#[topcoat::router::route(POST "/{locale}/upload")]
pub async fn upload_handler(cx: &Cx, mut form_data: Multipart) -> Result<SeeOther> {
    let locale = path_param_segment(cx, "locale");
    let (mut title, mut content_type) = (String::new(), String::from("video"));
    let mut body = String::new();
    let mut topics = String::new();
    let mut file_data: Option<(Vec<u8>, String)> = None;
    let mut csrf = String::new();
    while let Some(field) = form_data
        .next_field()
        .await
        .map_err(|e| bad_request(e.to_string()))?
    {
        match field.name().unwrap_or("") {
            "title" => title = field.text().await.unwrap_or_default(),
            "post_type" => content_type = field.text().await.unwrap_or_default(),
            "body" => body = field.text().await.unwrap_or_default(),
            "topics" => topics = field.text().await.unwrap_or_default(),
            "csrf_token" => csrf = field.text().await.unwrap_or_default(),
            "file" => {
                let mime = field.content_type().unwrap_or("").to_string();
                let bytes = field.bytes().await.unwrap_or_default().to_vec();
                file_data = Some((bytes, mime));
            }
            _ => {}
        }
    }
    // CSRF 校验必须在任何副作用（文件写入/建记录）之前
    if !session::verify_csrf(cx, &csrf).await {
        return Err(forbidden().into());
    }
    let (file_bytes, file_mime) = match (&content_type[..], file_data) {
        ("article", _) => (Vec::new(), String::new()),
        (_, Some((d, mime))) if !d.is_empty() => (d, mime),
        _ => {
            return Err(bad_request(loader::t(&locale, "upload_no_file")).into());
        }
    };
    // 32 位 hex：DB key（无引号字符，可安全往返）
    let id = db::new_record_key();
    let media_url = if file_bytes.is_empty() {
        String::new()
    } else {
        if file_bytes.len() > media::UPLOAD_MAX_BYTES {
            return Err(bad_request(loader::t(&locale, "upload_too_large")).into());
        }
        let Some(ext) = media::mime_to_extension(&file_mime) else {
            return Err(bad_request(loader::t(&locale, "upload_verify_failed")).into());
        };
        let result = if media::is_image_ext(ext) {
            media::save_image(&file_bytes, ext).await
        } else {
            media::save_video(&file_bytes, ext).await
        };
        match result {
            Ok(url) => url,
            Err(e) => {
                return Err(bad_request(format!(
                    "{}: {e}",
                    loader::t(&locale, "upload_verify_failed")
                ))
                .into());
            }
        }
    };
    let body_val = if body.is_empty() {
        None
    } else {
        Some(&body[..])
    };
    let topics_val = if topics.is_empty() {
        None
    } else {
        Some(&topics[..])
    };
    let author = auth::current_user(cx).await.unwrap_or_default();
    let author_val = if author.is_empty() {
        None
    } else {
        Some(author.as_str())
    };
    match arcs::create_arc(
        &id,
        &title,
        &content_type,
        &media_url,
        body_val,
        topics_val,
        author_val,
        None,
    )
    .await
    {
        Ok(()) => {
            super::notify::notify(cx, &title);
            Ok(see_other(&format!("/{locale}")))
        }
        Err(e) => {
            let _ = media::remove_upload(&media_url).await;
            Err(internal_server_error(std::io::Error::other(e.to_string())).into())
        }
    }
}

