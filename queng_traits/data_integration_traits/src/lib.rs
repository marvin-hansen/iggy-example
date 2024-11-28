use enum_dispatch::enum_dispatch;
use std::io::Error;

#[enum_dispatch(DataIntegration)]
pub trait DataIntegrationFactory {
    fn start_date(&self, data_id: &str) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn stop_date(&self, data_id: &str) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn stop_all_date(&self) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
