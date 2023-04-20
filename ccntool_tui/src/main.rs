use ccntool_core::{connectdb, myquery};

use cursive::theme::{BorderStyle, Palette};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, TextView};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                // First, override some colors from the base palette.
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }
            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
            }
        }),
    });

    siv.add_layer(
        Dialog::new()
            .title("Enter a port description")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(show_popup)
                    .with_name("description")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                let description = s
                    .call_on_name("description", |view: &mut EditView| view.get_content())
                    .unwrap();

                show_popup(s, &description);
            }),
    );

    siv.run();
}

fn show_popup(s: &mut Cursive, description: &str) {
    if description.is_empty() {
        s.add_layer(Dialog::info("Please enter a valid port description!"));
    } else {
        let conn =
            connectdb(Option::None, Option::None, Option::None).expect("Can't connect to database");
        let results = match myquery(conn, description) {
            Ok(rows) => rows,
            Err(error) => {
                eprintln!("Error: {error}");
                return;
            }
        };

        let content = format!(
            "Switchname: {}
IP: {}
Switchport: {}
Beschreibung: {}",
            results[0], results[3], results[2], results[1]
        );
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
    }
}
