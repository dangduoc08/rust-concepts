#[derive(Debug)]
pub enum SyncStatus {
    Synced,
    Unsynced,
}

#[derive(Debug)]
pub enum ProductVariation {
    SyncPrice(SyncStatus),
    SyncStock(SyncStatus),
}
