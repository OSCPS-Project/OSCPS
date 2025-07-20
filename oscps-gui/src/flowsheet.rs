use iced::mouse;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Rectangle, Renderer, Theme};
use oscps_lib::simulation::{BlockReference, Simulation};
use std::sync::Arc;

use std::time::{Duration, SystemTime};

use log::{debug, info, warn};
use strum_macros::Display;

#[derive(Default)]
pub struct State {
    cache: canvas::Cache,
    pub placement_mode: Component,
}

impl State {
    pub fn view<'a>(
        &'a self,
        components: &'a [Component],
        simulation: &'a Simulation,
    ) -> Element<'a, Component> {
        Canvas::new(Flowsheet {
            state: self,
            components,
            left_click_time: SystemTime::now(),
            simulation,
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

#[derive(Display, Debug, Clone)]
pub enum Component {
    Connector {
        from: Option<Point>,
        to: Option<Point>,
        from_block: Option<BlockReference>,
        to_block: Option<BlockReference>,
    },
    Mixer {
        at: Option<Point>,
        input: Option<Point>,
        output: Option<Point>,
        block: Option<BlockReference>,
    },
    Source {
        at: Option<Point>,
        output: Option<Point>,
        block: Option<BlockReference>,
    },
    Sink {
        at: Option<Point>,
        input: Option<Point>,
        block: Option<BlockReference>,
    },
}
fn block_refs_equal(block1: &Option<BlockReference>, block2: &Option<BlockReference>) -> bool {
    match (block1, block2) {
        (None, None) => true,
        (Some(b1), Some(b2)) => Arc::ptr_eq(b1, b2),
        _ => false,
    }
}
impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Component::Connector {
                    from: from1,
                    to: to1,
                    from_block: from_block1,
                    to_block: to_block1,
                },
                Component::Connector {
                    from: from2,
                    to: to2,
                    from_block: from_block2,
                    to_block: to_block2,
                },
            ) => {
                from1 == from2
                    && to1 == to2
                    && block_refs_equal(from_block1, from_block2)
                    && block_refs_equal(to_block1, to_block2)
            }
            (
                Component::Mixer {
                    at: at1,
                    input: input1,
                    output: output1,
                    block: block1,
                },
                Component::Mixer {
                    at: at2,
                    input: input2,
                    output: output2,
                    block: block2,
                },
            ) => {
                at1 == at2
                    && input1 == input2
                    && output1 == output2
                    && block_refs_equal(block1, block2)
            }
            (
                Component::Source {
                    at: at1,
                    output: output1,
                    block: block1,
                },
                Component::Source {
                    at: at2,
                    output: output2,
                    block: block2,
                },
            ) => at1 == at2 && output1 == output2 && block_refs_equal(block1, block2),
            (
                Component::Sink {
                    at: at1,
                    input: input1,
                    block: block1,
                },
                Component::Sink {
                    at: at2,
                    input: input2,
                    block: block2,
                },
            ) => at1 == at2 && input1 == input2 && block_refs_equal(block1, block2),
            _ => false,
        }
    }
}

impl Component {
    pub fn connector() -> Self {
        Component::Connector {
            from: None,
            to: None,
            from_block: None,
            to_block: None,
        }
    }

    pub fn source() -> Self {
        Component::Source {
            at: None,
            output: None,
            block: None,
        }
    }
    pub fn sink() -> Self {
        Component::Sink {
            at: None,
            input: None,
            block: None,
        }
    }
    pub fn mixer() -> Self {
        Component::Mixer {
            at: None,
            input: None,
            output: None,
            block: None,
        }
    }

    // Determine if cursor is within 5 pixels of a given point.
    fn is_in_bounds(&self, cursor_position: Point, input: Point) -> bool {
        info!(
            "Checking input bounds with cursor at ({}, {})",
            cursor_position.x, cursor_position.y
        );

        // TODO: (minor) Fix arbitrary 5-pixel bounding box. Make dynamic/program setting.
        let bound = 5.0;
        if cursor_position.x > input.x - bound && cursor_position.x < input.x + bound {
            debug!("Bound x match!");
            if cursor_position.y > input.y - bound && cursor_position.y < input.y + bound {
                info!("Bounds match!");
                return true;
            }
        }
        false
    }

    // Determine if the cursor is in bounds of the input
    fn on_input(&self, cursor_position: Point) -> bool {
        let input = self.get_input();
        match input {
            Some(point) => self.is_in_bounds(cursor_position, point),
            None => false,
        }
    }

    // Determine if the cursor is in bounds of the output
    fn get_input(&self) -> Option<Point> {
        return match self {
            Component::Connector { from, .. } => *from,
            Component::Mixer { input, .. } => *input,
            Component::Sink { input, .. } => *input,
            Component::Source { .. } => None, // Source does not have an input
        };
    }

