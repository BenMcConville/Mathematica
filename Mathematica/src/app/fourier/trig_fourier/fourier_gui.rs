use super::trigCalc;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, BarChart, BorderType, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
#[derive(Debug)]
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    input_mode: InputMode,
    pub coef_vals: Vec<(f64, f64)>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            coef_vals: vec![(1.0,2.0), (3.0, 4.0), (5.0, 6.0)],
        }
    }
}


pub fn user_input(app: &mut App) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    //let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut run_loop = true;
    while run_loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {//app.input is where to read equation
                        //Need to check equation input:
                        match parseEqn(app) {
                            None => (),
                            _ => println!("Error on Equation"),
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
fn parseEqn(app: &mut App) -> Option<()>   {
    let expr: meval::Expr = app.input.parse().unwrap();
    let func = expr.bind("x");
    match func  {
        Ok(_) => (),
        _ => return Some(()),
    }
    let func = func.unwrap();
    app.coef_vals = trigCalc::calc_coeff(0.0, 6.283185, &func);
    None
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(f.size());

    let bottom_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    let graph = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(bottom_chunk[1]);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Rgb(255,213,0)),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Another one")
        .border_type(BorderType::Rounded);
        //.title_alignment(Alignment::Right);
    f.render_widget(block, chunks[2]);

    let mut text = vec![//Display An + Bn
        Spans::from("An | Bn"),
    ];

    let mut data01: Vec<(&str, u64)> = vec![]; 
    let mut data02: Vec<(&str, u64)> = vec![];
    for (a,b) in &app.coef_vals  {
        text.push(Spans::from(format!(" {:.4} | {:.4}", a, b)));
        data01.push(("a", (a*a*100.0).floor() as u64));
        data02.push(("b", (b*b*100.0).floor() as u64));
    }

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::White));
    f.render_widget(paragraph, bottom_chunk[0]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Here?01") 
        .border_type(BorderType::Rounded);
    f.render_widget(block, bottom_chunk[0]);

    
    let barchart = BarChart::default()
        .block(Block::default().title("Data3").borders(Borders::ALL))
        .data(&data01)
        .bar_style(Style::default().fg(Color::Green))
        .bar_width(4)
        .bar_gap(0)
        .value_style(Style::default().bg(Color::Green))
        .label_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
    );
    f.render_widget(barchart, graph[0]);

    let barchart = BarChart::default()
        .block(Block::default().title("Data3").borders(Borders::ALL))
        .data(&data02)
        .bar_style(Style::default().fg(Color::Blue))
        .bar_width(4)
        .bar_gap(0)
        .value_style(Style::default().bg(Color::Blue))
        .label_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
    );
    f.render_widget(barchart, graph[1]);

}

