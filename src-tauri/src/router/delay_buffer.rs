use std::collections::VecDeque;

/// 延迟缓冲区 - 为每个设备提供独立的延迟补偿
pub struct DelayBuffer {
    buffer: VecDeque<f32>,
    delay_samples: usize,
    channels: usize,
}

impl DelayBuffer {
    pub fn new(delay_ms: u32, sample_rate: u32, channels: usize) -> Self {
        let delay_samples = ((delay_ms as f64 * sample_rate as f64 / 1000.0).round() as usize * channels)
            .max(channels);
        
        Self {
            buffer: VecDeque::with_capacity(delay_samples + 1000),
            delay_samples,
            channels,
        }
    }
    
    /// 批量写入音频数据
    pub fn push_slice(&mut self, data: &[f32]) {
        self.buffer.extend(data);
    }
    
    /// 读取一帧音频，延迟期内返回静音
    pub fn pop_or_silent(&mut self) -> Vec<f32> {
        if self.buffer.len() > self.delay_samples {
            let mut frame = Vec::with_capacity(self.channels);
            for _ in 0..self.channels {
                frame.push(self.buffer.pop_front().unwrap_or(0.0));
            }
            frame
        } else {
            vec![0.0; self.channels]
        }
    }
    
    /// 更新延迟设置
    pub fn set_delay(&mut self, delay_ms: u32, sample_rate: u32) {
        let new_delay_samples = ((delay_ms as f64 * sample_rate as f64 / 1000.0).round() as usize * self.channels)
            .max(self.channels);
        self.delay_samples = new_delay_samples;
    }
    
    /// 清空缓冲区
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_delay_buffer() {
        let mut buffer = DelayBuffer::new(100, 48000, 2);
        
        // 推入一些帧
        for i in 0..100 {
            buffer.push_slice(&[i as f32, i as f32]);
        }
        
        // 缓冲区应该有 200 个采样 (100 帧 * 2 通道)
        assert_eq!(buffer.buffer.len(), 200);
    }
}
