use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, LevelFilter};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use std::{error::Error, io, time::Duration};
use tui_logger::{init_logger, TuiLoggerWidget};

#[derive(Debug, Clone, Copy, PartialEq)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn next(&self) -> Self {
        match self {
            Self::Trace => Self::Debug,
            Self::Debug => Self::Info,
            Self::Info => Self::Warn,
            Self::Warn => Self::Error,
            Self::Error => Self::Trace,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Trace => Self::Error,
            Self::Debug => Self::Trace,
            Self::Info => Self::Debug,
            Self::Warn => Self::Info,
            Self::Error => Self::Warn,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => write!(f, "Trace"),
            Self::Debug => write!(f, "Debug"),
            Self::Info => write!(f, "Info"),
            Self::Warn => write!(f, "Warn"),
            Self::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
    Dracula,
}

impl Theme {
    fn next(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dracula,
            Self::Dracula => Self::Dark,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Dark => Self::Dracula,
            Self::Light => Self::Dark,
            Self::Dracula => Self::Light,
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => write!(f, "Dark"),
            Self::Light => write!(f, "Light"),
            Self::Dracula => write!(f, "Dracula"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConfigItem {
    Name,
    Counter,
    LogLevel,
    Theme,
}

impl std::fmt::Display for ConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name => write!(f, "Name"),
            Self::Counter => write!(f, "Counter"),
            Self::LogLevel => write!(f, "Log Level"),
            Self::Theme => write!(f, "Theme"),
        }
    }
}

struct Config {
    counter: i32,
    log_level: LogLevel,
    theme: Theme,
    name: String,
}

struct App {
    pub config: Config,
    pub running: bool,
    pub state: ListState,
    pub items: Vec<ConfigItem>,
    pub editing_text: bool,
}

impl App {
    fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            config: Config {
                counter: 0,
                log_level: LogLevel::Info,
                theme: Theme::Dark,
                name: String::from("Ratatui"),
            },
            running: true,
            state,
            items: vec![
                    ConfigItem::Name, 
                    ConfigItem::Counter, 
                    ConfigItem::LogLevel, 
                    ConfigItem::Theme
                ],
            editing_text: false,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn increase(&mut self) {
        let selected_item = self.state.selected().map(|i| self.items[i]);
        if let Some(item) = selected_item {
            match item {
                ConfigItem::Counter => {
                    self.config.counter += 1;
                    info!("Counter increased to {}", self.config.counter);
                }
                ConfigItem::LogLevel => {
                    self.config.log_level = self.config.log_level.next();
                    info!("Log level changed to {}", self.config.log_level);
                }
                ConfigItem::Theme => {
                    self.config.theme = self.config.theme.next();
                    info!("Theme changed to {}", self.config.theme);
                }
                _ => {}
            }
        }
    }

    fn decrease(&mut self) {
        let selected_item = self.state.selected().map(|i| self.items[i]);
        if let Some(item) = selected_item {
            match item {
                ConfigItem::Counter => {
                    self.config.counter -= 1;
                    info!("Counter decreased to {}", self.config.counter);
                }
                ConfigItem::LogLevel => {
                    self.config.log_level = self.config.log_level.prev();
                    info!("Log level changed to {}", self.config.log_level);
                }
                ConfigItem::Theme => {
                    self.config.theme = self.config.theme.prev();
                    info!("Theme changed to {}", self.config.theme);
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    init_logger(LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(LevelFilter::Trace);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    info!("Starting application");
    info!("Navigation: j/k to select, h/l to change values");
    info!("Press 'q' to quit");

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);

    while app.running {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.editing_text {
                        match key.code {
                            KeyCode::Enter | KeyCode::Esc => {
                                app.editing_text = false;
                            }
                            KeyCode::Char(c) => {
                                app.config.name.push(c);
                            }
                            KeyCode::Backspace => {
                                app.config.name.pop();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                            KeyCode::Char('j') | KeyCode::Down => app.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.previous(),
                            KeyCode::Char('l') | KeyCode::Right => app.increase(),
                            KeyCode::Char('h') | KeyCode::Left => app.decrease(),
                            KeyCode::Enter => {
                                let selected_item = app.state.selected().map(|i| app.items[i]);
                                if selected_item == Some(ConfigItem::Name) {
                                    app.editing_text = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(8),
        Constraint::Min(0),
    ])
    .split(f.area());

    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let is_selected = app.state.selected() == Some(i);
            let mut spans = vec![Span::raw(format!("{:<15} ", item.to_string()))];

            match item {
                ConfigItem::Counter => spans.push(Span::raw(format!("[{}]", app.config.counter))),
                ConfigItem::LogLevel => {
                    if is_selected {
                        spans.push(Span::raw("[ "));
                        let options = [
                            LogLevel::Trace,
                            LogLevel::Debug,
                            LogLevel::Info,
                            LogLevel::Warn,
                            LogLevel::Error,
                        ];
                        for (idx, opt) in options.iter().enumerate() {
                            if idx > 0 {
                                spans.push(Span::raw(" | "));
                            }
                            if *opt == app.config.log_level {
                                spans.push(Span::styled(
                                    format!(">{}<", opt.to_string().to_lowercase()),
                                    Style::default()
                                        .fg(Color::Cyan)
                                        .add_modifier(Modifier::BOLD),
                                ));
                            } else {
                                spans.push(Span::raw(format!("{}", opt.to_string().to_lowercase())));
                            }
                        }
                        spans.push(Span::raw(" ]"));
                    } else {
                        spans.push(Span::raw(format!("[{}]", app.config.log_level)));
                    }
                }
                ConfigItem::Theme => {
                    if is_selected {
                        spans.push(Span::raw("[ "));
                        let options = [Theme::Dark, Theme::Light, Theme::Dracula];
                        for (idx, opt) in options.iter().enumerate() {
                            if idx > 0 {
                                spans.push(Span::raw(" | "));
                            }
                            if *opt == app.config.theme {
                                spans.push(Span::styled(
                                    format!(">{}<", opt.to_string().to_lowercase()),
                                    Style::default()
                                        .fg(Color::Cyan)
                                        .add_modifier(Modifier::BOLD),
                                ));
                            } else {
                                spans.push(Span::raw(format!("{}", opt.to_string().to_lowercase())));
                            }
                        }
                        spans.push(Span::raw(" ]"));
                    } else {
                        spans.push(Span::raw(format!("[{}]", app.config.theme)));
                    }
                }
                ConfigItem::Name => {
                    if is_selected && app.editing_text {
                        spans.push(Span::styled(
                            format!("[ {}█ ]", app.config.name),
                            Style::default().fg(Color::Yellow),
                        ));
                    } else {
                        spans.push(Span::raw(format!("[{}]", app.config.name)));
                    }
                }
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Configuration / State (j/k: select, h/l: change) ")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let tui_logger: TuiLoggerWidget = TuiLoggerWidget::default()
        .block(Block::default().title(" Logs ").borders(Borders::ALL))
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan));

    f.render_widget(tui_logger, chunks[1]);
}
