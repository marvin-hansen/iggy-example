use enum_dispatch::enum_dispatch;
use sample_data_integration::SampleDataIntegration;

#[enum_dispatch]
enum DataIntegrationFactory {
    SampleDataIntegration,
}
