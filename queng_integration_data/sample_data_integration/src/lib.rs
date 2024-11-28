use data_integration_traits::DataIntegrationFactory;
use std::io::Error;

pub struct SampleDataIntegration {}

impl DataIntegrationFactory for SampleDataIntegration {
    async fn start_date(&self, _data_id: &str) -> Result<(), Error> {
        println!("SampleDataIntegration/start_date");

        Ok(())
    }

    async fn stop_date(&self, _data_id: &str) -> Result<(), Error> {
        println!("SampleDataIntegration/stop_date");

        Ok(())
    }

    async fn stop_all_date(&self) -> Result<(), Error> {
        println!("SampleDataIntegration/stop_all_date");

        Ok(())
    }
}
