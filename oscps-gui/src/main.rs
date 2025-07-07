mod flowsheet;
mod style;

use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, horizontal_space, hover, responsive, text};
use iced::{Center, Element, Fill, Length, Theme};

use crate::icon::State;

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

#[derive(Debug, Clone, Copy)]
enum Message {
    AddedComponent(flowsheet::Component),
    Clear,
    PlaceComponent(flowsheet::Component),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
}

// These are the structures which make up the main window
struct MainWindow {
    // theme: Theme,
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
    flowsheet: flowsheet::State,
    components: Vec<flowsheet::Component>,
    state: icon::State,
    state_component: flowsheet::Component,
}

impl MainWindow {
    fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::new_selection());
        if let Some((_, split)) = panes.split(pane_grid::Axis::Vertical, pane, Pane::new_canvas()) {
            panes.resize(split, 0.2);
        }

        let state_component = flowsheet::Component::mixer();
        let state = State::new(state_component);

        MainWindow {
            // theme: Theme::default(),
            panes,
            focus: None,
            flowsheet: flowsheet::State::default(),
            components: Vec::default(),
            state,
            state_component,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddedComponent(component) => {
                info!("Added component");
                self.components.push(component);
                self.flowsheet.request_redraw();
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
        let col = column![
            view_content(self.state.view(target_mode).map(Message::AddedComponent)),
            text(target_mode.to_string())
        ];

        container(
            button(container(col))
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
                                .view(&self.components)
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
    use iced::mouse;
    use iced::widget::canvas::{self, Canvas, Stroke};
    use iced::{Element, Rectangle};
    use iced::{Fill, Renderer, Theme};

    pub struct State {
        // TODO: Find a simpler way to do icons.
        cache: canvas::Cache,
        component: flowsheet::Component,
    }

    impl State {
        pub fn new(component: flowsheet::Component) -> Self {
            State {
                cache: canvas::Cache::default(),
                component,
            }
        }

        pub fn view<'a>(
            &'a self,
            component: flowsheet::Component,
        ) -> Element<'a, flowsheet::Component> {
            Canvas::new(Icon {
                state: self,
                component,
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
    pub struct Icon<'a> {
        state: &'a State,
        component: flowsheet::Component,
    }

    impl<'a> canvas::Program<flowsheet::Component> for Icon<'a> {
        type State = Option<flowsheet::Component>;

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let content = self.state.cache.draw(renderer, bounds.size(), |frame| {
                let mut builder = canvas::path::Builder::new();

                match self.component {
                    flowsheet::Component::Source { .. } => flowsheet::Component::draw_source(
                        &mut builder,
                        (0.0, 0.0).into(),
                        (50.0, 50.0).into(),
                    ),
                    flowsheet::Component::Sink { .. } => flowsheet::Component::draw_sink(
                        &mut builder,
                        (0.0, 0.0).into(),
                        (0.0, 50.0).into(),
                    ),
                    flowsheet::Component::Mixer { .. } => flowsheet::Component::draw_mixer(
                        &mut builder,
                        (0.0, 0.0).into(),
                        (-50.0, 50.0).into(),
                        (50.0, 50.0).into(),
                    ),
                    flowsheet::Component::Connector { .. } => println!("Naw"),
                }

                let path = builder.build();

                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_width(10.0)
                        .with_color(theme.palette().text),
                );
            });

            vec![content]
        }
    }

    // impl<Message, Theme, Renderer> From<Icon> for Element<'_, Message, Theme, Renderer>
    // where
    //     Renderer: renderer::Renderer,
    // {
    //     fn from(icon: Icon) -> Self {
    //         Self::new(icon)
    //     }
    // }
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
