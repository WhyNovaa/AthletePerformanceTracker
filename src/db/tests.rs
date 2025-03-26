use mockall::predicate::eq;
use crate::models::metrics::running;
use crate::models::metrics::running::Running;
use crate::models::sportsman::Sportsman;
use crate::traits::traits::{Metric, MockPool, Pool};

async fn setup_mock_db() -> (MockPool, Sportsman) {
    let mut mock_db = MockPool::new();
    let sportsman = Sportsman::unchecked_new("Aboba".to_string());

    mock_db.expect_add_sportsman()
        .with(eq(sportsman.clone()))
        .returning(|_| Ok(()));

    mock_db.add_sportsman(&sportsman).await.unwrap();

    (mock_db, sportsman)
}

fn create_running_metric() -> Box<Running> {
    Box::new(Running::new(running::Distance(1.0), running::Speed(2.0)))
}

#[tokio::test]
async fn add_sportsman_test() -> sqlx::Result<()> {
    let (_mock_db, _sportsman) = setup_mock_db().await;
    Ok(())
}

#[tokio::test]
async fn add_performance_test() -> sqlx::Result<()> {
    let (mut mock_db, sportsman) = setup_mock_db().await;
    let running = create_running_metric();

    mock_db.expect_add_performance()
        .withf({
            let s_cloned = sportsman.clone();
            move |s, m| s == &s_cloned && m.as_any().is::<Running>()
        })
        .returning(|_, _| Ok(()));

    mock_db.add_performance(&sportsman, running).await?;

    Ok(())
}

#[tokio::test]
async fn remove_existed_performance_test() -> sqlx::Result<()> {
    let (mut mock_db, sportsman) = setup_mock_db().await;
    let running = create_running_metric();

    mock_db.expect_add_performance()
        .withf({
            let s_cloned = sportsman.clone();
            move |s, m| s == &s_cloned && m.as_any().is::<Running>()
        })
        .returning(|_, _| Ok(()));

    mock_db.add_performance(&sportsman, running).await?;

    mock_db.expect_remove_performance::<Running>()
        .with(eq(sportsman.clone()))
        .returning(|_| Ok(true));

    let res = mock_db.remove_performance::<Running>(&sportsman).await?;
    assert!(res);

    Ok(())
}

#[tokio::test]
async fn remove_not_existed_performance_test() -> sqlx::Result<()> {
    let (mut mock_db, sportsman) = setup_mock_db().await;

    mock_db.expect_remove_performance::<Running>()
        .with(eq(sportsman.clone()))
        .returning(|_| Ok(false));

    let res = mock_db.remove_performance::<Running>(&sportsman).await?;
    assert!(!res);

    Ok(())
}
