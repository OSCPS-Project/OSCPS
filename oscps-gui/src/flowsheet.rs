use iced::mouse;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Rectangle, Renderer, Theme};

#[derive(Default)]
pub struct State {
    cache: canvas::Cache,
}

impl State {
    pub fn view<'a>(&'a self, curves: &'a [Curve], squares: &'a [Square]) -> Element<'a, Curve> {
        Canvas::new(Flowsheet {
            state: self,
            curves,
            squares,
        })
        .width(Fill)
        .height(Fill)
        .height(Fill)
        .into()
    }
    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

struct Flowsheet<'a> {
    state: &'a State,
    curves: &'a [Curve],
    squares: &'a [Square],
}

impl<'a> canvas::Program<Curve> for Flowsheet<'a> {
    type State = Option<Pending>;
    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Curve>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, None);
        };
        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match *state {
                            None => {
                                *state = Some(Pending::One {
                                    from: cursor_position,
                                });
                                None
                            }
                            Some(Pending::One { from }) => {
                                *state = Some(Pending::Two {
                                    from,
                                    to: cursor_position,
                                });
                                None
                            }
                            Some(Pending::Two { from, to }) => {
                                *state = None;
                                Some(Curve {
                                    from,
                                    to,
                                    control: cursor_position,
                                })
                            }
                        }
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Right) => {
                        None
                    }
                    _ => None,
                };
                (event::Status::Captured, message)
            },
            Event::Keyboard(_) => {
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
        let content =
            self.state.cache.draw(renderer, bounds.size(), |frame| {
                Curve::draw_all(self.curves, frame, theme);
                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default()
                    .with_width(20.0)
                    .with_color(theme.palette().text),
                );
            });
        if let Some(pending) = state {
            vec![content, pending.draw(renderer, theme, bounds, cursor)]
        } else {
            vec![content]
        }
    }
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

// impl<'a> canvas::Program<Square> for Flowsheet<'a> {
//     type State = Option<Pending>;
//     fn update(
//         &self,
//         state: &mut Self::State,
//         event: Event,
//         bounds: Rectangle,
//         cursor: mouse::Cursor,
//     ) -> (event::Status, Option<Curve>) {
//         let Some(cursor_position) = cursor.position_in(bounds) else {
//             return (event::Status::Ignored, None);
//         };
//         match event {
//             Event::Mouse(mouse_event) => {
//                 let message = match mouse_event {
//                     mouse::Event::ButtonPressed(mouse::Button::Left) => {
//                         match *state {
//                             None => {
//                                 *state = Some(Pending::One {
//                                     from: cursor_position,
//                                 });
//                                 None
//                             }
//                             Some(Pending::One { from }) => {
//                                 *state = Some(Pending::Two {
//                                     from,
//                                     to: cursor_position,
//                                 });
//                                 None
//                             }
//                             Some(Pending::Two { from, to }) => {
//                                 *state = None;
//                                 Some(Curve {
//                                     from,
//                                     to,
//                                     control: cursor_position,
//                                 })
//                             }
//                         }
//                     },
//                     mouse::Event::ButtonPressed(mouse::Button::Right) => {
//                         None
//                     }
//                     _ => None,
//                 };
//                 (event::Status::Captured, message)
//             },
//             Event::Keyboard(_) => {
//                 (event::Status::Captured, None)
//             }
//             _ => (event::Status::Ignored, None),
//         }
//     }

//     fn draw(
//         &self,
//         state: &Self::State,
//         renderer: &Renderer,
//         theme: &Theme,
//         bounds: Rectangle,
//         cursor: mouse::Cursor,
//     ) -> Vec<Geometry> {
//         // let content =
//         //     self.state.cache.draw(renderer, bounds.size(), |frame| {
//         //         Curve::draw_all(self.curves, frame, theme);
//         //         frame.stroke(
//         //             &Path::rectangle(Point::ORIGIN, frame.size()),
//         //             Stroke::default()
//         //             .with_width(20.0)
//         //             .with_color(theme.palette().text),
//         //         );
//         //     });
//         // if let Some(pending) = state {
//         //     vec![content, pending.draw(renderer, theme, bounds, cursor)]
//         // } else {
//         //     vec![content]
//         // }
//         vec![]
//     }

//     fn mouse_interaction(
//         &self,
//         _state: &Self::State,
//         bounds: Rectangle,
//         cursor: mouse::Cursor,
//     ) -> mouse::Interaction {
//         // if cursor.is_over(bounds) {
//         //     mouse::Interaction::Crosshair
//         // } else {
//             mouse::Interaction::default()
//         // }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Curve {
    from: Point,
    to: Point,
    control: Point,
}

impl Curve {
    fn draw_all(curves: &[Curve], frame: &mut Frame, theme: &Theme) {
        let curves = Path::new(|p| {
            for curve in curves {
                p.move_to(curve.from);
                p.quadratic_curve_to(curve.control, curve.to);
            }
        });

        frame.stroke(
            &curves,
            Stroke::default()
            .with_width(2.0)
            .with_color(theme.palette().text),
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    from: Point,
    to: Point,
    control: Point,
}

impl Square {
    fn draw_all(curves: &[Curve], frame: &mut Frame, theme: &Theme) {
        let curves = Path::new(|p| {
            for curve in curves {
                p.move_to(curve.from);
                p.quadratic_curve_to(curve.control, curve.to);
            }
        });

        frame.stroke(
            &curves,
            Stroke::default()
            .with_width(2.0)
            .with_color(theme.palette().text),
        );
    }
}

#[derive(Debug, Clone, Copy)]
enum Pending {
    One { from: Point },
    Two { from: Point, to: Point },
}

impl Pending {
    fn draw(
        &self,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Geometry {
        let mut frame = Frame::new(renderer, bounds.size());

        if let Some(cursor_position) = cursor.position_in(bounds) {
            match *self {
                Pending::One { from } => {
                    let line = Path::line(from, cursor_position);
                    frame.stroke(
                        &line,
                        Stroke::default()
                        .with_width(2.0)
                        .with_color(theme.palette().text),
                    );
                }
                Pending::Two { from, to } => {
                    let curve = Curve {
                        from,
                        to,
                        control: cursor_position,
                    };
                    
                    Curve::draw_all(&[curve], &mut frame, theme);
                }
            };
        }

        frame.into_geometry()
    }
}
