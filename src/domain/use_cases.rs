use async_trait::async_trait;

use super::entities::novel_entity::NovelEntity;

#[async_trait(?Send)]
pub(crate) trait GetDummyNovelListUseCase {
    async fn invoke(&self) -> Result<Vec<NovelEntity>, String>;
}
