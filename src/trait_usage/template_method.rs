use async_trait::async_trait;
use std::error::Error;

#[async_trait]
trait Metric: Send {
    type Output;
    type Error: Error;

    async fn refresh_metric(&mut self) -> Result<Self::Output, Self::Error>;
}

#[derive(Default)]
struct StaticMetric;

#[derive(Debug, Default)]
struct MyErr;

impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("I AM ERROR")
    }
}
impl Error for MyErr {}

#[async_trait]
impl Metric for StaticMetric {
    type Output = ();
    type Error = MyErr;

    async fn refresh_metric(&mut self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}


#[derive(Default)]
struct StaticMetricTwo;

#[async_trait]
impl Metric for StaticMetricTwo {
    type Output = ();
    type Error = MyErr;

    async fn refresh_metric(&mut self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}


struct LocalSystemData<T> {
    inner: T,
}

impl<T> LocalSystemData<T>
where
    T: Metric,
    <T as Metric>::Error: 'static,
{
    fn new(inner: T) -> LocalSystemData<T> {
        LocalSystemData { inner }
    }

    async fn refresh_all(&mut self) -> Result<(), Box<dyn Error>> {
        self.inner.refresh_metric().await?;
        Ok(())
    }
}

// #[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut sys_data = LocalSystemData::new(StaticMetricTwo::default());
    sys_data.refresh_all().await?;

    Ok(())
}
