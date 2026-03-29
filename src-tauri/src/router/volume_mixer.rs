use std::collections::HashMap;

/// 音量混合器 - 管理每个设备的音量
pub struct VolumeMixer {
    device_volumes: HashMap<String, f32>,
    sync_mode: bool,
    master_device: Option<String>,
}

impl VolumeMixer {
    pub fn new() -> Self {
        Self {
            device_volumes: HashMap::new(),
            sync_mode: false,
            master_device: None,
        }
    }
    
    /// 设置设备音量
    pub fn set_volume(&mut self, device_id: &str, volume: f32) {
        let clamped_volume = volume.clamp(0.0, 1.0);
        self.device_volumes.insert(device_id.to_string(), clamped_volume);
        
        // 如果是同步模式且这是主设备，同步到所有设备
        if self.sync_mode && Some(device_id) == self.master_device.as_deref() {
            self.sync_to_all(clamped_volume);
        }
    }
    
    /// 获取设备音量
    pub fn get_volume(&self, device_id: &str) -> f32 {
        self.device_volumes.get(device_id).copied().unwrap_or(1.0)
    }
    
    /// 应用音量到音频数据
    pub fn apply_volume(&self, device_id: &str, samples: &mut [f32]) {
        if let Some(&volume) = self.device_volumes.get(device_id) {
            for sample in samples.iter_mut() {
                *sample *= volume;
            }
        }
    }
    
    /// 设置同步模式
    pub fn set_sync_mode(&mut self, enabled: bool) {
        self.sync_mode = enabled;
    }
    
    /// 设置主设备
    pub fn set_master_device(&mut self, device_id: Option<String>) {
        self.master_device = device_id;
    }
    
    /// 获取主设备ID
    pub fn master_device(&self) -> Option<&str> {
        self.master_device.as_deref()
    }
    
    /// 从主设备同步音量到所有设备
    pub fn sync_from_master(&mut self) {
        if let Some(master) = &self.master_device {
            if let Some(&master_vol) = self.device_volumes.get(master) {
                self.sync_to_all(master_vol);
            }
        }
    }
    
    /// 同步音量到所有设备
    fn sync_to_all(&mut self, volume: f32) {
        for vol in self.device_volumes.values_mut() {
            *vol = volume;
        }
    }
    
    /// 添加设备
    pub fn add_device(&mut self, device_id: &str, volume: f32) {
        self.device_volumes.insert(device_id.to_string(), volume);
    }
    
    /// 移除设备
    pub fn remove_device(&mut self, device_id: &str) {
        self.device_volumes.remove(device_id);
        if self.master_device.as_deref() == Some(device_id) {
            self.master_device = self.device_volumes.keys().next().cloned();
        }
    }
    
    /// 清空所有设备
    pub fn clear(&mut self) {
        self.device_volumes.clear();
        self.master_device = None;
    }
    
    /// 获取所有设备音量
    pub fn all_volumes(&self) -> &HashMap<String, f32> {
        &self.device_volumes
    }
    
    /// 是否为同步模式
    pub fn is_sync_mode(&self) -> bool {
        self.sync_mode
    }
}

impl Default for VolumeMixer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_volume_mixer() {
        let mut mixer = VolumeMixer::new();
        
        mixer.add_device("device1", 0.8);
        mixer.add_device("device2", 0.6);
        
        assert_eq!(mixer.get_volume("device1"), 0.8);
        assert_eq!(mixer.get_volume("device2"), 0.6);
        
        // 测试应用音量
        let mut samples = vec![1.0, 1.0, 1.0];
        mixer.apply_volume("device1", &mut samples);
        assert_eq!(samples, vec![0.8, 0.8, 0.8]);
    }
    
    #[test]
    fn test_sync_mode() {
        let mut mixer = VolumeMixer::new();
        mixer.set_sync_mode(true);
        mixer.set_master_device(Some("device1".to_string()));
        
        mixer.add_device("device1", 0.8);
        mixer.add_device("device2", 0.6);
        
        // 修改主设备音量，应该同步到所有设备
        mixer.set_volume("device1", 0.5);
        
        assert_eq!(mixer.get_volume("device1"), 0.5);
        assert_eq!(mixer.get_volume("device2"), 0.5);
    }
}
