pub const OK: Success = Success { ok: true };
pub const FAIL: Success = Success { ok: false };

#[must_use]
pub struct Success {
    pub ok: bool,
}

impl PartialEq<i32> for Success {
    fn eq(&self, int: &i32) -> bool {
        self.ok == (*int == 1)
    }
}
