mod flowsheet;
mod style;

use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, horizontal_space, hover, responsive, text};
use iced::{Center, Element, Fill, Length, Theme};

use oscps_lib::simulation::{self, Settings, Simulation};

use icon::Icon;

use log::{debug, info};

pub fn main() -> iced::Result {
    // Start the GUI env_logger::init();
    info!("Starting application");

    let mut settings = iced::window::Settings::default();
    settings.size = (1920.0, 1080.0).into();
    settings.min_size = Some((480.0, 720.0).into());

    let application = iced::application(
        "Open Source Chemical Process Simulator",
        MainWindow::update,
        MainWindow::view,
    )
    .window(settings)
    .theme(|_| Theme::CatppuccinMocha)
    .antialiasing(true)
    .centered();

    application.run()
}

// These are the structures which make up the main window
#[allow(dead_code)]
struct MainWindow {
    // theme: Theme,
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
    flowsheet: flowsheet::State,
    components: Vec<flowsheet::Component>,
    simulation: Simulation,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddedComponent(flowsheet::Component),
    Clear,
    PlaceComponent(flowsheet::Component),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
}

impl MainWindow {
    fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::new_selection());
        if let Some((_, split)) = panes.split(pane_grid::Axis::Vertical, pane, Pane::new_canvas()) {
            panes.resize(split, 0.2);
        }

        let settings = Settings::default();

        MainWindow {
            // theme: Theme::default(),
            panes,
            focus: None,
            flowsheet: flowsheet::State::default(),
            components: Vec::default(),
            simulation: Simulation::new(settings),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddedComponent(component) => {
                info!("Added component");
                self.components.push(component);
                self.flowsheet.request_redraw();
                match component {
                    flowsheet::Component::Source{ ..}  => todo!(),
                    flowsheet::Component::Sink{ ..} => todo!(),
                    flowsheet::Component::Mixer{ ..} => {
                        self.simulation.add_block(simulation::BlockType::Mixer);
                    },
                    flowsheet::Component::Connector{ .. } => { 
                        // self.simulation.add_stream(simulation::BlockType::Mixer);
                        todo!();
                    },
                }
            }
            // TODO: Make the clear option more deliberate (2 clicks at least)
            Message::Clear => {
                self.flowsheet = flowsheet::State::default();
                self.components.clear();
            }
            // Default placement mode should be 'None'
            Message::PlaceComponent(component) => {
                match component {
                    // TODO: Modify to do more work other than a simple assignment.
                    flowsheet::Component::Connector { .. } => {
                        info!("Setting to connector placement mode.");
                        self.flowsheet.placement_mode = flowsheet::Component::connector();
                    }
                    flowsheet::Component::Mixer { .. } => {
                        info!("Setting to mixer placement mode.");
                        self.flowsheet.placement_mode = flowsheet::Component::mixer();
                    }
                    flowsheet::Component::Source { .. } => {
                        info!("Setting to source placement mode.");
                        self.flowsheet.placement_mode = flowsheet::Component::source();
                    }
                    flowsheet::Component::Sink { .. } => {
                        info!("Setting to sink placement mode.");
                        self.flowsheet.placement_mode = flowsheet::Component::sink();
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
                info!("You clicked on a pane!")
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
                info!("You dragged a pane!")
            }
            Message::Dragged(_) => {
                info!("You dragged, but did not drop a pane!")
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
                info!("You resized a pane!")
            }
        }
    }

    // Create a button to add a certain component
    fn placement_button<'a>(
        &'a self,
        target_mode: flowsheet::Component,
    ) -> impl Into<Element<'a, Message>> {
        container(
            button(container(column![
                Icon::new(target_mode),
                text(target_mode.to_string())
            ]))
            .style(match self.flowsheet.placement_mode {
                mode if mode == target_mode => button::danger,
                _ => button::secondary,
            })
            .on_press(Message::PlaceComponent(target_mode)),
        )
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = focus == Some(id);
            match pane {
                Pane::ComponentSelection => {
                    debug!("Found Selection!");
                    return column![
                        container(text("Component Selection"))
                            .padding(5)
                            .width(Length::Fill)
                            .style(if is_focused {
                                style::title_bar_focused
                            } else {
                                style::title_bar_active
                            }),
                        self.placement_button(flowsheet::Component::source()).into(),
                        self.placement_button(flowsheet::Component::sink()).into(),
                        self.placement_button(flowsheet::Component::connector())
                            .into(),
                        self.placement_button(flowsheet::Component::mixer()).into(),
                    ]
                    .width(Length::Fill)
                    .into();
                }
                Pane::Canvas => {
                    debug!("Found canvas!");

                    let flowsheet_title_bar = pane_grid::TitleBar::new("Flowsheet")
                        .padding(10)
                        .style(if is_focused {
                            style::title_bar_focused
                        } else {
                            style::title_bar_active
                        });

                    pane_grid::Content::new(responsive(move |_size| {
                        view_content(hover(
                            self.flowsheet
                                .view(&self.components, &self.simulation)
                                .map(Message::AddedComponent),
                            if self.components.is_empty() {
                                container(horizontal_space())
                            } else {
                                container(
                                    button("Clear")
                                        .style(button::danger)
                                        .on_press(Message::Clear),
                                )
                                .padding(10)
                                .align_top(Fill)
                            },
                        ))
                    }))
                    .title_bar(flowsheet_title_bar)
                    .style(if is_focused {
                        style::pane_focused
                    } else {
                        style::pane_active
                    })
                }
            }
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        container(column![pane_grid,]).padding(20).into()
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow::new()
    }
}

mod icon {
    use crate::flowsheet;
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::border;
    use iced::mouse;
    use iced::{Color, Element, Length, Rectangle, Size};

    pub struct Icon {
        // component: flowsheet::Component,
    }

    impl Icon {
        pub fn new(_component: flowsheet::Component) -> Self {
            Self { 
                // component
            }
        }
    }

    #[allow(dead_code)]
    pub fn icon(component: flowsheet::Component) -> Icon {
        Icon::new(component)
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Icon
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Shrink,
                height: Length::Shrink,
            }
        }

        fn layout(
            &self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            let hard_size = 100.0; // HACK: Temporary, figure out a more elegant solution later.
            layout::Node::new(Size::new(hard_size, hard_size))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            let hard_size = 50.0; // HACK: Again, temporary

            // TODO: Placeholder for when custom widgets have better support.

            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: border::rounded(hard_size),
                    ..renderer::Quad::default()
                },
                Color::BLACK,
            );
        }
    }
    impl<Message, Theme, Renderer> From<Icon> for Element<'_, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(icon: Icon) -> Self {
            Self::new(icon)
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Pane {
    Canvas,

    #[default]
    ComponentSelection,
}

impl Pane {
    fn new_selection() -> Self {
        Pane::ComponentSelection
    }
    fn new_canvas() -> Self {
        Pane::Canvas
    }
}

fn view_content<'a>(flowsheet: Element<'a, Message>) -> Element<'a, Message> {
    let content = column![flowsheet] // controls,
        .spacing(10)
        .align_x(Center);

    container(content).center_y(Fill).padding(5).into()
}
