use derive_setters::Setters;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::utils::{TTTSide, TTTSymbol};

#[derive(Debug, Default, Setters)]
pub struct TTTCell {
    symbol: Option<TTTSymbol>,
    selected: bool,
    is_dying: bool,
    is_highlighted: bool,
}


impl Widget for TTTCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let widget = if self.is_highlighted {
            Block::default()
                .on_light_green()
        } else if self.selected {
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::default().green())
        } else {
            Block::bordered()
        };

        widget.render(area, buf);

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
    }
}