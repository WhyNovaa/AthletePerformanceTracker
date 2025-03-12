use std::any::TypeId;
use std::env;
use sqlx::{PgPool, Row};
use crate::models::metrics::biathlon::Biathlon;
use crate::models::metrics::running::Running;
use crate::models::metrics::weightlifting::WeightLifting;
use crate::models::performance_tracker::PerformanceTracker;
use crate::models::sportsman::Sportsman;
use crate::traits::traits::{Metric, Pool};

pub struct DBPool(PgPool);

impl DBPool {
    pub async fn new() -> Self {
        let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not found in .env file");
        let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found in .env file");
        let db = env::var("POSTGRES_DB").expect("POSTGRES_DB not found in .env file");

        let database_url = format!("postgres://{}:{}@pg:5432/{}", user, password, db);

        let connection = PgPool::connect(database_url.as_str()).await.expect("Connection error");

        Self(connection)
    }

}

impl DBPool {
    pub async fn add_sportsman(&self, sportsman: &Sportsman) -> Result<(), sqlx::Error> {
        let req = "INSERT INTO Sportsmen(name) VALUES($1);";

        let _ = sqlx::query(req)
            .bind(sportsman.name())
            .execute(&self.0)
            .await?;

        Ok(())
    }
    pub async fn get_sportsman_id(&self, sportsman: &Sportsman) -> Result<i32, sqlx::Error> {
        let req = "SELECT id FROM Sportsmen WHERE name=$1";

        let row = sqlx::query(req)
            .bind(sportsman.name())
            .fetch_one(&self.0)
            .await?;

        let id: i32 = row.try_get("id")?;

        Ok(id)
    }

    // TODO! need tests
    pub async fn if_metric_exists<T: Metric>(&self, sportsman_id: i32) -> Result<bool, sqlx::Error> {
        let db_name = match TypeId::of::<T>() {
            id if id == TypeId::of::<Running>() => {
                "Running"
            }
            id if id == TypeId::of::<Biathlon>() => {
                "Biathlon"
            }
            id if id == TypeId::of::<WeightLifting>() => {
                "WeightLifting"
            }
            _ => return Err(sqlx::error::Error::TypeNotFound { type_name: "Unknown metric".to_string() }),
        };

        let req = format!("SELECT FROM {} WHERE sportsman_id=$1", db_name).as_str();

        let res = sqlx::query(req)
            .bind(sportsman_id)
            .fetch_optional(&self.0)
            .await?;

        Ok(res.is_some())
    }

    // TODO! need tests
    pub async fn remove_metric<T: Metric>(&self, sportsman_id: i32) -> Result<(), sqlx::Error> {
        let db_name = match TypeId::of::<T>() {
            id if id == TypeId::of::<Running>() => {
                "Running"
            }
            id if id == TypeId::of::<Biathlon>() => {
                "Biathlon"
            }
            id if id == TypeId::of::<WeightLifting>() => {
                "WeightLifting"
            }
            _ => return Err(sqlx::error::Error::TypeNotFound { type_name: "Unknown metric".to_string() }),
        };

        let req = format!("DELETE FROM {} WHERE sportsman_id=$1", db_name).as_str();

        let _ = sqlx::query(req)
            .bind(sportsman_id)
            .execute(&self.0)
            .await?;

        Ok(())
    }
}
impl Pool for DBPool {
    // TODO!
    async fn add_performance(&self, sportsman: &Sportsman, metric: Box<dyn Metric>) -> Result<(), sqlx::Error> {
        match metric.as_any().type_id() {
            id if id == TypeId::of::<Running>() => {
                let down_casted = metric.as_any().downcast_ref::<Running>().expect("Error while casting dyn Metric into Running");

                let id = self.get_sportsman_id(sportsman)?;



            }
            id if id == TypeId::of::<Biathlon>() => {
                let down_casted = metric.as_any().downcast_ref::<Biathlon>().expect("Error while casting dyn Metric into Biathlon");

            }
            id if id == TypeId::of::<WeightLifting>() => {
                let down_casted = metric.as_any().downcast_ref::<WeightLifting>().expect("Error while casting dyn Metric into WeightLifting");

            }
            _ => return Err(sqlx::error::Error::TypeNotFound { type_name: "Unknown metric".to_string() }),
        }

        Ok(())
    }

    async fn get_performance<T: Metric>(&self, sportsman: &Sportsman) -> Result<(), sqlx::Error> {
        todo!()
    }

    async fn remove_performance<T: Metric>(&self, sportsman: &Sportsman) -> Result<(), sqlx::Error> {
        todo!()
    }

    async fn load_performance_tracker(&self) -> PerformanceTracker {
        todo!()
    }
}
