mod flowsheet;
mod style;

use iced::keyboard;
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{toggler, button, column, container, horizontal_space, hover, responsive, row, text};
use iced::{Center, Color, Element, Fill, Size, Subscription, Theme};

use log::{error, warn, info, debug, trace};


pub fn main() -> iced::Result {
    env_logger::init();
    info!("Starting application");
    iced::application("Open Source Chemical Process Simulator", MainWindow::update, MainWindow::view)
        .theme(|_| Theme::CatppuccinMocha)
        .antialiasing(true)
        .centered()
        .run()
}

struct MainWindow {
    theme: Theme,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    flowsheet: flowsheet::State,
    curves: Vec<flowsheet::Curve>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(flowsheet::Curve),
    Clear,
    PlaceComponent(flowsheet::BlockPlacement),
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
}

impl MainWindow {

    fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::new_canvas(0));
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::new_selection());

        MainWindow {
            theme: Theme::default(),
            panes,
            panes_created: 1,
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
            Message::Split(_axis, _pane) => {
                info!("You split a pane!")
            },
            Message::SplitFocused(_axis) => {
                info!("You split a focused pane!")
            },
            Message::FocusAdjacent(_direction) => (),
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
            Message::TogglePin(_pane) => {
                println!("You pinned a pane!")
            },
            Message::Maximize(_pane) => {
                println!("You maximized a pane!")
            },
            Message::Restore => {
                println!("You restored a window!")
            },
            Message::Close(_pane) => {
                println!("You closed a pane!")
            },
            Message::CloseFocused => {
                println!("You closed a focused pane!")
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            if !modifiers.command() {
                return None;
            }

            handle_hotkey(key_code)
        })
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
        match pane {
            Pane::Canvas{ id: _, is_pinned: _} => {
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

        //  let pin_button = button(
        //     text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14),
        //  )
        // .on_press(Message::TogglePin(id))
        // .padding(3);

        let title = row![
            // pin_button,
            "Flowsheet",
            // text(pane.id.to_string()).color(if is_focused{
            //     PANE_ID_COLOR_FOCUSED
            // } else {
            //     PANE_ID_COLOR_UNFOCUSED
            // }),
        ]
            .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                // .controls(pane_grid::Controls::dynamic(
                //         view_controls(
                //             id,
                //             total_panes,
                //             pane.is_pinned,
                //             is_maximized,
                //         ),
                //         button(text("X").size(14))
                //             .style(button::danger)
                //             .padding(3)
                //             .on_press_maybe(
                //                 if total_panes > 1 && !pane.is_pinned {
                //                     Some(Message::Close(id))
                //                 } else {
                //                     None
                //                 },
                //             ),
                // ))
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
                // pane.is_pinned,
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

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);

const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::{Axis, Direction};

    match key.as_ref() {
        Key::Character("v") => Some(Message::SplitFocused(Axis::Vertical)),
        Key::Character("h") => Some(Message::SplitFocused(Axis::Horizontal)),
        Key::Character("w") => Some(Message::CloseFocused),
        Key::Named(key) => {
            let direction = match key {
                key::Named::ArrowUp => Some(Direction::Up),
                key::Named::ArrowDown => Some(Direction::Down),
                key::Named::ArrowLeft => Some(Direction::Left),
                key::Named::ArrowRight => Some(Direction::Right),
                _ => None,
            };

            direction.map(Message::FocusAdjacent)
        }
        _ => None,
    }
}

// #[derive(Clone,Copy,Default)]
// struct Pane {
//     id: usize,
//     pub is_pinned: bool,
// }
#[derive(Clone,Copy,Default)]
enum Pane {
    Canvas{
        id: usize,
        is_pinned: bool,
    },
    #[default]
    UnitSelection,
}

impl Pane {
    fn new_canvas(id: usize) -> Self {
        Pane::Canvas {
            id,
            is_pinned: false,
        }
    }

    fn new_selection() -> Self {
        Pane::UnitSelection
    }

    // fn default() -> Self {
    //     Pane::Canvas {
    //         id: 0,
    //         is_pinned: false,
    //     }
    // }
}


fn view_content<'a>(
    _pane: pane_grid::Pane,
    _total_panes: usize,
    _is_pinned: bool,
    size: Size,
    flowsheet: Element<'a, Message>,
) -> Element<'a, Message> {
    // let button = |label, message| {
    //     button(text(label).width(Fill).align_x(Center).size(16))
    //         .width(Fill)
    //         .padding(8)
    //         .on_press(message)
    // };

    // let controls = column![
    //     button(
    //         "Split horizontally",
    //         Message::Split(pane_grid::Axis::Horizontal, pane),
    //     ),
    //     button(
    //         "Split vertically",
    //         Message::Split(pane_grid::Axis::Vertical, pane),
    //     )
    // ]
    // .push_maybe(if total_panes > 1 && !is_pinned {
    //     Some(button("Close", Message::Close(pane)).style(button::danger))
    // } else {
    //     None
    // })
    // .spacing(5)
    // .max_width(160);

    let content =
        column![flowsheet, text!("{}x{}", size.width, size.height).size(24), ] // controls,
            .spacing(10)
            .align_x(Center);

    // container(scrollable(content))
    container(content)
        .center_y(Fill)
        .padding(5)
        .into()
}

fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", Message::Restore)
        } else {
            ("Maximize", Message::Maximize(pane))
        };

        Some(
            button(text(content).size(14))
                .style(button::secondary)
                .padding(3)
                .on_press(message),
        )
    } else {
        None
    });

    let close = button(text("Close").size(14))
        .style(button::danger)
        .padding(3)
        .on_press_maybe(if total_panes > 1 && !is_pinned {
            Some(Message::Close(pane))
        } else {
            None
        });

    row.push(close).into()
}
