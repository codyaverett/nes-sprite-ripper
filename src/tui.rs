use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

use crate::palette;
use crate::tile::Tile;

const TILES_PER_ROW: usize = 16;

pub fn run_tui(tiles: &[Tile], pal: &[palette::Color; 4]) -> Result<bool> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let total_rows = (tiles.len() + TILES_PER_ROW - 1) / TILES_PER_ROW;
    let mut scroll: usize = 0;
    let mut should_export = false;

    let colors: [Color; 4] = pal.map(|c| Color::Rgb(c[0], c[1], c[2]));

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::vertical([
                Constraint::Min(1),
                Constraint::Length(1),
            ]).split(area);

            let visible_tile_rows = (chunks[0].height as usize) / 4;
            let end_row = (scroll + visible_tile_rows).min(total_rows);

            let mut lines: Vec<Line> = Vec::new();

            for tile_row in scroll..end_row {
                for pixel_pair in (0..8).step_by(2) {
                    let mut spans: Vec<Span> = Vec::new();
                    for tile_col in 0..TILES_PER_ROW {
                        let tile_idx = tile_row * TILES_PER_ROW + tile_col;
                        if tile_idx < tiles.len() {
                            let tile = &tiles[tile_idx];
                            for x in 0..8 {
                                let top = tile.pixels[pixel_pair][x] as usize;
                                let bot = tile.pixels[pixel_pair + 1][x] as usize;
                                spans.push(Span::styled(
                                    "▀",
                                    Style::default()
                                        .fg(colors[top])
                                        .bg(colors[bot]),
                                ));
                            }
                        } else {
                            spans.push(Span::raw("        "));
                        }
                    }
                    lines.push(Line::from(spans));
                }
            }

            let tile_view = Paragraph::new(lines)
                .block(Block::default().borders(Borders::NONE));
            frame.render_widget(tile_view, chunks[0]);

            let status = format!(
                " {} tiles | rows {}-{} of {} | ↑↓=scroll  Enter=export  q/Esc=quit",
                tiles.len(),
                scroll + 1,
                end_row,
                total_rows
            );
            let status_bar = Paragraph::new(status)
                .style(Style::default().fg(Color::Black).bg(Color::White));
            frame.render_widget(status_bar, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Enter => {
                    should_export = true;
                    break;
                }
                KeyCode::Up => {
                    scroll = scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    if scroll + 1 < total_rows {
                        scroll += 1;
                    }
                }
                KeyCode::PageUp => {
                    scroll = scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    scroll = (scroll + 10).min(total_rows.saturating_sub(1));
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;

    Ok(should_export)
}