    fn on_output(&self, cursor_position: Point) -> bool {
        let output = self.get_output();
        match output {
            Some(point) => self.is_in_bounds(cursor_position, point),
            None => false,
        }
    }

    fn get_output(&self) -> Option<Point> {
        return match self {
            Component::Connector { to, .. } => *to,
            Component::Mixer { output, .. } => *output,
            Component::Source { output, .. } => *output,
            Component::Sink { .. } => None, // Sink does not have an output
        };
    }

    fn draw_all(components: &[Component], frame: &mut Frame, theme: &Theme) {
        // TODO: (minor) Nitpicky, but this uses dynamic memory uncessesarily.
        // Consider changing the function name fetching to a macro approach.
        let function_name = std::any::type_name::<fn()>()
            .split("::")
            .last()
            .unwrap_or("unknown");
        let expect_string = format!(
            "{} should should only be called with existing points.",
            function_name
        );
        let components = Path::new(|p| {
            for component in components {
                match component {
                    Component::Connector { from, to, .. } => {
                        let from = from.expect(&expect_string);
                        let to = to.expect(&expect_string);

                        Component::draw_connector(p, to, from)
                    }
                    Component::Mixer {
                        at, input, output, ..
                    } => {
                        let at = at.expect(&expect_string);
                        let input = input.expect(&expect_string);
                        let output = output.expect(&expect_string);

                        Component::draw_mixer(p, at, input, output)
                    }
                    Component::Source { at, output, .. } => {
                        let at = at.expect(&expect_string);
                        let output = output.expect(&expect_string);

                        Component::draw_source(p, at, output)
                    }
                    Component::Sink { at, input, .. } => {
                        let at = at.expect(&expect_string);
                        let input = input.expect(&expect_string);

                        Component::draw_sink(p, at, input)
                    }
                }
            }
        });

        frame.stroke(
            &components,
            Stroke::default()
                .with_width(2.0)
                .with_color(theme.palette().text),
        );
    }

    pub fn draw_connector(p: &mut Builder, to: Point, from: Point) {
        debug!("Drawing connector");
        p.move_to(from);
        let half_x_coord = from.x + (to.x - from.x) / 2.0;
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

    pub fn draw_mixer(p: &mut Builder, at: Point, input: Point, output: Point) {
        debug!("Drawing mixer.");

        p.move_to(at);
        let bottom_point = Point::new(at.x, at.y + 100.0);
        let middle_point = Point::new(at.x + 100.0, at.y + 50.0);
        p.line_to(bottom_point);
        p.line_to(middle_point);
        p.line_to(at);

        // Draw a circle for input connectors
        p.move_to(at);
        p.circle(input, 5.0);
        // Another circle for output connectors
        p.move_to(at);
        p.circle(output, 5.0);
    }

    pub fn draw_source(p: &mut Builder, at: Point, output: Point) {
        debug!("Drawing source.");

        p.move_to(at);
        p.rectangle(at, (50.0, 100.0).into());

        // Circle for output
        p.circle(output, 5.0);
    }

    pub fn draw_sink(p: &mut Builder, at: Point, input: Point) {
        debug!("Drawing sink.");

        p.move_to(at);
        p.rectangle(at, (50.0, 100.0).into());

        // Circle for input
        p.move_to(at);
        p.circle(input, 5.0);
    }
}

// Declare the default block to be the humble connector.
impl Default for Component {
    fn default() -> Self {
        Component::Connector {
            from: None,
            to: None,
            from_block: None,
            to_block: None,
        }
    }
}

struct Flowsheet<'a> {
    state: &'a State,
    components: &'a [Component],
    left_click_time: SystemTime,
    simulation: &'a Simulation,
}

impl<'a> Flowsheet<'a> {
    fn place_sink(cursor_position: Point) -> Option<Component> {
        let input = Point::new(cursor_position.x - 5.0, cursor_position.y + 50.0);
        info!(
            "Creating source at ({}, {}) with input at ({}, {}).",
            cursor_position.x, cursor_position.y, input.x, input.y
        );
        Some(Component::Sink {
            at: Some(cursor_position),
            input: Some(input),
            block: None,
        })
    }

    fn place_source(cursor_position: Point) -> Option<Component> {
        let output = Point::new(cursor_position.x + 55.0, cursor_position.y + 50.0);
        info!(
            "Creating source at ({}, {}) with output at ({}, {}).",
            cursor_position.x, cursor_position.y, output.x, output.y
        );
        Some(Component::Source {
            at: Some(cursor_position),
            output: Some(output),
            block: None,
        })
    }

