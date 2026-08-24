mod markdown_list_parser;

use crate::markdown_list_parser::parse_list;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, row, space, text, Column, Container};
use iced::{padding, Element, Length, Size};
use rand::seq::IteratorRandom;
use rfd::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct State {
    lists: Vec<List>,
    selected: String,
}

struct List {
    elements: Vec<String>,
    text: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ChooseFile,
    Randomize(usize),
}

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .window_size(Size::new(800.0, 600.0))
        .resizable(false)
        .run()
}

impl State {
    pub fn view(&self) -> Container<'_, Message> {
        let default_padding = padding::vertical(20).horizontal(30);
        let column_width = 200.0;

        let choose_file = button(text("Choose file").align_x(Horizontal::Center))
            .width(column_width)
            .height(30)
            .on_press(Message::ChooseFile);

        let mut buttons: Vec<Element<Message>> = Vec::new();
        for (index, list) in self.lists.iter().enumerate() {
            buttons.push(
                button(list.text.as_str())
                    .on_press(Message::Randomize(index))
                    .clip(true)
                    .width(column_width)
                    .height(30)
                    .into(),
            );
        }
        let column: Column<'_, Message> = Column::from_vec(buttons)
            .width(column_width)
            .spacing(10)
            .into();

        let result_container: Container<'_, Message> = if !self.selected.is_empty() {
            container(
                text(&self.selected)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(column_width - default_padding.left - default_padding.right),
            )
            .padding(default_padding)
            .style(container::bordered_box)
        } else {
            container(space()).width(column_width)
        };

        let row = row![choose_file, column, result_container].spacing(10);
        container(row)
            .align_x(Horizontal::Center)
            .padding(default_padding)
            .width(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChooseFile => {
                let chosen_path = FileDialog::new()
                    .add_filter("Markdown", &["md"])
                    .set_directory("/")
                    .pick_file();

                match chosen_path {
                    None => {}
                    Some(path) => {
                        let file = File::open(path).unwrap();
                        let reader = BufReader::new(file);

                        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
                        let lists = parse_list(lines);
                        self.lists = lists;
                    }
                }
            }
            Message::Randomize(index) => {
                let random_element = &self.lists[index]
                    .elements
                    .iter()
                    .choose(&mut rand::rng())
                    .unwrap();
                self.selected = random_element.to_string();
            }
        }
    }
}
