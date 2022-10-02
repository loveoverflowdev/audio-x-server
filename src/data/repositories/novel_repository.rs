use crate::domain::{repositories::NovelRepository, entities::novel_entity::{NovelEntity, components::{Title, Chapter, Tag}}};

pub(crate) struct NovelRepositoryImpl {
    
}

impl NovelRepository for NovelRepositoryImpl {
    fn new() -> NovelRepositoryImpl {
        NovelRepositoryImpl{}
    }
    
    fn get_dummy_novel_list(&self) -> Vec<NovelEntity> {
        vec![
            NovelEntity {
                title: Title {
                    author: String::from("Author"),
                    introduction: String::from("A long story"),
                },
                chaper_list: vec![
                    Chapter {
                        index:1, 
                        name: String::from("Chapter 1: Please, Forgive me"), 
                        content: String::from("Sorry about left you alone"),
                    }
                ],
                tag_list: vec![
                    Tag::MartialArt,
                ],
            },
            NovelEntity {
                title: Title {
                    author: String::from("Alexander"),
                    introduction: String::from("A short story"),
                },
                chaper_list: vec![
                    Chapter {
                        index:1, 
                        name: String::from("Chapter 1: Sorry, Blame it on me"), 
                        content: String::from("Sorry about left you alone"),
                    }
                ],
                tag_list: vec![
                    Tag::MartialArt,
                ],
            },
        ]
    }
}
