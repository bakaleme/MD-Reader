use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use std::path::Path;

#[derive(Clone, PartialEq)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocEntry>,
}

pub fn render_markdown(text: &str) -> RenderedMarkdown {
    let text = strip_frontmatter(text);

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);

    let events: Vec<Event> = Parser::new_ext(text, opts).collect();

    let mut toc = Vec::new();
    let mut heading_buf = String::new();
    let mut heading_level = 0u8;
    let mut heading_depth = 0u32;

    for ev in &events {
        match ev {
            Event::Start(Tag::Heading { level, .. }) => {
                heading_depth += 1;
                if heading_depth == 1 {
                    heading_level = *level as u8;
                    heading_buf.clear();
                }
            }
            Event::End(TagEnd::Heading(..)) => {
                heading_depth -= 1;
                if heading_depth == 0 {
                    let id = slugify(&heading_buf);
                    toc.push(TocEntry {
                        level: heading_level,
                        text: heading_buf.clone(),
                        id,
                    });
                }
            }
            _ if heading_depth > 0 => {
                push_event_text(ev, &mut heading_buf);
            }
            _ => {}
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    let html_output = wrap_code_blocks(&html_output);
    RenderedMarkdown { html: html_output, toc }
}

const COPY_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>"#;

fn wrap_code_blocks(html: &str) -> String {
    let mut result = String::with_capacity(html.len() + html.len() / 4);
    let mut rest = html;

    while let Some(pre_start) = rest.find("<pre>") {
        result.push_str(&rest[..pre_start]);
        let after_pre = &rest[pre_start + 5..];

        if let Some(pre_end) = after_pre.find("</pre>") {
            let inner = &after_pre[..pre_end];
            let lang = extract_language(inner);

            result.push_str(r#"<div class="code-block-wrapper">"#);
            result.push_str(r#"<div class="code-block-header">"#);
            if let Some(lang) = lang {
                result.push_str(r#"<span class="code-block-lang">"#);
                result.push_str(lang);
                result.push_str("</span>");
            } else {
                result.push_str(r#"<span class="code-block-lang code-block-lang-empty"></span>"#);
            }
            result.push_str(r#"<button class="copy-btn" onclick="window.copyCode(this)" type="button" title="Copy to clipboard" aria-label="Copy to clipboard">"#);
            result.push_str(r#"<span class="copy-icon">"#);
            result.push_str(COPY_ICON);
            result.push_str(r#"</span><span class="copy-label">Copy</span></button></div>"#);

            result.push_str("<pre>");
            result.push_str(inner);
            result.push_str("</pre></div>");
            rest = &after_pre[pre_end + 6..];
        } else {
            result.push_str(&rest[pre_start..]);
            break;
        }
    }
    result.push_str(rest);
    result
}

/// Extract the language identifier from `<code class="language-xxx">` inside a `<pre>` block.
fn extract_language(inner: &str) -> Option<&str> {
    // pulldown-cmark generates: <pre><code class="language-rust">...</code></pre>
    let code_open = inner.find("<code")?;
    let after_code = &inner[code_open..];
    let class_start = after_code.find(r#"class=""#)? + 7;
    let class_segment = &after_code[class_start..];
    let class_end = class_segment.find('"')?;
    let classes = &class_segment[..class_end];
    classes
        .split_whitespace()
        .find(|c| c.starts_with("language-"))
        .map(|c| &c[9..])
}

fn push_event_text(ev: &Event, buf: &mut String) {
    match ev {
        Event::Text(t) | Event::Code(t) => buf.push_str(t),
        Event::Html(t) | Event::InlineHtml(t) => buf.push_str(t),
        Event::SoftBreak | Event::HardBreak => {
            if !buf.ends_with(' ') {
                buf.push(' ');
            }
        }
        _ => {}
    }
}

fn strip_frontmatter(text: &str) -> &str {
    if let Some(rest) = text.strip_prefix("---") {
        if let Some(end) = rest.find("\n---") {
            return rest[end + 4..].trim_start();
        }
    }
    if let Some(rest) = text.strip_prefix("+++") {
        if let Some(end) = rest.find("\n+++") {
            return rest[end + 4..].trim_start();
        }
    }
    text
}

fn slugify(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_dash = false;

    for ch in text.to_lowercase().chars() {
        if ch.is_alphanumeric() {
            result.push(ch);
            prev_dash = false;
        } else if !prev_dash {
            result.push('-');
            prev_dash = true;
        }
    }

    result.trim_matches('-').to_string()
}

/// Check whether a path has a Markdown extension (case-insensitive).
pub fn is_markdown(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .as_deref(),
        Some("md" | "markdown" | "mdown" | "mkd" | "mkdn")
    )
}
