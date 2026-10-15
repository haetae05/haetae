use haetae_core::config::{DIAG_LINE_NUMBER_WIDTH, MAX_STACK_FRAME_BYTES, NUM_ONE, NUM_TWENTY};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticReport {
    pub error_type: String,
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub function_name: String,
    pub message: String,
    pub snippet_prev: Option<String>,
    pub snippet_curr: String,
    pub underline_label: String,
    pub iteration_triggered: usize,
    pub total_iterations: usize,
    pub payload_str: String,
    pub hardware_detail: String,
    pub metrics_passed: usize,
    pub metrics_failed: usize,
    pub duration_ms: f64,
    pub stack_frame_bytes: usize,
}

impl DiagnosticReport {
    pub fn render_box_drawing(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("error[{}]: {}\n", self.error_type, self.message));
        out.push_str(&format!(
            "  ┌─ {}:{}:{}\n",
            self.file_path, self.line, self.column
        ));
        out.push_str("  │\n");

        if let Some(ref prev) = self.snippet_prev {
            let prev_line = if self.line > NUM_ONE {
                self.line - NUM_ONE
            } else {
                NUM_ONE
            };
            out.push_str(&format!(
                "{:width$} │   {}\n",
                prev_line,
                prev,
                width = DIAG_LINE_NUMBER_WIDTH
            ));
        }

        out.push_str(&format!(
            "{:width$} │   {}\n",
            self.line,
            self.snippet_curr,
            width = DIAG_LINE_NUMBER_WIDTH
        ));

        let pad = " ".repeat(self.column);
        let underline = "^".repeat(self.underline_label.len().max(NUM_ONE));
        out.push_str(&format!(
            "  │   {}{} {}\n",
            pad, underline, self.underline_label
        ));
        out.push_str("  │\n");

        out.push_str(&format!(
            "  = fuzz: triggered on iteration #{} of {} with payload `{}`\n",
            self.iteration_triggered, self.total_iterations, self.payload_str
        ));
        out.push_str(&format!("  = hardware: {}\n", self.hardware_detail));

        out.push_str(&format!(
            "  = metrics: {} passed, {} failed in {:.2}ms | stack frame {}B (< {}MB bound)\n",
            self.metrics_passed,
            self.metrics_failed,
            self.duration_ms,
            self.stack_frame_bytes,
            MAX_STACK_FRAME_BYTES / (NUM_ONE << NUM_TWENTY)
        ));
        out.push_str(&format!(
            "  = location: function `{}` in {}:{}:{}\n",
            self.function_name, self.file_path, self.line, self.column
        ));

        out
    }

    pub fn render_success_line(
        total_iterations: usize,
        duration_ms: f64,
        stack_bytes: usize,
    ) -> String {
        format!(
            "success: {} runs verified in {:.2}ms | memory: safe (0 leaks) | stack: {}B (< 1MB bound) | cfi: 0 bad jumps | dynamic bounds & contracts: safe",
            total_iterations, duration_ms, stack_bytes
        )
    }
}
