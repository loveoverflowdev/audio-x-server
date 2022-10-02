use actix_web::{get, HttpResponse};
use crate::core::api_response;
use crate::{data::{repositories::novel_repository::NovelRepositoryImpl, use_cases::get_dummy_novel_list_use_case_impl::GetDummyNovelListUseCaseImpl}, domain::{repositories::NovelRepository, use_cases::GetDummyNovelListUseCase}};

#[get("/novel_list")]
async fn get_novel_list() -> HttpResponse {
    let novel_repository = NovelRepositoryImpl::new();
    let get_dummy_novel_list_use_case = GetDummyNovelListUseCaseImpl::new(&novel_repository);
    let response = get_dummy_novel_list_use_case.invoke().await;

    api_response::pack(response)
}
