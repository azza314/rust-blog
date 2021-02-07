use crate::errors::AppError; 
use actix_web::HttpResponse; 

pub(super) mod comments; 
pub(super) mod users;
pub(super) mod posts; 

fn convert<T, E>(res: Result<T, E>) -> Result<HttpResponse, AppError>
where
    T: serde::Serialize, 
    AppError: From<E>, 
{
    res.map(|d| HttpResponse::Ok().json(d))
    .map_err(Into::into)
}