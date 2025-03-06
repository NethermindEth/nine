/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use regex::{Match, Regex};
use slugify_rs::slugify;
use std::collections::VecDeque;
use thiserror::Error;
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Classes, Html};

// const PATT: &str = r#"(?s)^\s*%\[(?P<attr>\w+)="(?P<value>\w+)"\]%\s*(?P<text>.*)$"#;

/// A pattern for parsing attributes.
const PATT: &str = r#"^\s*(?P<attr>\w+)\s*=\s*"(?P<value>\w+)"\s*$"#;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No parent element")]
    NoParent,
    #[error("Lost kind of a codeblock")]
    LostKind,
    #[error("Lost aligns of a table")]
    LostAligns,
}

pub fn slug_title(title: &str) -> String {
    slugify(&title, "", "-", None, false, 0)
}

pub trait RenderRules: 'static {
    /// Fills links according to the parameters
    ///
    /// The first parameter is a mutable reference to a link tag.
    /// The second is an `href` value.
    ///
    /// If you want to set just `href` attribute, use
    /// [`set_href`] as a filler.
    fn fill_link(&self, tag: &mut VTag, text: &str);
}

pub struct ConfigurableRules {
    pub blank: bool,
    pub href: bool,
}

impl Default for ConfigurableRules {
    fn default() -> Self {
        Self {
            blank: true,
            href: true,
        }
    }
}

impl RenderRules for ConfigurableRules {
    fn fill_link(&self, tag: &mut VTag, text: &str) {
        if self.blank {
            tag.add_attribute("target", "_blank");
        }
        if self.href {
            tag.add_attribute("href", text.to_string());
        }
    }
}

pub struct MarkdownRender {
    filler: Box<dyn RenderRules + Sync + Send>,
    // TODO: Regex and extra rules for text blocks should be moved to the Rules trait
    regex: Regex,
}

impl MarkdownRender {
    // pub fn new(filler: impl RenderRules) -> Self {
    pub fn new() -> Self {
        Self {
            filler: Box::new(ConfigurableRules::default()),
            regex: Regex::new(PATT).unwrap(),
        }
    }

    pub fn inline(&self, src: &str) -> Html {
        InlineProcessor::new(self, src).render()
    }

    pub fn block(&self, src: &str) -> Html {
        BlockProcessor::new(self, src).render()
    }
}

/// Adds a class to the VTag.
/// You can also provide multiple classes separated by ascii whitespaces.
///
/// Note that this has a complexity of O(n),
/// where n is the number of classes already in VTag plus
/// the number of classes to be added.
fn add_class(vtag: &mut VTag, class: impl Into<Classes>) {
    let mut classes: Classes = vtag
        .attributes
        .iter()
        .find(|(k, _)| *k == "class")
        .map(|(_, v)| Classes::from(v.to_owned()))
        .unwrap_or_default();
    classes.push(class);
    vtag.add_attribute("class", classes.to_string());
}

pub fn set_href(el: &mut VTag, href: &str) {
    el.add_attribute("href", href.to_string());
}

struct InlineProcessor<'a> {
    render: &'a MarkdownRender,
    source: &'a str,
}

impl<'a> InlineProcessor<'a> {
    pub fn new(render: &'a MarkdownRender, source: &'a str) -> Self {
        Self {
            render,
            source,
        }
    }
}

impl<'a> InlineProcessor<'a> {
    pub fn render(&mut self) -> Html {
        // TODO: Render the real `Error`
        self.render_opt().ok().unwrap_or_default()
    }

