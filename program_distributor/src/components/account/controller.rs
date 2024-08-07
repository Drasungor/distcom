use actix_web::{web, Responder};

use crate::common;
use crate::common::app_http_response_builder::AppHttpResponseBuilder;
use crate::utils::general_controller_helpers::{process_paging_inputs, PagingParameters};
use crate::utils::jwt_helpers::{validate_jwt, Claims};

use super::service::AccountService;
use super::model::{Credentials, GetPagedOrganizations, ReceivedNewAccount, RefreshToken, TokenId};

pub struct AccountController;

impl AccountController {

    pub async fn register(body: web::Json<ReceivedNewAccount>) -> impl Responder {
        let registration_result = AccountService::register(body.into_inner()).await;
        return AppHttpResponseBuilder::get_http_response(registration_result);
    }

    pub async fn login(body: web::Json<Credentials>) -> impl Responder {
        let login_result = AccountService::login(body.username.clone(), body.password.clone()).await;
        return AppHttpResponseBuilder::get_http_response(login_result);
    }

    pub async fn refresh_basic_token(body: web::Json<RefreshToken>) -> impl Responder {
        let refresh_token = body.refresh_token.clone();
        let refresh_token_payload: Claims = validate_jwt(common::config::CONFIG_OBJECT.token.refresh_token_secret.as_str(), &refresh_token).expect("Error in token decoding");
        let refresh_token_id = refresh_token_payload.token_id;
        let organization_id = refresh_token_payload.organization_id;
        let token_refreshment_result = AccountService::refresh_basic_token(refresh_token_id, organization_id).await;
        return AppHttpResponseBuilder::get_http_response(token_refreshment_result);
    }

    pub async fn delete_refresh_token(body: web::Json<TokenId>) -> impl Responder {
        let login_result = AccountService::delete_refresh_token(body.token_id.clone()).await;
        return AppHttpResponseBuilder::get_http_response(login_result);
    }
    
    pub async fn get_paged_organizations(query_params: web::Query<GetPagedOrganizations>) -> impl Responder {
        let query_params = query_params.into_inner();
        let paging = PagingParameters {
            limit: query_params.limit,
            page: query_params.page,
        };
        let paging_params = process_paging_inputs(paging);
        let get_organizations_result = AccountService::get_organizations(query_params.name_filter, paging_params.limit, paging_params.page).await;
        return AppHttpResponseBuilder::get_http_response(get_organizations_result);
    }
    
}
