use async_trait::async_trait;

use crate::domain::{repositories::NovelRepository, use_cases::GetDummyNovelListUseCase, entities::novel_entity::NovelEntity};

pub(crate) struct GetDummyNovelListUseCaseImpl<'a> {
    novel_repository: &'a dyn NovelRepository,
}

impl<'a> GetDummyNovelListUseCaseImpl<'a> {
    pub(crate) fn new(novel_repository: &'a dyn NovelRepository) -> GetDummyNovelListUseCaseImpl<'a> {
        GetDummyNovelListUseCaseImpl{novel_repository}
    }
}

#[async_trait(?Send)]
impl<'a> GetDummyNovelListUseCase for GetDummyNovelListUseCaseImpl<'a> {
    async fn invoke(&self) -> Result<Vec<NovelEntity>, String> {
        Ok(self.novel_repository.get_dummy_novel_list())
    }
}
