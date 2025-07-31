mod algorithms;
use std::io::{self};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};
use std::sync::{Arc, Mutex};
use std::thread;

mod data;
mod scenes;

use algorithms::maze_generation::RecursiveBacktracker;
use algorithms::sorting::BubbleSort;
use data::data_structures::{Maze, MazeContext, Runnable, Runner, Scene, SortingContext};
use scenes::{maze_scene::MazeScene, sort_scene::SortScene};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_items = vec!["Maze Generation", "Sorting", "Quit"];
    let mut selected = 0;
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &menu_items, selected))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < menu_items.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => match menu_items[selected] {
                        "Quit" => break,
                        "Sorting" => run_sorting(&mut terminal)?,
                        "Maze Generation" => run_maze(&mut terminal)?,
                        _ => {}
                    },
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn ui<B: Backend>(f: &mut ratatui::Frame<B>, items: &[&str], selected: usize) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            items
                .iter()
                .map(|_| Constraint::Length(3))
                .collect::<Vec<Constraint>>(),
        )
        .split(size);

    for (i, item) in items.iter().enumerate() {
        let style = if i == selected {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title(*item)
            .style(style);
        f.render_widget(block, chunks[i]);
    }
}

fn run_sorting(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scene = SortScene::new();
    let scene_arc = Arc::new(Mutex::new(scene));
    let tasks: Vec<Box<dyn Runnable<Vec<i32>, SortingContext>>> = vec![Box::new(BubbleSort)];
    let data = vec![5, 3, 1, 4, 2];
    let mut runner = Runner::new(tasks, scene_arc.clone(), data);
    runner.start();
    loop {
        terminal.draw(|f| {
            scene_arc.lock().unwrap().render(f);
        })?;
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    runner.stop();
                    break;
                }
            }
        }
    }
    Ok(())
}

fn run_maze(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MazeScene::new(10, 10);
    let scene_arc = Arc::new(Mutex::new(scene));
    let tasks: Vec<Box<dyn Runnable<Maze, MazeContext>>> = vec![Box::new(RecursiveBacktracker)];
    let data = Maze::new(10, 10);
    let mut runner = Runner::new(tasks, scene_arc.clone(), data);
    runner.start();
    loop {
        terminal.draw(|f| {
            scene_arc.lock().unwrap().render(f);
        })?;
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    runner.stop();
                    break;
                }
            }
        }
    }
    Ok(())
}
