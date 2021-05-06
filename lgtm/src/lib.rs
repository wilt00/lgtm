use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;

static L_INPUT: &str = include_str!("dicts/l.txt");
static G_INPUT: &str = include_str!("dicts/g.txt");
static T_INPUT: &str = include_str!("dicts/t.txt");
static M_INPUT: &str = include_str!("dicts/m.txt");

pub struct Dict<'a> {
    pub l: Vec<&'a str>,
    pub g: Vec<&'a str>,
    pub t: Vec<&'a str>,
    pub m: Vec<&'a str>,
}

#[wasm_bindgen]
pub fn lgtm() -> String {
    let d: Dict = Dict {
        l: L_INPUT.split('\n').collect(),
        g: G_INPUT.split('\n').collect(),
        t: T_INPUT.split('\n').collect(),
        m: M_INPUT.split('\n').collect(),
    };
    let mut rng = rand::thread_rng();
    [
        (&d.l[..], "let's"),
        (&d.g[..], "get"),
        (&d.t[..], "this"),
        (&d.m[..], "merged"),
    ]
    .iter()
    .map(|d| *d.0.choose(&mut rng).unwrap_or_else(|| &d.1))
    .collect::<Vec<&str>>()
    .join(" ")
}
