// We are going to define a module here to define the bezier curves that we are 
// going to use.
use iced::mouse; // We would like to be able to detect mouse movements.
use iced::widget::canvas::event::{self, Event}; // We can capture events in the canvas (clicks,
                                                // for example)
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke}; // We will be using a
                                                                         // number of structs
                                                                         // provided by the
                                                                         // Canvas module.
use iced::{Element, Fill, Point, Rectangle, Renderer, Theme}; // Some tools thatcan we 

// Here we define the state of the screen. It will cache the state of the screen.
#[derive(Default)]
pub struct State {
    cache: canvas::Cache,
}

// We will implement some methods for State. Reminder on lifetime annotation, 
// 'a is the lifetime that [Curve] must have.
impl State {
    // The view method, which accepts a self reference and a curves vector and an array.
    pub fn view<'a>(&'a self, curves: &'a [Curve]) -> Element<'a, Curve> {
        // Create a new Bezier curve. It will have a state of itself.
        Canvas::new(Bezier {
            state: self,
            curves,
        })
        .width(Fill)
            .height(Fill)
            .into()
    }

    // We request that we redraw the screen. This just clears the cache.
    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

// Stores Bezier curves. It has a state, as well as a curves array.
struct Bezier<'a> {
    state: &'a State,
    curves: &'a [Curve],
}

// A Program canvas.
impl<'a> canvas::Program<Curve> for Bezier<'a> {
    type State = Option<Pending>;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Curve>) {
        // Capture the cursor position. Ignore if not over box.
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, None);
        };

        // Detect an event.
        match event {
            // If there is a mouse event.
            Event::Mouse(mouse_event) => {
                // Message will be determined by event type.
                let message = match mouse_event {
                    // A mouse button was pressed. The left button specifically.
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match *state {
                            // If there is no state, set the value of the
                            // first point to the cursor position.
                            None => {
                                println!("Started Bezier creation.");
                                *state = Some(Pending::One {
                                    from: cursor_position,
                                });

                                // We create a pending but we do not return a message yet.
                                None
                            }
                            // If there is already one point, set the value
                            // of the second point.
                            Some(Pending::One { from }) => {
                                println!("Bezier destination chosen.");
                                *state = Some(Pending::Two {
                                    from,
                                    to: cursor_position,
                                });
                                
                                // We update the state of the Pending, but we don't actually return
                                // a message for Curve creation yet.
                                None
                            }
                           
                            // If there is a second point, capture the third point and make a
                            // curve.
                            Some(Pending::Two { from, to }) => {
                                *state = None; // Set state to none so you can draw another Bezier

                                println!("Bezier complete.");
                                Some(Curve {
                                    from,
                                    to,
                                    control: cursor_position,
                                })
                            }
                        }
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Right) => {
                        println!("You pressed the right mouse button!");
                        None
                    }
                    // Ignore anything else
                    _ => None,
                };

                (event::Status::Captured, message)
            },
            Event::Keyboard(_) => {
                println!("You pressed a key!");


                (event::Status::Captured, None) 
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {

        println!("Bezier draw.");
        // This creates content from cache for the drawing of a frame. The third argument is a lambda function
        // that accepts a frame and uses draw_all()
        let content =
            self.state.cache.draw(renderer, bounds.size(), |frame| {
                // Draws all curves on the frame. 
                Curve::draw_all(self.curves, frame, theme);

                //  Draws border currounding the drawing box.
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default()
                    .with_width(20.0)
                    .with_color(theme.palette().text),
                );
            });

        // Draw the pending curve, if there is one. Otherwise, just draw the current content.
        // The return value is decided here.
        if let Some(pending) = state {
            vec![content, pending.draw(renderer, theme, bounds, cursor)]
        } else {
            vec![content]
        }
    }

    // Control what the mouse cursor looks like.
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        print!("Mouse is over bounds.");
        if cursor.is_over(bounds) {
            println!(" Drawing crosshair.");
            mouse::Interaction::Crosshair
        } else {
            println!(" Drawing pointer.");
            mouse::Interaction::default()
        }
    }
}

// Three points which define a Bezier curve.
#[derive(Debug, Clone, Copy)]
pub struct Curve {
    from: Point, // The starting point
    to: Point, // The ending point
    control: Point, // The point that controls the curvature.
}

// We will implement on Curve several functions.
impl Curve {
    // This accepts a reference to the array of Curve vectors.
    // It will accept a frame, as well as a theme.
    fn draw_all(curves: &[Curve], frame: &mut Frame, theme: &Theme) {
        println!("Drawing all curves.");
        // We will define curves as Path with 
        let curves = Path::new(|p| {
            // This calculates the path of the curves.
            for curve in curves {
                p.move_to(curve.from);
                p.quadratic_curve_to(curve.control, curve.to);
            }
        });

        // This will draw a stroke for the curve on the canvas.
        frame.stroke(
            &curves,
            Stroke::default()
            .with_width(2.0)
            .with_color(theme.palette().text),
        );
    }
}

// A type which represents the intermediate phases of drawing a Bezier curve.
#[derive(Debug, Clone, Copy)]
enum Pending {
    One { from: Point },
    Two { from: Point, to: Point },
}

// Functions that Pending implements.
impl Pending {
    fn draw(
        &self,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Geometry {
        println!("Pending draw.");
        let mut frame = Frame::new(renderer, bounds.size());

        if let Some(cursor_position) = cursor.position_in(bounds) {
            match *self {
                Pending::One { from } => {
                    println!("Drawing a line.");
                    // Draw a line from the "from" point to the cursor position.
                    let line = Path::line(from, cursor_position);
                    frame.stroke(
                        &line,
                        Stroke::default()
                        .with_width(1.0)
                        .with_color(theme.palette().text),
                    );
                }
                Pending::Two { from, to } => {
                    println!("Drawing an in-situ curve.");
                    let curve = Curve {
                        from,
                        to,
                        control: cursor_position,
                    };
                    
                    // Draw this curve in this frame, using this theme.
                    Curve::draw_all(&[curve], &mut frame, theme);
                }
            };
        }

        frame.into_geometry()
    }
}
