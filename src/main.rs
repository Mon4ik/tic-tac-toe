use std::io::{Result, stdout};
use std::rc::Rc;
use std::time::Duration;

use crossterm::{
    event::KeyCode,
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::{Event, poll, read};
//
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

use crate::cell::TTTCell;
use crate::engine::Engine;
use crate::pos2::Pos2;
use crate::utils::{TTTSide, TTTSymbol};

mod cell;
mod pos2;
mod utils;
mod engine;

const CELL_WIDTH: u16 = 13;
const CELL_HEIGHT: u16 = 7;


fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    terminal.clear()?;

    while !app.should_quit {
        terminal.draw(|frame| {
            app.ui(frame)
        })?;

        app.handle()?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

#[derive(PartialEq)]
enum AppState {
    Game,
    ShowingWinner,
}

struct App {
    should_quit: bool,
    selection: Pos2,

    state: AppState,
    highlighting: Vec<Pos2>,

    side: TTTSide,
    stack: Vec<TTTSymbol>,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,

            state: AppState::Game,
            highlighting: vec![],

            selection: Pos2::new(0, 0),
            side: TTTSide::Cross,
            stack: vec![],
        }
    }

    fn handle(&mut self) -> Result<()> {
        if poll(Duration::from_millis(1 / 60 * 1000))? {
            match read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,

                        KeyCode::Left => {
                            if self.selection.x > 0 {
                                self.selection -= Pos2::new(1, 0);
                            }
                        }
                        KeyCode::Right => {
                            if self.selection.x < 2 {
                                self.selection += Pos2::new(1, 0);
                            }
                        }
                        KeyCode::Up => {
                            if self.selection.y > 0 {
                                self.selection -= Pos2::new(0, 1);
                            }
                        }
                        KeyCode::Down => {
                            if self.selection.y < 2 {
                                self.selection += Pos2::new(0, 1);
                            }
                        }

                        KeyCode::Enter => {
                            if self.state == AppState::ShowingWinner {
                                self.state = AppState::Game;
                                self.side = TTTSide::Cross;

                                self.stack = vec![];
                                self.highlighting = vec![];
                                self.selection = Pos2::new(0, 0);

                                return Ok(());
                            }

                            for symbol in &self.stack {
                                if symbol.pos == self.selection {
                                    return Ok(());
                                }
                            }

                            if self.stack.len() == 6 {
                                self.stack.remove(0);
                            }

                            self.stack.push(TTTSymbol {
                                pos: self.selection.clone(),
                                side: self.side.clone(),
                            });

                            let (winner, mut poses) = Engine::calculate_winner(&self.stack);
                            if winner != " " {
                                self.state = AppState::ShowingWinner;
                                self.highlighting.append(&mut poses);
                                return Ok(());
                            }

                            match self.side {
                                TTTSide::Cross => self.side = TTTSide::Circle,
                                TTTSide::Circle => self.side = TTTSide::Cross,
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        let table_width: u16 = CELL_WIDTH * 3 + 2 + 2;
        let table_height: u16 = CELL_HEIGHT * 3 + 2 + 2;

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(table_height)
            ])
            .split(Rect::new(0, 0, table_width, table_height + 5));

        let turn_of = if self.side == TTTSide::Cross {
            Span::styled("Cross", Style::default().red().bold())
        } else {
            "Circle".green()
        };

        let status_line = if self.state == AppState::Game {
            vec![
                Span::styled(" Current turn of: ", Style::default()),
                turn_of.into(),
            ].into()
        } else {
            vec![
                Span::styled(" WINNER: ", Style::default()),
                turn_of.into(),
            ].into()
        };

        frame.render_widget(
            Paragraph::new(vec![
                Line::default(),
                status_line,
            ])
                .block(Block::bordered()),
            layout[0],
        );

        self.draw_ttt(frame, layout[1]);
    }

    fn draw_ttt(&self, frame: &mut Frame, rect: Rect) {
        let table = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(CELL_HEIGHT),
                Constraint::Length(CELL_HEIGHT),
                Constraint::Length(CELL_HEIGHT),
            ])
            .spacing(1)
            .margin(1)
            .split(rect);

        let mut rows: Vec<Rc<[Rect]>> = vec![];

        for i in 0..3usize {
            rows.push(
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(CELL_WIDTH),
                        Constraint::Length(CELL_WIDTH),
                        Constraint::Length(CELL_WIDTH),
                    ])
                    .spacing(1)
                    .split(table[i])
            );
        }

        for y in 0..3usize {
            for x in 0..3usize {
                let current_pos = Pos2::new(x, y);

                let mut symbol: Option<TTTSymbol> = None;
                for iter_symbol in &self.stack {
                    if iter_symbol.pos == current_pos {
                        symbol = Some(iter_symbol.clone())
                    }
                }

                let is_dying = if self.stack.len() < 6 {
                    false
                } else {
                    self.stack[0].pos == current_pos
                };

                frame.render_widget(
                    TTTCell::default()
                        .symbol(symbol)
                        .is_dying(is_dying)
                        .is_highlighted(self.highlighting.contains(&current_pos))
                        .selected(self.state == AppState::Game && self.selection == current_pos)
                        .position(current_pos),
                    rows[y][x],
                );
            }
        }
    }
}

