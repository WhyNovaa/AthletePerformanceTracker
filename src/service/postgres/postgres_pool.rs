use std::sync::Arc;
use sqlx::PgPool;

pub struct DBPool(Arc<PgPool>);


