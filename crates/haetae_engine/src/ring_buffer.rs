use haetae_core::config::{
    BYTES_PER_MB, L3_CACHE_BOUND_MB, L3_SAFETY_MARGIN_MB, WEIGHT_SLICE_SIZE_MB,
};

pub const MIN_RING_BUFFER_SLICES: usize = 2;

pub struct WeightRingBuffer {
    pub slice_size_bytes: usize,
    pub total_slices: usize,
    pub current_slice_idx: usize,
    pub active_buffer: Vec<u8>,
    pub prefetch_buffer: Vec<u8>,
}

impl WeightRingBuffer {
    pub fn new(total_slices: usize) -> Self {
        let slice_size_bytes = WEIGHT_SLICE_SIZE_MB * BYTES_PER_MB;
        Self {
            slice_size_bytes,
            total_slices: total_slices.max(MIN_RING_BUFFER_SLICES),
            current_slice_idx: 0,
            active_buffer: vec![0; slice_size_bytes],
            prefetch_buffer: vec![0; slice_size_bytes],
        }
    }

    pub fn is_l3_compliant(&self) -> bool {
        self.slice_size_bytes
            <= (L3_CACHE_BOUND_MB * BYTES_PER_MB + L3_SAFETY_MARGIN_MB * BYTES_PER_MB)
    }

    pub fn load_active_slice(&mut self, data: &[u8]) {
        let copy_len = data.len().min(self.slice_size_bytes);
        self.active_buffer[0..copy_len].copy_from_slice(&data[0..copy_len]);
    }

    pub fn prefetch_next_slice(&mut self, data: &[u8]) {
        let copy_len = data.len().min(self.slice_size_bytes);
        self.prefetch_buffer[0..copy_len].copy_from_slice(&data[0..copy_len]);
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.active_buffer, &mut self.prefetch_buffer);
        self.current_slice_idx = (self.current_slice_idx + 1) % self.total_slices;
    }

    pub fn active_slice(&self) -> &[u8] {
        &self.active_buffer
    }
}
