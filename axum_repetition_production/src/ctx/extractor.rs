#[derive(Debug, Clone)]
pub struct Ctx {
    pub user_id: i32
}

//constructor
impl Ctx {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

//property accessors
impl Ctx {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}