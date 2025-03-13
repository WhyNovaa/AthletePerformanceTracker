use crate::models::metrics::biathlon::Biathlon;
use crate::models::metrics::running::Running;
use crate::models::metrics::weight_lifting::WeightLifting;
use crate::models::metrics::{biathlon, running, weight_lifting};
use crate::models::performance_tracker::{Metrics, PerformanceTracker};
use crate::models::sportsman::Sportsman;
use crate::service::models::Id;
use crate::traits::traits::{Metric, Pool};
use sqlx::{PgPool, Row};
use std::any::TypeId;
use std::collections::HashMap;
use std::env;

pub struct DBPool(PgPool);

impl DBPool {
    pub async fn new() -> Self {
        let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not found in .env file");
        let password =
            env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found in .env file");
        let db = env::var("POSTGRES_DB").expect("POSTGRES_DB not found in .env file");

        let database_url = format!("postgres://{}:{}@pg:5432/{}", user, password, db);

        let connection = PgPool::connect(database_url.as_str())
            .await
            .expect("Connection error");

        Self(connection)
    }

    fn get_sportsmen_table_name(&self) -> &'static str {
        "Sportsmen"
    }

    fn get_metric_table_name<T: Metric>(&self) -> Option<&'static str> {
        match TypeId::of::<T>() {
            id if id == TypeId::of::<Running>() => Some("Running"),
            id if id == TypeId::of::<Biathlon>() => Some("Biathlon"),
            id if id == TypeId::of::<WeightLifting>() => Some("WeightLifting"),
            _ => None,
        }
    }

    pub async fn get_all_sportsmen(&self) -> Result<Vec<(Id, Sportsman)>, sqlx::Error> {
        let req = format!("SELECT * FROM {}", self.get_sportsmen_table_name());

        let res = sqlx::query_as::<_, (i32, String)>(req.as_str())
            .fetch_all(&self.0)
            .await?
            .into_iter()
            .map(|(id, name)| (Id(id), Sportsman::new(name)))
            .collect();

        Ok(res)
    }

    pub async fn add_sportsman(&self, sportsman: &Sportsman) -> Result<(), sqlx::Error> {
        let req = format!(
            "INSERT INTO {} (name) VALUES($1);",
            self.get_sportsmen_table_name()
        );

        let _ = sqlx::query(req.as_str())
            .bind(sportsman.name())
            .execute(&self.0)
            .await?;

        Ok(())
    }
    pub async fn get_sportsman_id(&self, sportsman: &Sportsman) -> Result<i32, sqlx::Error> {
        let req = format!(
            "SELECT id FROM {} WHERE name=$1",
            self.get_sportsmen_table_name()
        );

        let row = sqlx::query(req.as_str())
            .bind(sportsman.name())
            .fetch_one(&self.0)
            .await?;

        let id: i32 = row.try_get("id")?;

        Ok(id)
    }

    pub async fn add_metric<T: Metric>(
        &self,
        sportsman_id: i32,
        metric: T,
    ) -> Result<(), sqlx::Error> {
        let table_name = match self.get_metric_table_name::<T>() {
            Some(name) => name,
            None => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Unknown metric".to_string(),
                })
            }
        };

        let req = match TypeId::of::<T>() {
            id if id == TypeId::of::<Running>() => format!(
                "INSERT INTO {} (sportsman_id, distance, speed) VALUES ($1, $2, $3)",
                table_name
            ),
            id if id == TypeId::of::<Biathlon>() => format!(
                "INSERT INTO {} (sportsman_id, accuracy, distance, speed) VALUES ($1, $2, $3, $4)",
                table_name
            ),
            id if id == TypeId::of::<WeightLifting>() => format!(
                "INSERT INTO {} (sportsman_id, weight, lifted_weight) VALUES ($1, $2, $3)",
                table_name
            ),
            _ => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Unknown metric".to_string(),
                })
            }
        };

        let mut query_builder = sqlx::query(req.as_str()).bind(sportsman_id);

        if let Some(running) = metric.as_any().downcast_ref::<Running>() {
            query_builder = query_builder.bind(running.distance.0).bind(running.speed.0);
        } else if let Some(biathlon) = metric.as_any().downcast_ref::<Biathlon>() {
            query_builder = query_builder
                .bind(biathlon.accuracy.0)
                .bind(biathlon.distance.0)
                .bind(biathlon.speed.0);
        } else if let Some(weight_lifting) = metric.as_any().downcast_ref::<WeightLifting>() {
            query_builder = query_builder
                .bind(weight_lifting.weight.0)
                .bind(weight_lifting.lifted_weight.0);
        }

        query_builder.execute(&self.0).await?;
        Ok(())
    }

    // TODO! not working
    pub async fn get_all_metrics<T: Metric>(&self) -> Result<Vec<(Id, Box<dyn Metric>)>, sqlx::Error> {
        let table_name = match self.get_metric_table_name::<T>() {
            Some(name) => name,
            None => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Unknown metric".to_string(),
                })
            }
        };

        let req = format!("SELECT * FROM {}", table_name);

        let res: Vec<(Id, Box<dyn Metric>)> = if TypeId::of::<T>() == TypeId::of::<Running>() {
            sqlx::query_as::<_, (i32, f32, f32)>(req.as_str())
                .fetch_all(&self.0)
                .await?
                .into_iter()
                .map(|(id, dist, speed)| {
                    (
                        Id(id),
                        Running::new(running::Distance(dist), running::Speed(speed)).clone_box(),
                    )
                })
                .collect::<Vec<(Id, Box<dyn Metric>)>>()
        }
        else if TypeId::of::<T>() == TypeId::of::<Biathlon>() {
            sqlx::query_as::<_, (i32, f32, f32, f32)>(req.as_str())
                .fetch_all(&self.0)
                .await?
                .into_iter()
                .map(|(id, accur, dist, speed)| {
                    (
                        Id(id),
                        Biathlon::new(
                            biathlon::Accuracy(accur),
                            biathlon::Distance(dist),
                            biathlon::Speed(speed),
                        ).clone_box(),
                    )
                })
                .collect::<Vec<(Id, Box<dyn Metric + 'static>)>>()
        }
        else if TypeId::of::<T>() == TypeId::of::<WeightLifting>() {
            sqlx::query_as::<_, (i32, f32, f32)>(req.as_str())
                .fetch_all(&self.0)
                .await?
                .into_iter()
                .map(|(id, weight, lifted_weight)| {
                    (
                        Id(id),
                        WeightLifting::new(
                            weight_lifting::Weight(weight),
                            weight_lifting::LiftedWeight(lifted_weight),
                        ).clone_box(),
                    )
                })
                .collect::<Vec<(Id, Box<dyn Metric + 'static>)>>()
        }
        else {
            return Err(sqlx::Error::TypeNotFound {
                type_name: "Unknown metric".to_string(),
            });
        };

        Ok(res)
    }

    pub async fn if_metric_exists<T: Metric>(
        &self,
        sportsman_id: i32,
    ) -> Result<bool, sqlx::Error> {
        let table_name = match self.get_metric_table_name::<T>() {
            Some(name) => name,
            None => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Unknown metric".to_string(),
                })
            }
        };

        let req = format!("SELECT FROM {} WHERE sportsman_id=$1", table_name);

        let res = sqlx::query(req.as_str())
            .bind(sportsman_id)
            .fetch_optional(&self.0)
            .await?;

        Ok(res.is_some())
    }

    pub async fn remove_metric_if_exists<T: Metric>(
        &self,
        sportsman_id: i32,
    ) -> Result<bool, sqlx::Error> {
        let table_name = match self.get_metric_table_name::<T>() {
            Some(name) => name,
            None => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Unknown metric".to_string(),
                })
            }
        };

        let req = format!("DELETE FROM {} WHERE sportsman_id=$1", table_name);

        let res = sqlx::query(req.as_str())
            .bind(sportsman_id)
            .execute(&self.0)
            .await?;

        Ok(res.rows_affected() == 1)
    }
}

