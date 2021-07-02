// Portions originally licensed under the Apache License from the Druid Authors,
// Version 2.0 (the "License"); you may not use this file except in compliance
// with the License. You may obtain a copy of the License at
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
    widget::{Label, LineBreaking, List, Painter, RawLabel, Scroll, Split},
    AppLauncher, Color, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use joenal::{Tag, get_config, get_jots, get_tags, gui::*, make_pool};

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Joenal");

const SPACER_SIZE: f64 = 8.0;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config();
    std::env::set_var("DATABASE_URL", config.db_file);

    let conn = make_pool().await;

    // insert_jot(&conn, &jot);

    let jots = get_jots(&conn).await;
    let jot = &jots[0];

    let tags = get_tags(&conn).await;

    // just between us friends, we don't have any non-utf8 bytes in our content yet
    let content: &str = std::str::from_utf8(jot.content().bytes).unwrap();

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((700.0, 600.0));

    let initial_state = AppState::new(
        rebuild_rendered_text(content),
        0,
        conn.clone(),
        Arc::new(jots),
        Arc::new(tags),
    );

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
        let label = Label::new(|item: &JotCard, _env: &_| item.label().to_string())
            .align_vertical(UnitPoint::CENTER)
            .padding(10.0)
            .expand()
            .height(50.0)
            .border(Color::rgb8(0, 0, 0), 2.0)
            .background(Painter::new(jot_card_background));
        label.on_click(|_event_ctx, jotcard, _env| jotcard.make_current())
    }))
    .vertical();

    let tagbox = Scroll::new(
        List::new(|| {
            Label::new(|item: &Tag, _env: &_| item.text().to_string())
                .with_line_break_mode(LineBreaking::WordWrap)
                .with_text_color(Color::grey8(222))
                .background(Color::BLACK)
                .align_horizontal(UnitPoint::CENTER)
                .align_vertical(UnitPoint::CENTER)
                .padding(5.0)
                .expand()
                .height(50.0)
                .width(150.0)
        })
        .horizontal(),
    )
    .horizontal()
    .lens(AppState::current_tags);

    let tags_and_rendered = Split::rows(tagbox, rendered).draggable(true);

    Split::columns(jotbox, tags_and_rendered).draggable(true)
}
