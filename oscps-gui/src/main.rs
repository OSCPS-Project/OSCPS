mod flowsheet;
mod style;

use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, column, container, horizontal_space, hover, responsive, row, text};
use iced::{Center, Element, Fill, Size, Theme};

use log::{info, debug};

pub fn main() -> iced::Result {
    // Start the GUI
    env_logger::init();
    info!("Starting application");
    iced::application("Open Source Chemical Process Simulator", MainWindow::update, MainWindow::view)
        .theme(|_| Theme::CatppuccinMocha)
        .antialiasing(true)
        .centered()
        .run()
}

struct MainWindow {
    // theme: Theme,
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
    flowsheet: flowsheet::State,
    curves: Vec<flowsheet::Curve>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(flowsheet::Curve),
    Clear,
    PlaceComponent(flowsheet::BlockPlacement),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
}

impl MainWindow {

    fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::new_canvas());
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::new_selection());

        MainWindow {
            // theme: Theme::default(),
            panes,
            focus: None,
            flowsheet: flowsheet::State::default(),
            curves: Vec::default(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddCurve(curve) => {
                info!("Adding curve");
                self.curves.push(curve);
                self.flowsheet.request_redraw();
            }
            Message::Clear => {
                self.flowsheet = flowsheet::State::default();
                self.curves.clear();
            }
            // Default placement mode should be 'None'
            Message::PlaceComponent(component) => {
                match component { // TODO: Modify to do more work other than a simple assignment.
                    flowsheet::BlockPlacement::Default => {
                        info!("Setting to default placement mode.");
                        self.flowsheet.placement_mode = flowsheet::BlockPlacement::default();
                    },
                    flowsheet::BlockPlacement::Connector => {
                        info!("Setting to connector placement mode.");
                        self.flowsheet.placement_mode = flowsheet::BlockPlacement::Connector;
                    },
                    flowsheet::BlockPlacement::Mixer => {
                        info!("Setting to mixer placement mode.");
                        self.flowsheet.placement_mode = flowsheet::BlockPlacement::Mixer;
                    },
                }
            },
            Message::Clicked(pane) => {
                self.focus = Some(pane);
                info!("You clicked on a pane!")
                },
            Message::Dragged(pane_grid::DragEvent::Dropped{ pane, target }) => { // pane, target
                self.panes.drop(pane, target);
                println!("You dragged a pane!")
            },
            Message::Dragged(_) => {
                println!("You dragged, but did not drop a pane!")
            },
            Message::Resized(pane_grid::ResizeEvent { split, ratio } ) => {
                self.panes.resize(split, ratio);
                println!("You resized a pane!")
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
        match pane {
            Pane::Canvas
            // { id: _, is_pinned: _}
            => {
                debug!("Found canvas!"); 
            }
            Pane::UnitSelection => {
                debug!("Found Selection!");
                return row![
                    container(
                        button("Place Connector")
                        .style(
                            match self.flowsheet.placement_mode {
                                flowsheet::BlockPlacement::Connector => button::danger,
                                _ => button::secondary,
                            }
                        )
                        .on_press( 
                            match self.flowsheet.placement_mode {
                                flowsheet::BlockPlacement::Connector => Message::PlaceComponent(flowsheet::BlockPlacement::Default),
                                _ => Message::PlaceComponent(flowsheet::BlockPlacement::Connector) 
                            }
                        )
                    ),
                    container(
                        button("Place Mixer")
                        .style(
                            match self.flowsheet.placement_mode {
                                flowsheet::BlockPlacement::Mixer => button::danger,
                                _ => button::secondary,
                            }
                        )
                        .on_press( 
                            match self.flowsheet.placement_mode {
                                flowsheet::BlockPlacement::Mixer => Message::PlaceComponent(flowsheet::BlockPlacement::Default),
                                _ => Message::PlaceComponent(flowsheet::BlockPlacement::Mixer) 
                            }
                        )
                    ),
                ].into()
            }
        }
        let is_focused = focus == Some(id);

        let title = row![
            "Flowsheet",
        ]
            .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                .padding(10)
                .style(if is_focused {
                    style::title_bar_focused
                } else {
                    style::title_bar_active
                });

            pane_grid::Content::new(responsive(move |size| {
                view_content(
                id, 
                total_panes, 
                false,
                size,
            hover(
                self.flowsheet.view(&self.curves).map(Message::AddCurve),
                if self.curves.is_empty() {
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
            ),

                )
        }))
        .title_bar(title_bar)
        .style(if is_focused {
            style::pane_focused
        } else {
            style::pane_active
        })
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        container(
            column![
            pane_grid,
            ]
        )
        .padding(20)
        .into()
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow::new()
    }
}

#[derive(Clone,Copy,Default)]
enum Pane {
    Canvas
    // {
    //     id: usize,
    //     is_pinned: bool,
    // }
    ,
    #[default]
    UnitSelection,
}

impl Pane {
    fn new_canvas(
        // id: usize
        ) -> Self {
        Pane::Canvas 
        // {
            // id,
            // is_pinned: false,
        // }
    }

    fn new_selection() -> Self {
        Pane::UnitSelection
    }

}

fn view_content<'a>(
    _pane: pane_grid::Pane,
    _total_panes: usize,
    _is_pinned: bool,
    size: Size,
    flowsheet: Element<'a, Message>,
) -> Element<'a, Message> {
    let content =
        column![flowsheet, text!("{}x{}", size.width, size.height).size(24), ] // controls,
            .spacing(10)
            .align_x(Center);

    container(content)
        .center_y(Fill)
        .padding(5)
        .into()
}
