mod delay_buffer;
mod router;
mod volume_mixer;

pub use router::AudioRouter;

use serde::{Deserialize, Serialize};

/// 路由设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterDevice {
    pub id: String,
    pub name: String,
    pub volume: f32,
    pub delay_ms: u32,
    pub enabled: bool,
}

/// 路由配置
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct RouterConfig {
    pub devices: Vec<RouterDevice>,
}

/// 路由状态
#[derive(Clone, Serialize)]
pub struct RouterStatus {
    pub is_running: bool,
    pub source_device_id: Option<String>,
    pub source_device_name: Option<String>,
    pub target_devices: Vec<RouterDevice>,
    pub vb_cable_id: Option<String>,
    pub original_default_device_id: Option<String>,
}

/// VB-Cable 检测结果
#[derive(Clone, Serialize)]
pub struct VirtualDeviceStatus {
    pub is_installed: bool,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
}

/// 验证结果
#[derive(Clone, Serialize)]
pub struct ValidationResult {
    pub has_conflicts: bool,
    pub conflict_devices: Vec<String>,
    pub warning: String,
}
