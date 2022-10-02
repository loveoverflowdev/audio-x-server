use super::entities::novel_entity::NovelEntity;

pub(crate) trait NovelRepository {
    fn new() -> Self where Self: Sized;
    fn get_dummy_novel_list(&self) -> Vec<NovelEntity>;
}
