use gtk::*;
pub struct Header {
    pub container: HeaderBar,
    //pub hello_btn: Button,

}
impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_show_close_button(true);
        container.set_title("game of life");
        let hello_btn = Button::new_with_label("Test!");
        let other_button = Button::new_with_label("Test!");
        container.pack_start(&hello_btn);
        container.pack_start(&other_button);
        hello_btn.get_style_context().map(|c| c.add_class("hello-btn"));
        other_button.get_style_context().map(|c| c.add_class("hello-btn"));

Header {
            container,
            //hello_btn,       
        }
    }
}