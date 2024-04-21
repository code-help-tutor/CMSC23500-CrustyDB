WeChat: cstutorcs
QQ: 749389476
Email: tutorcs@163.com
use crate::{StorageManager, TransactionManager};

#[allow(dead_code)] //TODO: remove this
pub struct IndexManager {
    sm: &'static StorageManager,
    tm: &'static TransactionManager,
}

impl IndexManager {
    pub fn new(sm: &'static StorageManager, tm: &'static TransactionManager) -> Self {
        Self { sm, tm }
    }
}
