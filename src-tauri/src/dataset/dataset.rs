

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
}
