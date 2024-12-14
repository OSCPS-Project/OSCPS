//! This example showcases an interactive `Canvas` for drawing Bézier curves.
mod bezier;
use iced::widget::{button, container, horizontal_space, hover};
use iced::{Element, Fill, Theme};

// Main function. Returns result from iced::application
pub fn main() -> iced::Result {
    // Sets window title, update method, and view method. Sets the theme, enables
    // antialiasing, centers the main window, and runs the program.
    iced::application("Bezier Tool - Iced", Example::update, Example::view)
        .theme(|_| Theme::Dark)
        .antialiasing(true)
        .centered()
        .run()
}

// Creates the Example struct, which has a bezier and and a vector of beziers.
// Presumably, the first bezier is the one actively being used
#[derive(Default)]
struct Example {
    bezier: bezier::State,
    curves: Vec<bezier::Curve>,
}

// Messages that are processed. The first is a click to add a curve, the second
// is to clear the screen.
#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(bezier::Curve),
    Clear,
}

// For example, we are going to implement the update and view functions.
impl Example {
    // This function accepts the Example struct, as well as a message.
    fn update(&mut self, message: Message) {
        // Match the message to one of two actions
        match message {
            // If it is add curve, we will push a curve to the curves struct.
            // The message will also contain the curve to add. We will then
            // redraw the screen.
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.bezier.request_redraw();
            }
            // If a clear message is sent, then the state of the bezier will
            // be set to default, and the bezier curve vector will be cleared.
            Message::Clear => {
                self.bezier = bezier::State::default();
                self.curves.clear();
            }
        }
    }
    // For this function, we are going to configure how the view is seen by the
    // user.
    fn view(&self) -> Element<Message> {
        // A container (https://docs.rs/iced/latest/iced/widget/container/index.html)
        // is essentially a box that allows a user to align the contents within. In 
        // this case, we are giving it a padding of 10 (presumably points) and will
        // aligh_right(Fill), which will align the container to the right. This should
        // not effect the appearance, as the container has even padding on all sides. 
        // Confirmed, there appears to be no effect.
        container(
            // Hover displays one widget on top of another one. This will be 
            // where the bezier curves are drawn. (I think)
            hover(
                // This askes the current bezier curve to display a view, and it 
                // will map it to the AddCurve message.
                self.bezier.view(&self.curves).map(Message::AddCurve),
                // If the curves vector is empty, just draw a container with a horizontal space
                // inside.
                if self.curves.is_empty() {
                    container(horizontal_space())
                } else {
                    // What we are doing to do is return a button which allows
                    // the user to clear the screen. In this case it will have the
                    // "danger" style, which makes it red. It will then clear the
                    // screen if it is pressed. It is again given  padding and aligned right. 
                    // In this case
                    container(
                        button("Clear")
                        .style(button::danger)
                        .on_press(Message::Clear),
                    )
                        .padding(10)
                        .align_bottom(Fill)
                },
            )
        )
        .padding(20)
        .into()
    }
}
