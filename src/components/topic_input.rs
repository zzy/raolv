#![allow(non_snake_case)]

use crate::components::label::label;
use crate::i18n::loader;
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

/// 话题输入组件 — 注册页/发表页共用
/// 渲染为可交互的标签输入框，隐藏域提交逗号分隔的话题字符串
#[component]
pub async fn TopicInput(locale: String) -> Result<impl View> {
    let placeholder = loader::t(&locale, "input_topics");
    Ok(view! {
        <div>
            label(attrs: attributes! {}, (loader::t(&locale, "input_topics")))
            <div
                class="form-input flex flex-wrap items-center gap-1 cursor-text"
                onclick="var i=this.querySelector('input');if(i)i.focus()"
            >
                <div class="flex flex-wrap items-center gap-1" id="topic-tags"></div>
                <input
                    type="text"
                    class="border-0 outline-none flex-1 bg-transparent text-sm"
                    placeholder=(placeholder)
                    onkeydown="var v=this.value.trim();var tags=document.getElementById('topic-tags');if((event.key==='Enter'||event.key===','||event.key===' ')&&v){event.preventDefault();addTopic(v,tags);this.value=''}else if(event.key==='Backspace'&&!this.value){var last=tags.lastChild;if(last)last.remove();updateTopicInput()}"
                >
            </div>
            <input type="hidden" name="topics" id="topic-input-hidden" value="">
        </div>
        <script>
            (format!("var topicList=[];function addTopic(n,tags){{n=n.toLowerCase().trim();if(!n||topicList.includes(n))return;topicList.push(n);var s=document.createElement('span');s.className='badge-blue inline-flex items-center gap-1 text-xs';s.innerHTML=n+'<button type=\"button\" class=\"ml-0.5 text-blue-500 hover:text-red-500 font-bold leading-none cursor-pointer border-0 bg-transparent p-0 text-base\" onclick=\"this.parentElement.remove();var i=topicList.indexOf(\\''+n+'\\');if(i>-1)topicList.splice(i,1);updateTopicInput()\">×</button>';tags.appendChild(s);updateTopicInput()}}function updateTopicInput(){{document.getElementById('topic-input-hidden').value=topicList.join(',')}}"))
        </script>
    })
}
