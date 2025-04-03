use elnard::Elnard;
use ratatui::{buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::Rect, symbols::border, text::{Line, Text}, widgets::{Block, Padding, Paragraph, Widget}, DefaultTerminal, Frame};

mod elnard;

fn main() {
    let mut terminal = ratatui::init();
    let _ = App::default().run(&mut terminal);
    ratatui::restore();
}

#[derive(Default)]
struct App {
    elnard: Elnard,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) {
        self.elnard.populate_list();
        self.elnard.current_matches = 512;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame)).expect("msg");
            self.handle_events();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) {
        match event::read().expect("msg") {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit = true,

            KeyCode::Up => {
                self.elnard.current_values.push(0);
                self.elnard.find_possible_matches();
            }

            KeyCode::Down => {
                self.elnard.current_values.push(1);
                self.elnard.find_possible_matches();
            }

            KeyCode::Left => {
                self.elnard.current_values.push(2);
                self.elnard.find_possible_matches();
            }

            KeyCode::Right => {
                self.elnard.current_values.push(3);
                self.elnard.find_possible_matches();
            }

            KeyCode::Enter => {
                self.elnard.current_values.push(4); // wildcard
                self.elnard.find_possible_matches();
            }

            KeyCode::Backspace => {
                if self.elnard.current_values.pop().is_some() {
                    self.elnard.find_possible_matches();
                }
            }

            KeyCode::Delete => {
                self.elnard.current_values.clear();
                self.elnard.find_possible_matches();
            }
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(vec![" 7th Saga seed manipulation ".into()]);
        let title_bottom = Line::from(vec![" Arrow keys: movement | Enter: wildcard | Backspace: undo | Delete: clear ".into()]);
        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .title(title.centered())
            .title_bottom(title_bottom.centered())
            .border_set(border::PLAIN);

        let arrows = ['↑', '↓', '←', '→', '!'];

        let mut cur_steps2 = String::new();
        for &steps in &self.elnard.current_values {
            cur_steps2.push(arrows[steps as usize]);
            cur_steps2.push_str("  ");
        }

        let mut next_steps = Vec::new();
        if let Some(upcoming) = &self.elnard.upcoming_values {
            for list in upcoming {
                let mut res2 = String::new();

                for &steps in list {
                    res2.push(arrows[steps as usize]);
                    res2.push_str("  ");
                }

                next_steps.push(res2);
            }
        }

        let mut lines = vec![
            Line::from(format!("Current matches: {}", self.elnard.current_matches)),
            Line::from(""),
            Line::from("Current steps:"),
            Line::from(cur_steps2),
            Line::from(""),
            Line::from("Next steps:"),
        ];

        for next in next_steps {
            lines.push(Line::from(next));
        }

        let counter_text = Text::from(lines);

        Paragraph::new(counter_text).left_aligned().block(block).render(area, buf);
    }
}
