use rand::seq::SliceRandom;

pub struct Dict<'a> {
    pub l: Vec<&'a str>,
    pub g: Vec<&'a str>,
    pub t: Vec<&'a str>,
    pub m: Vec<&'a str>,
}

pub fn lgtm(d: &Dict) -> String {
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
