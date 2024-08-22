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
    let limit_max = common::config::GENERAL_CONSTANTS.limit_max;
    let limit_min = common::config::GENERAL_CONSTANTS.limit_min;
    let page_min = common::config::GENERAL_CONSTANTS.page_min;

    if let Some(received_limit) = paging_params.limit {
        if received_limit > limit_max {
            limit = limit_max;
        } else if received_limit < limit_min {
            limit = limit_min;
        } else {
            limit = received_limit;
        }
    } else {
        limit = limit_max;
    }

    if let Some(received_page) = paging_params.page {
        if received_page < page_min {
            page = page_min;
        } else {
            page = received_page;
        }
    } else {
        page = common::config::GENERAL_CONSTANTS.page_min;
    }

    MandatoryPagingParameters {
        limit,
        page,
    }
}