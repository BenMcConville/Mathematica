pub mod twod_graph;
pub mod graph_gui;
/*
pub fn graph_setup()    {
    twod_graph::graph("sin(x)");
}*/

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

/*
pub fn run()    {
    let mut directory = String::from("~/");
    directory.push_str("next/"); 
    for i in directory.split("/")   {
        println!("{}", i);
    }
}*/
pub fn graph_setup() -> Result<usize, Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    match res   {
        //Ok(num) => Ok(num),
        Ok(0) => twod_graph::graph(),
        _ => return Ok(99),
    };
    Ok(1)
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<usize> {
    let mut running = true;
    app.next();
    while running {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return match app.state.selected(){
                    Some(num) => Ok(num),
                    _ => Ok(0),
                },
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Right => running = false,
                _ => {}
            }
        }
    }
    println!("-- {:?}", app.state);
    match app.state.selected(){
                    Some(num) => Ok(num),
                    _ => Ok(0),
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    //let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let selected_style = Style::default().fg(Color::Rgb(255,213,0));
    let normal_style = Style::default().bg(Color::Blue);
    /*let header_cells = ["Header1", "Header2"] --Header
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1) 
        .bottom_margin(1);
    */let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        //.header(header) -- Header
        .block(Block::default().borders(Borders::ALL).title("Mathematica"))
        .highlight_style(selected_style)
        .highlight_symbol(" > ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}









struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![ //Current mods (Auto fill later)
                vec!["2D Graph", "(Click Here)Enter any 2D function"],
                vec!["2D Scalar", "Graph"],
                vec!["3D Graph", "Graph"],
            ],
        }
    }
    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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
}

