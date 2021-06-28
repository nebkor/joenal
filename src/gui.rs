use std::sync::Arc;

use crate::Jot;

use druid::text::{AttributesAdder, RichText, RichTextBuilder};
use druid::{
    widget::{Controller, ListIter},
    AppDelegate, Color, Command, Data, DelegateCtx, Env, Event, EventCtx, FontFamily, FontStyle,
    FontWeight, Handled, Lens, PaintCtx, Rect, RenderContext, Selector, Target, Widget,
};

use pulldown_cmark::{Event as ParseEvent, Parser, Tag};
use sqlx::SqlitePool;

pub trait Labelable {
    fn short_label(&self, length: usize) -> String;
}

const GLORANGE: Color = Color::rgb8(207, 91, 1);
const BACK_BLUE: Color = Color::rgb8(5, 11, 110);
const ACTIVE_GREEN: Color = Color::rgb8(0, 150, 5);

const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);
const OPEN_LINK: Selector<String> = Selector::new("joenal-gui.open-link");

pub fn jot_card_background(ctx: &mut PaintCtx, data: &Item, _env: &Env) {
    let bounds = ctx.size().to_rect();
    if ctx.is_hot() {
        ctx.fill(bounds, &GLORANGE);
    } else if data.2 == data.1 {
        ctx.fill(bounds, &ACTIVE_GREEN);
    } else {
        ctx.fill(bounds, &BACK_BLUE);
    }

    //
    let mut smounds = ctx.size();
    smounds.width -= 16.0;
    smounds.height -= 16.0;
    let smounds = Rect::from_center_size(bounds.center(), smounds).to_rounded_rect(5.0);
    ctx.fill(smounds, &Color::rgb(0.4, 0.4, 0.4));
}

/// Parse a markdown string and generate a `RichText` object with
/// the appropriate attributes.
pub fn rebuild_rendered_text(text: &str) -> RichText {
    let mut current_pos = 0;
    let mut builder = RichTextBuilder::new();
    let mut tag_stack = Vec::new();

    let parser = Parser::new(text);
    for event in parser {
        match event {
            ParseEvent::Start(tag) => {
                tag_stack.push((current_pos, tag));
            }
            ParseEvent::Text(txt) => {
                builder.push(&txt);
                current_pos += txt.len();
            }
            ParseEvent::End(end_tag) => {
                let (start_off, tag) = tag_stack
                    .pop()
                    .expect("parser does not return unbalanced tags");
                assert_eq!(end_tag, tag, "mismatched tags?");
                add_attribute_for_tag(
                    &tag,
                    builder.add_attributes_for_range(start_off..current_pos),
                );
                if add_newline_after_tag(&tag) {
                    builder.push("\n\n");
                    current_pos += 2;
                }
            }
            ParseEvent::Code(txt) => {
                builder.push(&txt).font_family(FontFamily::MONOSPACE);
                current_pos += txt.len();
            }
            ParseEvent::Html(txt) => {
                builder
                    .push(&txt)
                    .font_family(FontFamily::MONOSPACE)
                    .text_color(BLOCKQUOTE_COLOR);
                current_pos += txt.len();
            }
            ParseEvent::HardBreak => {
                builder.push("\n\n");
                current_pos += 2;
            }
            _ => (),
        }
    }
    builder.build()
}

fn add_newline_after_tag(tag: &Tag) -> bool {
    !matches!(
        tag,
        Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..)
    )
}

fn add_attribute_for_tag(tag: &Tag, mut attrs: AttributesAdder) {
    match tag {
        Tag::Heading(lvl) => {
            let font_size = match lvl {
                1 => 38.,
                2 => 32.0,
                3 => 26.0,
                4 => 20.0,
                5 => 16.0,
                _ => 12.0,
            };
            attrs.size(font_size).weight(FontWeight::BOLD);
        }
        Tag::BlockQuote => {
            attrs.style(FontStyle::Italic).text_color(BLOCKQUOTE_COLOR);
        }
        Tag::CodeBlock(_) => {
            attrs.font_family(FontFamily::MONOSPACE);
        }
        Tag::Emphasis => {
            attrs.style(FontStyle::Italic);
        }
        Tag::Strong => {
            attrs.weight(FontWeight::BOLD);
        }
        Tag::Link(_link_ty, target, _title) => {
            attrs
                .underline(true)
                .text_color(LINK_COLOR)
                .link(OPEN_LINK.with(target.to_string()));
        }
        // ignore other tags for now
        _ => (),
    }
}

#[derive(Clone, Lens)]
pub struct AppState {
    rendered: RichText,
    current_jot: usize,
    pool: sqlx::SqlitePool,
    jots: Arc<Vec<Jot>>,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.current_jot == other.current_jot && self.rendered.same(&other.rendered)
    }
}

impl AppState {
    pub fn new(
        rendered: RichText,
        current_jot: usize,
        pool: SqlitePool,
        jots: Arc<Vec<Jot>>,
    ) -> Self {
        AppState {
            rendered,
            current_jot,
            pool,
            jots,
        }
    }
}

#[derive(Clone, Data, Debug)]
pub struct Item(String, usize, usize);

impl Item {
    pub fn new(label: String, id: usize, current: usize) -> Self {
        Item(label, id, current)
    }

    pub fn is_current(&self) -> bool {
        self.1 == self.2
    }

    pub fn make_current(&mut self) {
        self.2 = self.1;
    }

    pub fn label(&self) -> &str {
        &self.0
    }
}

impl ListIter<Item> for AppState {
    fn for_each(&self, mut cb: impl FnMut(&Item, usize)) {
        for (i, item) in self.jots.iter().enumerate() {
            let s = item.short_label(50);
            let d = Item(s, i, self.current_jot);
            cb(&d, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Item, usize)) {
        let mut new_current_jot = self.current_jot;
        let mut any_changed = false;

        for (i, item) in self.jots.iter().enumerate() {
            let s = item.short_label(50);
            let mut d = Item(s, i, self.current_jot);
            cb(&mut d, i);

            // if !any_changed && !(*item, i, self.current_jot_room).same(&d) {
            if !self.current_jot.same(&d.2) {
                any_changed = true;
                new_current_jot = d.2;
            }
        }

        if any_changed {
            self.current_jot = new_current_jot;
            let text = std::str::from_utf8(self.jots[new_current_jot].content().bytes).unwrap();
            self.rendered = rebuild_rendered_text(text);
        }
    }

    fn data_len(&self) -> usize {
        self.jots.len()
    }
}

/// A controller that rebuilds the preview when edits occur
pub struct RichTextRebuilder;

impl<W: Widget<AppState>> Controller<AppState, W> for RichTextRebuilder {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        let pre_data = data.current_jot;
        child.event(ctx, event, data, env);
        if data.current_jot != pre_data {
            let jot = &data.jots[data.current_jot];
            let txt = std::str::from_utf8(jot.content().bytes).unwrap();
            data.rendered = rebuild_rendered_text(txt);
        }
    }
}

pub struct Delegate;

impl<T: Data> AppDelegate<T> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut T,
        _env: &Env,
    ) -> Handled {
        if let Some(url) = cmd.get(OPEN_LINK) {
            open::that_in_background(url);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
