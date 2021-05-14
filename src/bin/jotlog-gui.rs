// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of live markdown preview

use std::sync::Arc;

use druid::{
    text::{AttributesAdder, RichText, RichTextBuilder},
    widget::{
        prelude::*, Button, Controller, Flex, Label, LineBreaking, List, ListIter, RawLabel,
        Scroll, Split,
    },
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, FontFamily, FontStyle, FontWeight,
    Handled, Lens, LensExt, LocalizedString, Selector, Target, UnitPoint, Widget, WidgetExt,
    WindowDesc,
};
use jotlog::{get_config, get_jots, make_pool, Jot};
use pulldown_cmark::{Event as ParseEvent, Parser, Tag};
use uuid::Uuid;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Joenal");

const SPACER_SIZE: f64 = 8.0;
const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);
const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config();
    std::env::set_var("DATABASE_URL", config.db_file);

    let conn = make_pool().await;

    // insert_jot(&conn, &jot);

    let jots = get_jots(&conn).await;
    let jot = &jots[0];

    // just between us friends, we don't have any non-utf8 bytes in our content yet
    let content: &str = std::str::from_utf8(jot.content().bytes).unwrap();

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((700.0, 600.0));

    let initial_state = AppState {
        rendered: rebuild_rendered_text(content),
        current_jot: 0,
        pool: conn.clone(),
        jots: Arc::new(jots),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");

    Ok(())
}

fn build_root_widget() -> impl Widget<AppState> {
    let rendered = Scroll::new(
        RawLabel::new()
            .with_text_color(Color::BLACK)
            .with_line_break_mode(LineBreaking::WordWrap)
            .lens(AppState::rendered)
            .expand_width()
            .padding((SPACER_SIZE * 4.0, SPACER_SIZE)),
    )
    .vertical()
    .background(Color::grey8(222))
    .expand();

    let jotbox = Scroll::new(List::new(|| {
        let label = Label::new(|item: &(Jot, usize, usize), _env: &_| {
            let jot = &item.0;
            jot.button_label()
        });
        let button = Button::from_label(label)
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5));

        button.on_click(|_event_ctx, data, _env| (*data).2 = data.1)
    }))
    .vertical();

    Split::columns(jotbox, rendered)
}

impl ListIter<(Jot, usize, usize)> for AppState {
    fn for_each(&self, mut cb: impl FnMut(&(Jot, usize, usize), usize)) {
        for (i, item) in self.jots.iter().enumerate() {
            let d = (item.to_owned(), i, self.current_jot);
            cb(&d, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut (Jot, usize, usize), usize)) {
        let mut new_data = Vec::with_capacity(self.data_len());
        let mut any_changed = false;
        let mut new_current_jot = self.current_jot;

        for (i, item) in self.jots.iter().enumerate() {
            let mut d = (item.to_owned(), i, self.current_jot);
            cb(&mut d, i);

            // if !any_changed && !(*item, i, self.current_jot_room).same(&d) {
            if !any_changed && !self.current_jot.same(&d.2) {
                dbg!(d.2, self.current_jot);
                any_changed = true;
                new_current_jot = d.2;
            }
            new_data.push(d.0);
        }

        if any_changed {
            self.jots = Arc::new(new_data);
            self.current_jot = new_current_jot;
            let text = std::str::from_utf8(self.jots[new_current_jot].content().bytes).unwrap();
            self.rendered = rebuild_rendered_text(text);
        }
    }

    fn data_len(&self) -> usize {
        self.jots.len()
    }
}

/// Parse a markdown string and generate a `RichText` object with
/// the appropriate attributes.
fn rebuild_rendered_text(text: &str) -> RichText {
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
struct AppState {
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

/// A controller that rebuilds the preview when edits occur
struct RichTextRebuilder;

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

struct Delegate;

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
