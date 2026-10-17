use haetae_core::config::{
    BYTES_PER_MB, COLLISION_WINDOW_TOKENS, DEFAULT_CONTEXT_WINDOW_TOKENS, L3_CACHE_BOUND_MB,
    L3_CACHE_PER_CORE_BOUND_MB,
};

pub type TokenId = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZipperDecoder {
    pub prefix_tokens: Vec<TokenId>,
    pub suffix_tokens: Vec<TokenId>,
    pub collision_window: usize,
    pub max_buffer_tokens: usize,
    pub converged: bool,
}

impl ZipperDecoder {
    pub fn new(prefix_seed: &[TokenId], suffix_seed: &[TokenId], collision_window: usize) -> Self {
        Self {
            prefix_tokens: prefix_seed.to_vec(),
            suffix_tokens: suffix_seed.to_vec(),
            collision_window: collision_window.max(1),
            max_buffer_tokens: DEFAULT_CONTEXT_WINDOW_TOKENS,
            converged: false,
        }
    }

    pub fn with_collision_window(mut self, window: usize) -> Self {
        self.collision_window = window.max(1);
        self
    }

    pub fn with_max_buffer_tokens(mut self, max_tokens: usize) -> Self {
        self.max_buffer_tokens = max_tokens;
        self
    }

    pub fn set_collision_window(&mut self, window: usize) {
        self.collision_window = window.max(1);
    }

    pub fn argmax(logits: &[f32]) -> TokenId {
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
        best_idx as TokenId
    }

    pub fn total_tokens_count(&self) -> usize {
        self.prefix_tokens.len() + self.suffix_tokens.len()
    }

    pub fn is_within_sliding_bounds(&self) -> bool {
        if self.max_buffer_tokens == 0 {
            true
        } else {
            self.total_tokens_count() <= self.max_buffer_tokens
        }
    }

    pub fn enforce_sliding_bounds(&mut self) {
        if self.max_buffer_tokens == 0 {
            return;
        }
        while self.total_tokens_count() > self.max_buffer_tokens {
            if self.prefix_tokens.len() > self.suffix_tokens.len() && !self.prefix_tokens.is_empty()
            {
                self.prefix_tokens.remove(0);
            } else if !self.suffix_tokens.is_empty() {
                self.suffix_tokens.pop();
            } else {
                break;
            }
        }
    }

    pub fn step_token_pair(
        &mut self,
        token_fwd: TokenId,
        token_bwd: TokenId,
    ) -> (TokenId, TokenId) {
        if self.converged {
            return (0, 0);
        }

        self.prefix_tokens.push(token_fwd);
        self.suffix_tokens.insert(0, token_bwd);
        self.enforce_sliding_bounds();

        if self.check_collision() {
            self.converged = true;
        }

        (token_fwd, token_bwd)
    }

    pub fn step_dual_tokens(
        &mut self,
        logits_forward: &[f32],
        logits_backward: &[f32],
    ) -> (TokenId, TokenId) {
        if self.converged {
            return (0, 0);
        }

        let token_fwd = Self::argmax(logits_forward);
        let token_bwd = Self::argmax(logits_backward);

        self.step_token_pair(token_fwd, token_bwd)
    }

    pub fn converge_with_generated_tokens(&mut self, tokens: &[TokenId]) -> Vec<TokenId> {
        for (&tf, &tb) in tokens.iter().zip(tokens.iter().rev()) {
            if self.converged {
                break;
            }
            self.step_token_pair(tf, tb);
        }
        if !self.converged {
            self.converged = self.check_collision();
        }
        self.stitch_sequence()
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

    pub fn stitch_sequence(&self) -> Vec<TokenId> {
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

    pub fn buffer_size_bytes(&self) -> usize {
        self.total_tokens_count() * std::mem::size_of::<TokenId>()
    }

    pub fn is_l3_compliant(&self) -> bool {
        self.is_l3_bound_compliant(L3_CACHE_BOUND_MB)
    }

    pub fn is_l3_bound_compliant(&self, bound_mb: usize) -> bool {
        self.buffer_size_bytes() <= (bound_mb * BYTES_PER_MB)
    }

    pub fn is_per_core_l3_compliant(&self) -> bool {
        self.buffer_size_bytes() < (L3_CACHE_PER_CORE_BOUND_MB * BYTES_PER_MB)
    }
}

impl Default for ZipperDecoder {
    fn default() -> Self {
        Self::new(&[], &[], COLLISION_WINDOW_TOKENS)
    }
}
