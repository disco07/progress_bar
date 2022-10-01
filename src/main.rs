use std::{io, time};
use std::borrow::Borrow;
use std::fmt::Error;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::time::{Duration, Instant};

struct Bar {
    state: State,
    option: Option,
    theme: Theme,
}

struct State {
    percent: f64,
    current: i64,
    current_graph_rate: isize,
}

struct Option {
    total: i64,
    start_time: Instant,
}

struct Theme {
    rate: String,
    bar_type: char,
    bar_start: char,
    bar_end: char,
    bar_width: isize,
}

impl State {
    fn new(max: i64) -> State {
        Self {
            current: 0,
            percent: get_percent(&0, &max),
            current_graph_rate: 0,
        }
    }
}

impl Theme {
    fn new(bar_type: char, bar_start: char, bar_end: char, bar_width: isize) -> Theme {
        Self {
            rate: "".to_string(),
            bar_type,
            bar_start,
            bar_end,
            bar_width,
        }
    }
}

impl Option {
    fn new(total: i64, time: Instant) -> Option {
        Self {
            total,
            start_time: time,
        }
    }
}

impl Bar {
    fn new(max: i64) -> Self {
        Self {
            state: State::new(max),
            option: Option::new(max, Instant::now()),
            theme: Theme::new('█', '[', ']', 50),
        }
    }

    fn render(&mut self) {
        let last = self.state.percent;
        self.state.percent = get_percent(&self.state.current, &self.option.total);
        let last_graph_rate = self.state.current_graph_rate;
        self.state.current_graph_rate = (self.state.percent / 100.0 * (self.theme.bar_width as f64)) as isize;
        if self.state.percent != last {
            let n: usize = (self.state.current_graph_rate - last_graph_rate) as usize;
            self.theme.rate.push_str("111");
        }
        let width = self.theme.bar_width;
        println!("\r{}{:>width$}{}{}{}%",
                 self.theme.bar_start,
                 width,
                 self.theme.rate,
                 self.theme.bar_end,
                 self.state.percent);
    }

    fn add(&mut self, num: isize) {
        assert!(&self.option.total == 0, "the max must be greater than zero");
        &self.state.current += (num as i64).borrow();
        assert!(&self.state.current > &self.option.total, "current exceeds total")
            & self.render()
    }
}

fn get_percent(current: &i64, total: &i64) -> f64 {
    100 * (current as f64) / (total as f64)
}

fn main() {
    let mut bar = Bar::new(100);

    for i in 0..100 {
        bar.add(1);
    }
}