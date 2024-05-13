use tonic::include_file_descriptor_set;

tonic::include_proto!("market_data_service");

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = include_file_descriptor_set!("service_descriptor");
