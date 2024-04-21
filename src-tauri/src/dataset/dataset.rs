#[taurpc::ipc_type]
pub struct TestingDataset {
    pub name: String,
    pub dataset_name: String,
    pub accuracy: f32,
}

#[taurpc::ipc_type]
pub struct TrainingDataset {
    pub name: String,
    pub data_amount: u16,
    pub feature_count: u16,
}

#[taurpc::ipc_type]
pub struct GeneralDataset {
    pub name: String,
    pub label_amount: u16,
    pub data_amount: u16,
}

#[taurpc::ipc_type]
pub struct Dataset {
    pub name: String,
    pub labels: Vec<Label>,
}

#[taurpc::ipc_type]
pub struct Label {
    pub name: String,
    pub data: Vec<String>,
    pub is_preprocessed: bool,
}

#[taurpc::ipc_type]
pub struct ProgressPayload {
    pub name: String,
    pub label: String,
    pub current_amount: u16,
    pub total_amount: u16,
}