    pub fn render_opt(&mut self) -> Result<Html, Error> {
        let src = self.source;
        let mut elems = Vec::new();
        let mut spine = VecDeque::new();

        let options = Options::empty();
        for ev in Parser::new_ext(src, options) {
            match ev {
                Event::Start(tag) => {
                    spine.push_back(self.make_inline_tag(tag));
                }
                Event::End(_tag) => {
                    let l = spine.len();
                    let top = spine.pop_back().ok_or(Error::NoParent)?;
                    if l == 1 {
                        elems.push(top);
                    } else {
                        spine[l - 2].add_child(top.into());
                    }
                }
                Event::Text(text) => {
                    let vtag = spine.back_mut().ok_or(Error::NoParent)?;
                    vtag.add_child(VText::new(text.to_string()).into());
                }
                Event::Code(code) => {
                    let mut tag = VTag::new("code");
                    tag.add_child(VText::new(code.to_string()).into());
                    spine
                        .back_mut()
                        .ok_or(Error::NoParent)?
                        .add_child(tag.into());
                }
                _ => {}
            }
        }
        Ok(html! {
            <span class="markdown inline">{ for elems.into_iter() }</span>
        })
    }

    fn make_inline_tag(&mut self, t: Tag) -> VTag {
        match t {
            Tag::Paragraph => VTag::new("span"),
            Tag::Emphasis => VTag::new("i"),
            Tag::Strong => VTag::new("b"),
            Tag::Strikethrough => VTag::new("s"),
            Tag::Link { title, dest_url, .. } => {
                let mut el = VTag::new("a");
                if !title.is_empty() {
                    el.add_attribute("title", title.to_string());
                }
                self.render.filler.fill_link(&mut el, &dest_url);
                el
            }
            Tag::Image { title, dest_url, .. } => {
                let mut el = VTag::new("img");
                el.add_attribute("src", dest_url.to_string());
                if !title.is_empty() {
                    el.add_attribute("title", title.to_string());
                }
                el
            }
            _ => VTag::new("span"),
        }
    }
}

struct BlockProcessor<'a> {
    render: &'a MarkdownRender,
    source: &'a str,
    kinds: VecDeque<CodeBlockKind<'a>>,
    aligns: VecDeque<Vec<Alignment>>,
}

impl<'a> BlockProcessor<'a> {
    pub fn new(render: &'a MarkdownRender, source: &'a str) -> Self {
        Self {
            render,
            source,
            kinds: VecDeque::new(),
            aligns: VecDeque::new(),
        }
    }
}

impl<'a> BlockProcessor<'a> {
    pub fn render(&mut self) -> Html {
        // TODO: Render the real `Error`
        self.render_opt().ok().unwrap_or_default()
    }

    /// Renders a string of Markdown to HTML with the default options (footnotes
    /// disabled, tables enabled).
    pub fn render_opt(&mut self) -> Result<Html, Error> {
        let src = self.source;
        let mut parse_attributes = false;
        let mut elems = Vec::new();
        let mut spine = VecDeque::new();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);

