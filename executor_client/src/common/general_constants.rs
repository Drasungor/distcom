pub struct GeneralConstants {
    pub page_default: i64,
    pub limit_default: i64,
}

pub fn get_general_constants() -> GeneralConstants {
    GeneralConstants {
        page_default: 1,
        limit_default: 50,
    }
}