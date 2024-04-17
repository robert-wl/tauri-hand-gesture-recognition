#[taurpc::ipc_type]
pub struct Dataset {
    pub name: String,
    pub label_amount: u16,
    pub data_amount: u16,
    pub thumbnail: String,
}

#[taurpc::ipc_type]
pub struct DatasetLabel {
    pub name: String,
    pub data_amount: u16,
    pub thumbnail: String,
}

#[taurpc::ipc_type]
pub struct DatasetData {
    pub name: String,
    pub thumbnail: String,
}