pub const OK: Success = Success { ok: true };
pub const FAIL: Success = Success { ok: false };

#[must_use]
pub struct Success {
    pub ok: bool,
}

pub struct Zero;

impl PartialEq<Zero> for Success {
    fn eq(&self, _zero: &Zero) -> bool {
        !self.ok
    }
}
