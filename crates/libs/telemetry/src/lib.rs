use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Telemetry {
    Spatial(SpatialTelemetry),
    Electrical(ElectricalTelemetry),
    Environmental(EnvironmentalTelemetry),
    Relay(RelayTelemetry),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ElectricalTelemetry {
    Amps { tag: String, value: f32 },
    Voltage { tag: String, value: f32 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvironmentalTelemetry {
    Temperature { tag: String, value: f32 },
    Humidity { tag: String, value: f32 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpatialTelemetry {
    pub latitude: f64,
    pub longitude: f64,
    pub velocity: f32,
    pub satellites: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelayTelemetry {
    pub tag: String,
    pub status: bool
}