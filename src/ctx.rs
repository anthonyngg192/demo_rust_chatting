use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Ctx {
    pub user_id: u64,
}

impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}
