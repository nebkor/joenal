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
use joenal::{get_config, get_jots, gui::*, make_pool};

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
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .border(Color::rgb8(0, 0, 0), 2.0)
            .background(Painter::new(jot_card_background));
        label.on_click(|_event_ctx, data, _env| data.make_current())
    }))
    .vertical();

    Split::columns(jotbox, rendered).draggable(true)
}