impl Pool for DBPool {
    async fn add_performance(
        &self,
        sportsman: &Sportsman,
        metric: Box<dyn Metric>,
    ) -> Result<(), sqlx::Error> {
        let sportsman_id = self.get_sportsman_id(sportsman).await?;

        match metric.as_any().type_id() {
            id if id == TypeId::of::<Running>() => {
                let down_casted = metric
                    .as_any()
                    .downcast_ref::<Running>()
                    .cloned()
                    .expect("Error while casting dyn Metric into Running");

                self.remove_metric_if_exists::<Running>(sportsman_id)
                    .await?;

                self.add_metric::<Running>(sportsman_id, down_casted)
                    .await?;
            }
            id if id == TypeId::of::<Biathlon>() => {
                let down_casted = metric
                    .as_any()
                    .downcast_ref::<Biathlon>()
                    .cloned()
                    .expect("Error while casting dyn Metric into Biathlon");

                self.remove_metric_if_exists::<Biathlon>(sportsman_id)
                    .await?;

                self.add_metric::<Biathlon>(sportsman_id, down_casted)
                    .await?;
            }
            id if id == TypeId::of::<WeightLifting>() => {
                let down_casted = metric
                    .as_any()
                    .downcast_ref::<WeightLifting>()
                    .cloned()
                    .expect("Error while casting dyn Metric into WeightLifting");

                self.remove_metric_if_exists::<WeightLifting>(sportsman_id)
                    .await?;

                self.add_metric::<WeightLifting>(sportsman_id, down_casted)
                    .await?;
            }
            _ => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Unknown metric".to_string(),
                })
            }
        }

        Ok(())
    }

    async fn remove_performance<T: Metric>(
        &self,
        sportsman: &Sportsman,
    ) -> Result<bool, sqlx::Error> {
        let id = self.get_sportsman_id(sportsman).await?;

        self.remove_metric_if_exists::<T>(id).await
    }

    async fn load_performance_tracker(&self) -> Result<PerformanceTracker, sqlx::Error> {
        let sportsmen = self.get_all_sportsmen().await?;

        let running_vec = self.get_all_metrics::<Running>().await?;
        let biathlon_vec = self.get_all_metrics::<Biathlon>().await?;
        let weight_lifting_vec = self.get_all_metrics::<WeightLifting>().await?;

        let mut sportsmen_to_metrics: HashMap<Sportsman, Metrics> = HashMap::new();

        let mut metrics_map: HashMap<Id, Vec<Box<dyn Metric>>> = HashMap::new();

        for (id, metric) in running_vec {
            metrics_map.entry(id).or_insert_with(Vec::new).push(metric);
        }

        for (id, metric) in biathlon_vec {
            metrics_map.entry(id).or_insert_with(Vec::new).push(metric);
        }

        for (id, metric) in weight_lifting_vec {
            metrics_map.entry(id).or_insert_with(Vec::new).push(metric);
        }

        for (id, sportsman) in sportsmen {
            let metrics = metrics_map.remove(&id).unwrap_or_default();
            sportsmen_to_metrics.insert(sportsman, metrics);
        }

        Ok(PerformanceTracker::new(sportsmen_to_metrics))
    }
}
