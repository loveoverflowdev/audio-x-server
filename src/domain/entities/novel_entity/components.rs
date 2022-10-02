use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Chapter {
    pub index: i32,
    pub  name: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub(crate) enum Tag {
    MartialArt,
    Mystery,
    Romance,
    Adult,
    Xuanhuan,
    Fantasy,
    School,
    Tragedy,
}

#[derive(Debug, Serialize)]
pub(crate) struct Title {
    pub author: String,
    pub introduction: String,
}
