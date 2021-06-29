use std::sync::Arc;

use crate::{Jot, Tag as JTag};

use druid::{
    text::RichText,
    widget::{Controller, ListIter},
    AppDelegate, Color, Command, Data, DelegateCtx, Env, Event, EventCtx, Handled, Lens, PaintCtx,
    Rect, RenderContext, Target, Widget,
};

use sqlx::SqlitePool;

mod markdown;
pub use markdown::*;

pub trait Labelable {
    fn short_label(&self, length: usize) -> String;
}

const GLORANGE: Color = Color::rgb8(207, 91, 1);
const BACK_BLUE: Color = Color::rgb8(5, 11, 110);
const ACTIVE_GREEN: Color = Color::rgb8(0, 150, 5);

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
    let mut inner_bounds = ctx.size();
    inner_bounds.width -= 16.0;
    inner_bounds.height -= 16.0;
    let smounds = Rect::from_center_size(bounds.center(), inner_bounds).to_rounded_rect(5.0);
    ctx.fill(smounds, &Color::rgb(0.4, 0.4, 0.4));
}

#[derive(Clone, Lens)]
pub struct AppState {
    rendered: RichText,
    current_jot: usize,
    current_tags: Arc<Vec<JTag>>,
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
            current_tags: Arc::new(vec![]),
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
