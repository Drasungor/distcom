pub struct GeneralConstants {
    pub page_min: i64,
    pub limit_min: i64,
    pub limit_max: i64,
}

pub fn get_general_constants() -> GeneralConstants {
    return GeneralConstants {
        page_min: 1,
        limit_min: 1,
        limit_max: 50,
    }
}