        for ev in Parser::new_ext(src, options) {
            match ev {
                Event::Start(tag) => {
                    spine.push_back(self.make_tag(tag));
                }
                Event::End(tag) => {
                    // TODO Verify stack end.
                    let l = spine.len();
                    let mut top = spine.pop_back().ok_or(Error::NoParent)?;
                    if let TagEnd::Heading { .. } = tag {
                        if let Some(item) = top.children().iter().next() {
                            if let VNode::VText(text_node) = item {
                                let id = slug_title(&*text_node.text);
                                top.add_attribute("id", id);
                            }
                        }
                    } else if let TagEnd::CodeBlock = tag {
                        let kind = self.kinds.pop_back()
                            .ok_or_else(|| Error::LostKind)?;
                        let label = make_badge_label(&kind);
                        // let mut pre = VTag::new("pre");
                        // // pre.add_attribute("lang", );
                        // // add_class(&mut pre, "bg-secondary text-white p-2");
                        // pre.add_child(top.into());

                        let mut codeblock_header = VTag::new("div");
                        add_class(&mut codeblock_header, "codeblock-header");
                        if !label.is_empty() {
                            let badge = html! {
                                <div class="badge">{ label }</div>
                            };
                            codeblock_header.add_child(badge.into());
                        }

                        let mut codeblock = VTag::new("div");
                        add_class(&mut codeblock, "codeblock");
                        codeblock.add_child(codeblock_header.into());
                        codeblock.add_child(top.into());

                        top = codeblock;
                        self.kinds.push_back(kind);

                    } else if let TagEnd::Table = tag {
                        let aligns = self.aligns.pop_back()
                            .ok_or_else(|| Error::LostAligns)?;
                        if let Some(children) = top.children_mut() {
                            for r in children.to_vlist_mut().iter_mut() {
                                if let VNode::VTag(ref mut vtag) = r {
                                    if let Some(children) = vtag.children_mut() {
                                        for (i, c) in children.to_vlist_mut().iter_mut().enumerate()
                                        {
                                            if let VNode::VTag(ref mut vtag) = c {
                                                match aligns[i] {
                                                    Alignment::None => {}
                                                    Alignment::Left => {
                                                        add_class(vtag, "align-left")
                                                    }
                                                    Alignment::Center => {
                                                        add_class(vtag, "align-center")
                                                    }
                                                    Alignment::Right => {
                                                        add_class(vtag, "align-right")
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if let TagEnd::TableHead = tag {
                        if let Some(children) = top.children_mut() {
                            for c in children.to_vlist_mut().iter_mut() {
                                if let VNode::VTag(ref mut vtag) = c {
                                    // TODO
                                    // vtag.tag = "th".into();
                                    vtag.add_attribute("scope", "col");
                                }
                            }
                        }
                    }
                    if l == 1 {
                        elems.push(top);
                    } else {
                        spine[l - 2].add_child(top.into());
                    }
                }
                Event::Text(text) => {
                    let vtag = spine.back_mut().ok_or(Error::NoParent)?;
                    match text.as_ref() {
                        "[" => {
                            parse_attributes = true;
                        }
                        "]" => {
                            parse_attributes = false;
                        }
                        attr_value if parse_attributes => {
                            if let Some(caps) = self.render.regex.captures(&attr_value) {
                                let attr = caps.name("attr").as_ref().map(Match::as_str);
                                let value = caps.name("value").as_ref().map(Match::as_str);
                                if let (Some("class"), Some(value)) = (attr, value) {
                                    vtag.add_attribute("class", value.to_owned());
                                } else {
                                    log::error!("Unsupported attribute: {}", text);
                                }
                            } else {
                                log::error!("Bad attribute: {}", text);
                            }
                        }
                        text => {
                            vtag.add_child(VText::new(text.to_string()).into());
                        }
                    }
                }
                Event::Code(code) => {
                    let mut tag = VTag::new("code");
                    tag.add_child(VText::new(code.to_string()).into());
                    spine
                        .back_mut()
                        .ok_or(Error::NoParent)?
                        .add_child(tag.into());
                }
                Event::Rule => {
                    spine
                        .back_mut()
                        .ok_or(Error::NoParent)?
                        .add_child(VTag::new("hr").into());
                }
                Event::SoftBreak => {
                    spine
                        .back_mut()
                        .ok_or(Error::NoParent)?
                        .add_child(VText::new("\n").into());
                }
                Event::HardBreak => {
                    spine
                        .back_mut()
                        .ok_or(Error::NoParent)?
                        .add_child(VTag::new("br").into());
                }
                _ => log::warn!("Unknown event: {:#?}", ev),
            }
        }

        Ok(html! {
            <div class="markdown">{ for elems.into_iter() }</div>
        })
    }

    fn make_tag(&mut self, t: Tag) -> VTag {
        match t {
            Tag::Paragraph => VTag::new("p"),
            Tag::Emphasis => VTag::new("i"),
            Tag::Strong => VTag::new("b"),
            Tag::Strikethrough => VTag::new("s"),
            Tag::Link { title, dest_url, .. } => {
                let mut el = VTag::new("a");
                if !title.is_empty() {
                    el.add_attribute("title", title.to_string());
                }
                self.render.filler.fill_link(&mut el, &dest_url);
                el
            }
            Tag::Image { title, dest_url, .. } => {
                let mut el = VTag::new("img");
                el.add_attribute("src", dest_url.to_string());
                if !title.is_empty() {
                    el.add_attribute("title", title.to_string());
                }
                el
            }

            Tag::Heading { level, .. } => VTag::new(level.to_string()),
            Tag::BlockQuote(_) => {
                let mut el = VTag::new("blockquote");
                el.add_attribute("class", "blockquote");
                el
            }
            Tag::CodeBlock(code_block_kind) => {
                let mut el = VTag::new("pre");

                if let CodeBlockKind::Fenced(lang) = code_block_kind {
                    el.add_attribute("class", format!("codeblock-highlight-{lang}"));
                    /*
                    // Different color schemes may be used for different code blocks,
                    // but a different library (likely js based at the moment) would be necessary to actually provide the
                    // highlighting support by locating the language classes and applying dom transforms
                    // on their contents.
                    match lang.as_ref() {
                        "html" => el.add_attribute("class", "html-language"),
                        "rust" => el.add_attribute("class", "rust-language"),
                        "java" => el.add_attribute("class", "java-language"),
                        "c" => el.add_attribute("class", "c-language"),
                        _ => {} // Add your own language highlighting support
                    };
                    */
                }

                el
            }
            Tag::List(None) => VTag::new("ul"),
            Tag::List(Some(1)) => VTag::new("ol"),
            Tag::List(Some(ref start)) => {
                let mut el = VTag::new("ol");
                el.add_attribute("start", start.to_string());
                el
            }
            Tag::Item => VTag::new("li"),
            Tag::Table(aligns) => {
                self.aligns.push_back(aligns);
                let mut el = VTag::new("table");
                el.add_attribute("class", "table");
                el
            }
            Tag::TableHead => {
                let mut el = VTag::new("tr");
                el.add_attribute("class", "table-head");
                el
            }
            Tag::TableRow => VTag::new("tr"),
            Tag::TableCell => {
                // TODO: Consider using table head (th) here
                VTag::new("td")
            }
            Tag::HtmlBlock => {
                VTag::new("div")
            }
            Tag::MetadataBlock(_) => {
                VTag::new("div")
            }
            Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"), // Footnotes are not rendered as anything special
            Tag::DefinitionList => {
                VTag::new("div")
            },
            Tag::DefinitionListTitle => {
                VTag::new("div")
            },
            Tag::DefinitionListDefinition => {
                VTag::new("div")
            },
            Tag::Superscript => {
                VTag::new("sup")
            },
            Tag::Subscript => {
                VTag::new("sub")
            },
        }
    }
}

// TODO: Add `Syntax`, `Code` types

fn make_badge_label<'a>(kind: &'a CodeBlockKind) -> &'a str {
    let lang = match kind {
        CodeBlockKind::Indented => "",
        CodeBlockKind::Fenced(kind) => kind.as_ref(),
    };
    // TODO: Use capitalize() instead?
    let name = match lang.to_lowercase().as_ref() {
        "rust" => "Rust",
        "url" => "URL",
        "toml" => "TOML",
        "json" => "JSON",
        "sh" | "shell" => "Shell",
        "bash" => "Bash",
        "output" => "Output",
        _other => lang.trim(),
    };
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attributes() {
        let regex = Regex::new(PATT).unwrap();
        let text = r#"myattr="value""#;
        let caps = regex.captures(text).unwrap();
        let attr = caps.name("attr").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();
        assert_eq!(attr, "myattr");
        assert_eq!(value, "value");
    }

    /*
    #[test]
    fn test_attributes() {
        let regex = Regex::new(PATT).unwrap();
        let text = r#"%[myattr="value"]% This is an example with
            multiple lines
            and spaces!"#;
        let caps = regex.captures(text).unwrap();
        let attr = caps.name("attr").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();
        let text = caps.name("text").unwrap().as_str();
        assert_eq!(attr, "myattr");
        assert_eq!(value, "value");
        assert!(text.starts_with("This is"));
    }
    */
}
