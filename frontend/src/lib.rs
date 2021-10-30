use std::array;

use zoon::*;
#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().update(|counter| counter + 1)
}

fn decrement() {
    counter().update(|counter| counter - 1)
}

fn topheader() -> RawHtmlEl {
    RawHtmlEl::new("nav")
        .attr("class", "navbar  navbar-expand-md")
        .children(array::IntoIter::new([
            RawHtmlEl::new("a")
                .attr("class", "navbar-brand")
                .attr("href", "#")
                .child("Admin Strip"),
            RawHtmlEl::new("div")
                .attr("class", "collapse navbar-collapse")
                .attr("id", "navbarsExampleDefault")
                .child(
                    RawHtmlEl::new("ul")
                        .class("nav")
                        .class("navbar-nav")
                        .class("navbar-right")
                        .children([
                            RawHtmlEl::new("li")
                                .attr("class", "nav-item active ")
                                .child(
                                    RawHtmlEl::new("a")
                                        .attr("class", "nav-link")
                                        .attr("href", "index.html")
                                        .child("Welcome to the Page"),
                                ),
                            RawHtmlEl::new("li")
                                .class("nav-item")
                                .child(
                                    RawHtmlEl::new("a")
                                        .attr("class", "nav-link")
                                        .attr("href", "pages.html")
                                        .child("Logout"),
                                )
                        ])
                ),
        ]))
}

fn root() -> impl Element {
    Column::new()
        .item(Button::new().label("-").on_press(decrement))
        .item(Text::with_signal(counter().signal()))
        .item(Button::new().label("+").on_press(increment))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app(None, topheader);
}
