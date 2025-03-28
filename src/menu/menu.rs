use ratatui::{
    backend::CrosstermBackend, crossterm, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::Span, widgets::{Block, Borders, List, ListItem, Paragraph}, Terminal
};
use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Menu {
    title: String,
    options: Vec<String>,
    selected_index: usize,
    running: Arc<AtomicBool>, // Shared flag to control menu state
}

impl Menu {
    pub fn new(title: &str, options: Vec<&str>) -> Self {
        Self {
            title: title.to_string(),
            options: options.into_iter().map(|s| s.to_string()).collect(),
            selected_index: 0,
            running: Arc::new(AtomicBool::new(true)), // Initialize as running
        }
    }

    pub fn run<F>(&mut self, mut on_select: F) -> io::Result<()>
    where
        F: FnMut(usize, Arc<AtomicBool>) -> io::Result<()>, // Pass the running flag to the callback
    {
        // Enable raw mode and switch to alternate screen
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        while self.running.load(Ordering::SeqCst) {
            terminal.draw(|f| {
                let size = f.size();

                // Layout
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                // Title
                let title = Paragraph::new(self.title.as_str())
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .block(Block::default().borders(Borders::ALL).title("Menu"));

                f.render_widget(title, chunks[0]);

                // Menu Items
                let items: Vec<ListItem> = self
                    .options
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let style = if i == self.selected_index {
                            Style::default().fg(Color::Black).bg(Color::White)
                        } else {
                            Style::default()
                        };
                        ListItem::new(Span::from(Span::styled(item.clone(), style)))
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Options"))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                f.render_widget(list, chunks[1]);

                // Footer
                let footer = Paragraph::new("Use Up/Down to navigate, Enter to select")
                    .style(Style::default().fg(Color::Gray));

                f.render_widget(footer, chunks[2]);
            })?;

            // Handle input
            if let Ok(event) = event::read() {
                if let Event::Key(KeyEvent { code, .. }) = event {
                    match code {
                        KeyCode::Up => {
                            if self.selected_index > 0 {
                                self.selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_index < self.options.len() - 1 {
                                self.selected_index += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if self.options[self.selected_index] == "Quit" {
                                self.running.store(false, Ordering::SeqCst);
                            } else {
                                on_select(self.selected_index, self.running.clone())?;
                            }
                        }
                        KeyCode::Char('q') => self.running.store(false, Ordering::SeqCst),
                        _ => {}
                    }
                }
            }
        }

        // Restore terminal state
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }
}
