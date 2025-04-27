use iced::mouse;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Rectangle, Renderer, Theme};

use log::{info, debug};

#[derive(Default)]
pub struct State {
    cache: canvas::Cache,
    pub placement_mode: BlockPlacement,
}

impl State {
    pub fn view<'a>(&'a self, curves: &'a [Curve]) -> Element<'a, Curve> {
        Canvas::new(Flowsheet {
            state: self,
            curves,
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

// Allows detection of "block placement mode", as well as which block to place.
#[derive(Debug, Clone, Copy)]
pub enum BlockPlacement {
    Connector,
    Mixer,
    Default,
}

impl Default for BlockPlacement {
    fn default() -> Self {
        BlockPlacement::Default
    }
}

struct Flowsheet<'a> {
    state: &'a State,
    curves: &'a [Curve],
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
                        info!("Click detected at ({})", cursor_position);
                        match self.state.placement_mode {
                            BlockPlacement::Connector => {
                                match *state {
                                    None => {
                                        info!("Beginning creation of connector...");
                                        let mut result = Some(Pending::One {
                                            from: cursor_position,
                                        });
                                        for curve in self.curves {
                                            if !matches!(curve, Curve::Connector{..}) && curve.on_output_connector(cursor_position) {
                                                info!("Connected to input!");
                                                result = Some(Pending::One {
                                                    from: curve.get_output_point()
                                                });
                                                break;
                                            }
                                        }
                                        *state = result;
                                        None
                                    }
                                    Some(Pending::One { from }) => {
                                        info!("Created connector.");
                                        *state = None;
                                        let mut result = Some(Curve::Connector {
                                                    from,
                                                    to: cursor_position
                                                });
                                        for curve in self.curves {
                                            if !matches!(curve, Curve::Connector{..}) && curve.on_input_connector(cursor_position) {
                                                info!("Connected to input!");
                                                result = Some(Curve::Connector {
                                                    from,
                                                    to: curve.get_input_point()
                                                });
                                                break;
                                            }
                                        }
                                        result
                                    }
                                    // Some(Pending::Two { from, to }) => {
                                    //     *state = None;
                                    //     Some(Curve::Connector {
                                    //         from,
                                    //         to,
                                    //     })
                                    // }
                                }
                            },
                            BlockPlacement::Mixer => {        
                                let input_point = Point::new(cursor_position.x - 5.0, cursor_position.y + 50.0);
                                let output_point = Point::new(cursor_position.x + 105.0, cursor_position.y + 50.0);
                                info!("Creating mixer at ({}, {}) with input at ({}, {}) and output at ({}, {})", cursor_position.x, cursor_position.y, input_point.x, input_point.y, output_point.x, output_point.y);
                                Some(Curve::Mixer {
                                    at: cursor_position,
                                    input_point,
                                    output_point,
                                })
                            },
                            BlockPlacement::Default => {
                                // TODO: Add code for selecting stuff
                                None
                            }
                        }

                    },
                    // Right click should cancel placement.
                    mouse::Event::ButtonPressed(mouse::Button::Right) => {
                        info!("Right mouse button clicked");
                        *state = None; 
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

#[derive(Debug, Clone, Copy)]
pub enum Curve {
    Connector {
        from: Point,
        to: Point,
    },
    Mixer {
        at: Point,
        input_point: Point,
        output_point: Point,
    }
}

impl Curve {
    fn on_input_connector(
        &self, 
        cursor_position: Point
        ) -> bool { 
        // TODO: Fix arbitrary 5-pixel bounding box. Ideally use a circular bound.
        let input = self.get_input_point();
        info!("Checking input bounds with cursor at ({}, {})", cursor_position.x, cursor_position.y);
        if cursor_position.x > input.x - 5.0 && cursor_position.x < input.x + 5.0 {
            debug!("Bound x match!");
            if cursor_position.y > input.y - 5.0 && cursor_position.y < input.y + 5.0 {
                info!("Bounds match!");
                return true 
            }
        }
        false
    }

    fn get_input_point(&self) -> Point {
        return match self {
            Curve::Connector{from, ..} => {
                *from
            },
            Curve::Mixer{input_point, ..} => {
                *input_point
            },
        }
    }

    fn on_output_connector(
        &self, 
        cursor_position: Point
        ) -> bool { 
        let output = self.get_output_point();
        info!("Checking output bounds with cursor at ({}, {})", cursor_position.x, cursor_position.y);
        if cursor_position.x > output.x - 5.0 && cursor_position.x < output.x + 5.0 {
            debug!("Bound x match!");
            if cursor_position.y > output.y - 5.0 && cursor_position.y < output.y + 5.0 {
                info!("Bounds match!");
                return true 
            }
        }
        false
    }

    fn get_output_point(&self) -> Point {
        return match self {
            Curve::Connector{to, ..} => {
                *to
            },
            Curve::Mixer{output_point, ..} => {
                *output_point
            },
        }
    }
    fn draw_all(curves: &[Curve], frame: &mut Frame, theme: &Theme) {
        let curves = Path::new(|p| {
            for curve in curves {
                match curve {
                    Curve::Connector{ from, to } => {
                        debug!("Drawing connector");
                        p.move_to(*from);
                        // p.quadratic_curve_to(*control, *to);
                        let half_x_coord = from.x + (to.x - from.x)/2.0; 
                        p.line_to(Point::new(half_x_coord, from.y));
                        p.line_to(Point::new(half_x_coord, to.y));
                        p.line_to(Point::new(to.x, to.y));
                        let mut arrow_offset_x = -10.0;
                        let arrow_offset_y = 5.0; 
                        if to.x < from.x {
                            arrow_offset_x *= -1.0; 
                        }                        
                        p.line_to(Point::new(to.x + arrow_offset_x, to.y + arrow_offset_y));
                        p.line_to(Point::new(to.x + arrow_offset_x, to.y - arrow_offset_y));
                        p.line_to(Point::new(to.x, to.y));
                    }
                    Curve::Mixer{at, input_point, output_point} => {
                        debug!("Drawing mixer.");
                        p.move_to(*at); 
                        // p.rectangle(*at, Size::new(200.0, 200.0));
                        let bottom_point = Point::new(at.x, at.y + 100.0);
                        let middle_point = Point::new(at.x + 100.0, at.y + 50.0);
                        p.line_to(bottom_point);
                        p.line_to(middle_point);
                        p.line_to(*at);
                        // Draw a circle for input connectors 
                        p.move_to(*at);
                        p.circle(*input_point, 5.0);
                        // Another circle for output connectors
                        p.move_to(*at);
                        p.circle(*output_point, 5.0);
                    }
                }
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
    // Two { from: Point, to: Point },
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
                    let to = cursor_position;
                    let line = Path::new(|p| {
                        p.move_to(from);
                        // p.quadratic_curve_to(*control, *to);
                        let half_x_coord = from.x + (to.x - from.x)/2.0; 
                        p.line_to(Point::new(half_x_coord, from.y));
                        p.line_to(Point::new(half_x_coord, to.y));
                        p.line_to(Point::new(to.x, to.y));

                        let mut arrow_offset_x = -10.0;
                        let arrow_offset_y = 5.0; 
                        if to.x < from.x {
                            arrow_offset_x *= -1.0; 
                        }                        
                        p.line_to(Point::new(to.x + arrow_offset_x, to.y + arrow_offset_y));
                        p.line_to(Point::new(to.x + arrow_offset_x, to.y - arrow_offset_y));
                        p.line_to(Point::new(to.x, to.y));
                    });
                    frame.stroke(
                        &line,
                        Stroke::default()
                        .with_width(2.0)
                        .with_color(theme.palette().text),
                    );
                }
            };
        }

        frame.into_geometry()
    }
}
