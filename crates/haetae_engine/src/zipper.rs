use haetae_core::config::COLLISION_WINDOW_TOKENS;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZipperDecoder {
    pub prefix_tokens: Vec<u32>,
    pub suffix_tokens: Vec<u32>,
    pub collision_window: usize,
    pub converged: bool,
}

impl ZipperDecoder {
    pub fn new(prefix_seed: &[u32], suffix_seed: &[u32], collision_window: usize) -> Self {
        Self {
            prefix_tokens: prefix_seed.to_vec(),
            suffix_tokens: suffix_seed.to_vec(),
            collision_window: collision_window.max(COLLISION_WINDOW_TOKENS),
            converged: false,
        }
    }

    pub fn argmax(logits: &[f32]) -> u32 {
        if logits.is_empty() {
            return 0;
        }
        let mut best_idx = 0;
        let mut best_val = logits[0];

        for (idx, &val) in logits.iter().enumerate() {
            if val > best_val {
                best_val = val;
                best_idx = idx;
            }
        }
        best_idx as u32
    }

    pub fn step_dual_tokens(
        &mut self,
        logits_forward: &[f32],
        logits_backward: &[f32],
    ) -> (u32, u32) {
        if self.converged {
            return (0, 0);
        }

        let token_fwd = Self::argmax(logits_forward);
        let token_bwd = Self::argmax(logits_backward);

        self.prefix_tokens.push(token_fwd);
        self.suffix_tokens.insert(0, token_bwd);

        if self.check_collision() {
            self.converged = true;
        }

        (token_fwd, token_bwd)
    }

    pub fn check_collision(&self) -> bool {
        if self.prefix_tokens.is_empty() || self.suffix_tokens.is_empty() {
            return false;
        }

        let p_len = self.prefix_tokens.len();
        let s_len = self.suffix_tokens.len();

        let window = self.collision_window.min(p_len).min(s_len);
        for w in 1..=window {
            let p_slice = &self.prefix_tokens[p_len - w..p_len];
            let s_slice = &self.suffix_tokens[0..w];
            if p_slice == s_slice {
                return true;
            }
        }

        false
    }

    pub fn stitch_sequence(&self) -> Vec<u32> {
        let mut result = self.prefix_tokens.clone();
        let p_len = self.prefix_tokens.len();
        let s_len = self.suffix_tokens.len();

        let mut overlap = 0;
        let window = self.collision_window.min(p_len).min(s_len);
        for w in (1..=window).rev() {
            if self.prefix_tokens[p_len - w..p_len] == self.suffix_tokens[0..w] {
                overlap = w;
                break;
            }
        }

        result.extend_from_slice(&self.suffix_tokens[overlap..]);
        result
    }
}

impl Default for ZipperDecoder {
    fn default() -> Self {
        Self::new(&[], &[], COLLISION_WINDOW_TOKENS)
    }
}
