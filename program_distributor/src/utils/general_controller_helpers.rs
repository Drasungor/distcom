use serde::Deserialize;

use crate::common;


#[derive(Deserialize)]
pub struct PagingParameters {
    pub limit: Option<i64>,
    pub page: Option<i64>,
}

pub struct MandatoryPagingParameters {
    pub limit: i64,
    pub page: i64,
}

pub fn process_paging_inputs(paging_params: PagingParameters) -> MandatoryPagingParameters {
    let limit;
    let page;

    if let Some(received_limit) = paging_params.limit {
        limit = received_limit;
    } else {
        limit = common::config::GENERAL_CONSTANTS.limit_max;
    }

    if let Some(received_page) = paging_params.page {
        page = received_page;
    } else {
        page = common::config::GENERAL_CONSTANTS.page_min;
    }

    return MandatoryPagingParameters {
        limit,
        page,
    }
}