    // Helper function to place a mixer block
    fn place_mixer(cursor_position: Point) -> Option<Component> {
        let input = Point::new(cursor_position.x - 5.0, cursor_position.y + 50.0);
        let output = Point::new(cursor_position.x + 105.0, cursor_position.y + 50.0);
        info!(
            "Creating mixer at ({}, {}) with input at ({}, {}) and output at ({}, {})",
            cursor_position.x, cursor_position.y, input.x, input.y, output.x, output.y
        );
        Some(Component::Mixer {
            at: Some(cursor_position),
            input: Some(input),
            output: Some(output),
            block: None,
        })
    }

    // Helper function to connect a connector to an input/output.
    fn place_connector(
        &self,
        state: &mut Option<Pending>,
        cursor_position: Point,
    ) -> Option<Component> {
        let floating_connectors = false; // HACK: Disallow floating connectors
        match state {
            None => {
                info!("Beginning creation of connector...");
                let mut result = Some(Pending::One {
                    from: cursor_position,
                });

                for component in self.components {
                    if !matches!(component, Component::Connector { .. })
                        && component.on_output(cursor_position)
                    {
                        info!("Connected to input!");
                        result = Some(Pending::One {
                            // NOTE: Should be safe. This must be Some(..) if
                            // on_output returned true.
                            from: component.get_output().unwrap(),
                        });
                        *state = result;
                        return None;
                    }
                }
                if floating_connectors {
                    *state = result;
                }
                None
            }
            Some(Pending::One { from }) => {
                info!("Created connector.");
                let from = *from;
                let mut result = Some(Component::Connector {
                    from: Some(from),
                    to: Some(cursor_position),
                    from_block: None, // TODO: Implement properly
                    to_block: None,
                });
                for component in self.components {
                    if !matches!(component, Component::Connector { .. })
                        && component.on_input(cursor_position)
                    {
                        info!("Connected to input!");
                        result = Some(Component::Connector {
                            from: Some(from),
                            // NOTE: Should be safe, on_input() returned true.
                            to: Some(component.get_input().unwrap()),
                            from_block: None,
                            to_block: None, // TODO: Implement properly
                        });
                        *state = None;
                        return result;
                    }
                }
                if floating_connectors {
                    *state = None;
                    result
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> canvas::Program<Component> for Flowsheet<'a> {
    type State = Option<Pending>;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Component>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, None);
        };
        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        info!("Click detected at ({})", cursor_position);
                        let current_time = SystemTime::now();

                        match current_time.duration_since(self.left_click_time) {
                            Ok(elapsed) => {
                                if elapsed < Duration::from_millis(200) {
                                    println!("Double click!")
                                }
                            }
                            Err(e) => {
                                warn!("Error {} when detecting double click.", e)
                            }
                        }

                        match self.state.placement_mode {
                            Component::Connector { .. } => {
                                Flowsheet::place_connector(&self, state, cursor_position)
                            }
                            Component::Mixer { .. } => {
                                self.simulation;
                                Flowsheet::place_mixer(cursor_position)
                            }
                            Component::Source { .. } => Flowsheet::place_source(cursor_position),
                            Component::Sink { .. } => Flowsheet::place_sink(cursor_position),
                        }
                    }
                    // Right click should cancel placement.
                    mouse::Event::ButtonPressed(mouse::Button::Right) => {
                        info!("Right mouse button clicked");
                        *state = None;
                        None
                    }
                    _ => None,
                };

                (event::Status::Captured, message)
            }
            Event::Keyboard(_) => (event::Status::Captured, None),
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
        let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
            Component::draw_all(self.components, frame, theme);
            // Border frame
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default()
                    .with_width(10.0)
                    .with_color(theme.palette().text),
            );
        });
        if let Some(pending) = state {
            vec![content, pending.draw(renderer, theme, bounds, cursor)] // Connector being drawn
        } else {
            vec![content] // Just draw current content.
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return mouse::Interaction::default();
        };

        // Only display a grab icon if placing a connector, and
        // the connector is hovering over an input when in input mode, or over
        // an output when in output mode.
        if cursor.is_over(bounds) {
            match self.state.placement_mode {
                Component::Connector { .. } => {
                    for component in self.components {
                        match component {
                            Component::Connector { .. } => (),
                            _ => match state {
                                Some(Pending::One { .. }) => {
                                    if component.on_input(cursor_position) {
                                        println!("Some");
                                        return mouse::Interaction::Grab;
                                    }
                                }
                                None => {
                                    if component.on_output(cursor_position) {
                                        println!("Component: {}", component);
                                        println!("None");
                                        return mouse::Interaction::Grab;
                                    }
                                }
                            },
                        }
                    }
                    mouse::Interaction::Crosshair
                }
                _ => mouse::Interaction::Crosshair,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Pending {
    One { from: Point },
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
                        let half_x_coord = from.x + (to.x - from.x) / 2.0;
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
