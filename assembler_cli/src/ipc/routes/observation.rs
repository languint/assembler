use assembler_core::models;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::ipc::schema::IpcSchema;

#[derive(Deserialize, Serialize)]
pub enum ObservationPayloadType {
    QueryRegion,
    Entity,
}

#[derive(Deserialize, Serialize)]
pub struct QueryRegionPayload {
    pub position: models::force::Position,
    pub size: models::force::Position,
}

#[derive(Deserialize, Serialize)]
pub struct EntityPayload {

}

#[derive(Deserialize, Serialize)]
pub struct ObservationPayloadData {
    pub payload_type: ObservationPayloadType,
}

#[derive(Deserialize, Serialize)]
pub struct ObservationPayload {
    pub schema: IpcSchema,
    pub data: ObservationPayloadData,
}

pub struct ObservationState {}

impl ObservationState {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_packet(
    tx_recv: &mpsc::UnboundedSender<String>,
    mut state: tokio::sync::MutexGuard<'_, ObservationState>,
    msg_json: ObservationPayload,
) -> Result<(), String> {
    Ok(())
}