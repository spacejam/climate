#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod colors;
mod config;
mod dateparse;
mod logging;
mod meta;
mod node;
mod pack;
mod pb;
mod plot;
mod screen;
mod serialization;
mod tagdb;
mod task;

use std::{cmp, collections::HashMap};

use regex::Regex;

pub use crate::{
    colors::random_fg_color,
    config::{Action, Config},
    dateparse::dateparse,
    logging::init_screen_log,
    meta::Meta,
    node::Node,
    pack::Pack,
    screen::Screen,
    serialization::{deserialize_screen, serialize_screen},
    tagdb::TagDB,
};

pub type Coords = (u16, u16);
pub type NodeID = u64;
pub type ScreenDesc = (HashMap<Coords, NodeID>, HashMap<NodeID, Coords>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    L,
    R,
}

pub fn distances(c1: Coords, c2: Coords) -> (u16, u16) {
    let x_cost = cmp::max(c1.0, c2.0) - cmp::min(c1.0, c2.0);
    let y_cost = cmp::max(c1.1, c2.1) - cmp::min(c1.1, c2.1);
    (x_cost, y_cost)
}

pub fn cost(c1: Coords, c2: Coords) -> u16 {
    let (x_cost, y_cost) = distances(c1, c2);
    x_cost + y_cost
}

pub fn re_matches<A: std::str::FromStr>(re: &Regex, on: &str) -> Vec<A> {
    let mut ret = vec![];
    if re.is_match(on) {
        for cap in re.captures_iter(on) {
            if let Some(a) = cap.get(1) {
                if let Ok(e) = a.as_str().parse::<A>() {
                    ret.push(e)
                }
            }
        }
    }
    ret
}

#[test]
fn test_regex_parsing() {
    let re = Regex::new(r"(\S+)").unwrap();
    assert_eq!(
        re_matches::<String>(&re, "yo ho ho"),
        vec!["yo".to_owned(), "ho".to_owned(), "ho".to_owned()]
    );
}
