use derive_setters::Setters;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::pos2::Pos2;
use crate::utils::{TTTSide, TTTSymbol};

#[derive(Debug, Default, Setters)]
pub struct TTTCell {
    symbol: Option<TTTSymbol>,
    selected: bool,
    is_dying: bool,
    is_highlighted: bool,
    position: Pos2,
}


impl Widget for TTTCell {
    fn render(self, inner_area: Rect, buf: &mut Buffer) {
        let mut border_flags = Borders::ALL;
        let mut border_set = symbols::border::PLAIN;

        match &self.position {
            pos if pos == &Pos2::new(0, 0) => {
                border_flags = Borders::LEFT | Borders::TOP;
            }

            pos if pos == &Pos2::new(1, 0) => {
                border_flags = Borders::LEFT | Borders::TOP;
                border_set = symbols::border::Set {
                    top_left: symbols::line::HORIZONTAL_DOWN,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(2, 0) => {
                border_flags = Borders::LEFT | Borders::TOP | Borders::RIGHT;
                border_set = symbols::border::Set {
                    top_left: symbols::line::HORIZONTAL_DOWN,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(0, 1) => {
                border_flags = Borders::LEFT | Borders::TOP;
                border_set = symbols::border::Set {
                    top_left: symbols::line::VERTICAL_RIGHT,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(1, 1) => {
                border_flags = Borders::LEFT | Borders::TOP;
                border_set = symbols::border::Set {
                    top_left: symbols::line::CROSS,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(2, 1) => {
                border_flags = Borders::LEFT | Borders::TOP | Borders::RIGHT;
                border_set = symbols::border::Set {
                    top_left: symbols::line::CROSS,
                    top_right: symbols::line::VERTICAL_LEFT,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(0, 2) => {
                border_flags = Borders::LEFT | Borders::TOP | Borders::BOTTOM;
                border_set = symbols::border::Set {
                    top_left: symbols::line::VERTICAL_RIGHT,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(1, 2) => {
                border_flags = Borders::LEFT | Borders::TOP | Borders::BOTTOM;
                border_set = symbols::border::Set {
                    top_left: symbols::line::CROSS,
                    bottom_left: symbols::line::HORIZONTAL_UP,
                    ..symbols::border::PLAIN
                };
            }

            pos if pos == &Pos2::new(2, 2) => {
                border_flags = Borders::ALL;
                border_set = symbols::border::Set {
                    top_left: symbols::line::CROSS,
                    top_right: symbols::line::VERTICAL_LEFT,
                    bottom_left: symbols::line::HORIZONTAL_UP,
                    ..symbols::border::PLAIN
                };
            }

            _ => {}
        }

        let area = Rect::new(
            inner_area.x - 1,
            inner_area.y - 1,
            inner_area.width + 2,
            inner_area.height + 2,
        );

        let border_block = Block::bordered()
            .borders(border_flags)
            .border_set(border_set);

        border_block.render(area, buf);

        if let Some(symbol) = self.symbol {
            let (str, mut style) = match symbol.side {
                TTTSide::Cross => {
                    if self.is_dying {
                        ("X", Style::default().red().dim().underlined())
                    } else {
                        ("X", Style::default().red())
                    }
                }
                TTTSide::Circle => {
                    if self.is_dying {
                        ("O", Style::default().green().dim().underlined())
                    } else {
                        ("O", Style::default().green())
                    }
                }
            };

            if self.is_highlighted {
                style = style.black();
            }

            buf.set_string(
                area.x + area.width / 2,
                area.y + area.height / 2,
                str,
                style,
            );
        }


        if self.selected {
            let selection = Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::default().green());

            selection.render(inner_area, buf);
        } else if self.is_highlighted {
            let highlight = Block::bordered()
                .border_type(BorderType::Thick)
                .border_style(Style::default().green())
                .bg(Color::Green);
            highlight.render(area, buf);
        }
    }
}