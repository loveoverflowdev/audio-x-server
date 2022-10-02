pub(crate) mod components;

use components::{Title, Chapter, Tag};
use serde::{Serialize};

#[warn(dead_code)]
#[derive(Debug, Serialize)]
pub(crate) struct NovelEntity {
    pub title: Title,
    pub chaper_list: Vec<Chapter>,
    pub tag_list: Vec<Tag>,
}
