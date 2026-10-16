var __getOwnPropNames = Object.getOwnPropertyNames;
var __commonJS = (cb, mod) => function __require() {
  try {
    return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
  } catch (e) {
    throw mod = 0, e;
  }
};

// config.js
var require_config = __commonJS({
  "config.js"(exports2, module2) {
    module2.exports = {
      STATUS_BAR_PRIORITY_HAETAE: 100,
      STATUS_BAR_PRIORITY_RESEARCH: 99,
      STATUS_BAR_PRIORITY_VERIFY: 98,
      STATUS_BAR_MESSAGE_TIMEOUT_MS: 3e3,
      EXEC_MAX_BUFFER_BYTES: 10485760,
      CHMOD_EXECUTABLE_MODE: 493,
      HTTP_OK: 200,
      OLLAMA_DEFAULT_HOST: "http://127.0.0.1:11434",
      OLLAMA_PROBE_TIMEOUT_MS: 1500,
      OLLAMA_BOOTSTRAP_POLL_INTERVAL_MS: 500,
      OLLAMA_BOOTSTRAP_MAX_ATTEMPTS: 20,
      OLLAMA_DOWNLOAD_URL: "https://ollama.com/download",
      OLLAMA_WINGET_INSTALL_CMD: "winget install --id Ollama.Ollama -e",
      OLLAMA_BREW_INSTALL_CMD: "brew install --cask ollama",
      OLLAMA_CURL_INSTALL_CMD: "curl -fsSL https://ollama.com/install.sh | sh",
      HAETAE_WIN_BIN_NAME: "haetae.exe",
      HAETAE_UNIX_BIN_NAME: "haetae",
      DEFAULT_LANGUAGE: "c",
      CLI_MAX_PREFIX_CHARS: 8192,
      CLI_MAX_SUFFIX_CHARS: 4096,
      RADIX_BASE_36: 36,
      TEMP_RANDOM_SLICE_START: 2
    };
  }
});

// extension.original.js
var vscode = require("vscode");
var cp = require("child_process");
var fs = require("fs");
var path = require("path");
var http = require("http");
var os = require("os");
var CONFIG = require_config();
var bundledBin = process.platform === "win32" ? path.join(__dirname, "bin", CONFIG.HAETAE_WIN_BIN_NAME) : path.join(__dirname, "bin", CONFIG.HAETAE_UNIX_BIN_NAME);
var userCargoBin = process.env.USERPROFILE ? path.join(process.env.USERPROFILE, ".cargo", "bin", CONFIG.HAETAE_WIN_BIN_NAME) : path.join(process.env.HOME || "", ".cargo", "bin", CONFIG.HAETAE_UNIX_BIN_NAME);
var globalResearchEnabled = false;
var activeRefactorThread = null;
function getHaetaeBinary() {
  const configPath = vscode.workspace.getConfiguration("haetae").get("binaryPath");
  if (configPath && typeof configPath === "string" && configPath.trim().length > 0) {
    const resolved = path.isAbsolute(configPath) ? configPath : path.resolve(configPath);
    if (fs.existsSync(resolved)) {
      return resolved;
    }
  }
  const candidatePaths = [
    bundledBin,
    path.join(__dirname, "bin", process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME),
    userCargoBin,
    path.join(__dirname, "..", "01_\uBC14\uC774\uB108\uB9AC_\uC2E4\uD589\uD30C\uC77C", process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME),
    path.join(__dirname, "..", "03_\uB7EC\uC2A4\uD2B8_\uC804\uCCB4\uC18C\uC2A4\uCF54\uB4DC", "target", "release", process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME),
    path.join(__dirname, "..", "03_\uB7EC\uC2A4\uD2B8_\uC804\uCCB4\uC18C\uC2A4\uCF54\uB4DC", "target", "debug", process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME),
    path.join(__dirname, "..", "target", "release", process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME),
    path.join(__dirname, "..", "target", "debug", process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME),
    path.join("/usr", "local", "bin", CONFIG.HAETAE_UNIX_BIN_NAME),
    path.join("/usr", "bin", CONFIG.HAETAE_UNIX_BIN_NAME),
    process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME
  ];
  for (const bin of candidatePaths) {
    if (bin && fs.existsSync(bin)) {
      if (process.platform !== "win32" && bin.startsWith(__dirname)) {
        try {
          fs.chmodSync(bin, CONFIG.CHMOD_EXECUTABLE_MODE);
        } catch {
        }
      }
      return bin;
    }
  }
  return process.platform === "win32" ? CONFIG.HAETAE_WIN_BIN_NAME : CONFIG.HAETAE_UNIX_BIN_NAME;
}
function runHaetaeCommand(args, inputData = null) {
  return new Promise((resolve) => {
    const bin = getHaetaeBinary();
    const child = cp.execFile(bin, args, { maxBuffer: CONFIG.EXEC_MAX_BUFFER_BYTES }, (err, stdout, stderr) => {
      if (err) {
        const exitCode = typeof err.code === "number" ? err.code : 1;
        resolve({ code: exitCode, stdout: stdout || "", stderr: stderr || err.message });
      } else {
        resolve({ code: 0, stdout: stdout || "", stderr: stderr || "" });
      }
    });
    if (inputData && child.stdin) {
      child.stdin.end(inputData);
    }
  });
}
function checkOllamaReachable(host = null) {
  const configuredHost = host || vscode.workspace.getConfiguration("haetae").get("ollamaHost") || CONFIG.OLLAMA_DEFAULT_HOST;
  return new Promise((resolve) => {
    try {
      const parsed = new URL(configuredHost);
      const req = http.get({
        hostname: parsed.hostname,
        port: parsed.port,
        path: "/",
        timeout: CONFIG.OLLAMA_PROBE_TIMEOUT_MS
      }, (res) => {
        let data = "";
        res.on("data", (chunk) => {
          data += chunk;
        });
        res.on("end", () => {
          resolve(res.statusCode === CONFIG.HTTP_OK && data.includes("Ollama is running"));
        });
      });
      req.on("error", () => resolve(false));
      req.on("timeout", () => {
        req.destroy();
        resolve(false);
      });
    } catch {
      resolve(false);
    }
  });
}
function findOllamaBinary() {
  if (process.platform === "win32") {
    const localApp = process.env.LOCALAPPDATA;
    if (localApp) {
      const p1 = path.join(localApp, "Programs", "Ollama", "ollama.exe");
      if (fs.existsSync(p1)) return p1;
    }
    const progFiles = process.env.ProgramFiles;
    if (progFiles) {
      const p2 = path.join(progFiles, "Ollama", "ollama.exe");
      if (fs.existsSync(p2)) return p2;
    }
    try {
      const res = cp.execSync("where.exe ollama", { stdio: "pipe" }).toString().trim().split("\r\n")[0];
      if (res && fs.existsSync(res)) return res;
    } catch {
    }
  } else {
    const candidates = process.platform === "darwin" ? [
      "/Applications/Ollama.app/Contents/Resources/ollama",
      path.join("/opt", "homebrew", "bin", "ollama"),
      path.join("/usr", "local", "bin", "ollama"),
      path.join("/usr", "bin", "ollama")
    ] : [
      path.join("/usr", "local", "bin", "ollama"),
      path.join("/usr", "bin", "ollama"),
      path.join("/bin", "ollama"),
      path.join(process.env.HOME || "", ".local", "bin", "ollama")
    ];
    for (const c of candidates) {
      if (fs.existsSync(c)) return c;
    }
    try {
      const res = cp.execSync("which ollama", { stdio: "pipe" }).toString().trim();
      if (res && fs.existsSync(res)) return res;
    } catch {
    }
  }
  return null;
}
async function startOllamaDaemon(binaryPath) {
  if (!binaryPath) return false;
  try {
    if (process.platform === "darwin" && fs.existsSync("/Applications/Ollama.app")) {
      cp.spawn("open", ["-g", "-a", "/Applications/Ollama.app"], {
        detached: true,
        stdio: "ignore"
      }).unref();
    } else {
      cp.spawn(binaryPath, ["serve"], {
        detached: true,
        stdio: "ignore",
        windowsHide: true
      }).unref();
    }
  } catch {
    return false;
  }
  for (let i = 0; i < CONFIG.OLLAMA_BOOTSTRAP_MAX_ATTEMPTS; i++) {
    await new Promise((r) => setTimeout(r, CONFIG.OLLAMA_BOOTSTRAP_POLL_INTERVAL_MS));
    if (await checkOllamaReachable()) {
      return true;
    }
  }
  return false;
}
async function ensureOllama(promptIfMissing = true) {
  if (await checkOllamaReachable()) {
    return true;
  }
  const bin = findOllamaBinary();
  if (bin) {
    vscode.window.setStatusBarMessage("$(sync~spin) Haetae: Starting Ollama background daemon...", CONFIG.STATUS_BAR_MESSAGE_TIMEOUT_MS);
    const started = await startOllamaDaemon(bin);
    if (started) {
      vscode.window.showInformationMessage("Haetae: Local Ollama AI daemon started successfully.");
      return true;
    }
  }
  if (!promptIfMissing) {
    return false;
  }
  const choice = await vscode.window.showInformationMessage(
    "Haetae: Local AI synthesis requires Ollama. Would you like to set it up now?",
    "Install Ollama (1-Click)",
    "Open Ollama Website",
    "Dismiss"
  );
  if (choice === "Install Ollama (1-Click)") {
    const terminal = vscode.window.createTerminal("Install Ollama");
    terminal.show();
    const installCmd = process.platform === "win32" ? CONFIG.OLLAMA_WINGET_INSTALL_CMD : process.platform === "darwin" ? CONFIG.OLLAMA_BREW_INSTALL_CMD : CONFIG.OLLAMA_CURL_INSTALL_CMD;
    terminal.sendText(installCmd);
    vscode.window.showInformationMessage(`Haetae: Running installer in terminal: ${installCmd}`);
  } else if (choice === "Open Ollama Website") {
    vscode.env.openExternal(vscode.Uri.parse(CONFIG.OLLAMA_DOWNLOAD_URL));
  }
  return false;
}
function cleanCodeOutput(raw) {
  if (!raw || typeof raw !== "string") {
    return "";
  }
  let text = raw.trim();
  const synthMarker = "Synthesized Code:";
  if (text.includes(synthMarker)) {
    const parts = text.split(synthMarker);
    text = parts[parts.length - 1].trim();
  }
  const refactorMarker = "Refactored Code:";
  if (text.includes(refactorMarker)) {
    const parts = text.split(refactorMarker);
    text = parts[parts.length - 1].trim();
  }
  if (text.startsWith("```")) {
    const lines = text.split("\n");
    lines.shift();
    if (lines.length > 0 && lines[lines.length - 1].trim().startsWith("```")) {
      lines.pop();
    }
    text = lines.join("\n").trim();
  }
  return text;
}
var HaetaeComment = class {
  constructor(body, author, parentThread) {
    this.body = body;
    this.mode = vscode.CommentMode.Preview;
    this.author = { name: author };
    this.parentThread = parentThread;
  }
};
function activate(context) {
  const diagChannel = vscode.window.createOutputChannel("Haetae Diagnostics");
  context.subscriptions.push(diagChannel);
  const mainStatusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    CONFIG.STATUS_BAR_PRIORITY_HAETAE
  );
  mainStatusBarItem.text = "$(shield) Haetae [Ctrl+I]";
  mainStatusBarItem.tooltip = "Haetae Contract Engine (Active) | Alt+C: Comment | Alt+V: Verify";
  mainStatusBarItem.command = "haetae.inline";
  mainStatusBarItem.show();
  context.subscriptions.push(mainStatusBarItem);
  const researchStatusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    CONFIG.STATUS_BAR_PRIORITY_RESEARCH
  );
  researchStatusBarItem.text = "$(search) Research [OFF]";
  researchStatusBarItem.tooltip = "Toggle Haetae Web Research (s.jina.ai) [Alt+R]";
  researchStatusBarItem.command = "haetae.toggleResearch";
  researchStatusBarItem.show();
  context.subscriptions.push(researchStatusBarItem);
  const verifyStatusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    CONFIG.STATUS_BAR_PRIORITY_VERIFY
  );
  verifyStatusBarItem.text = "$(beaker) Verify";
  verifyStatusBarItem.tooltip = "Haetae: Run In-Memory Micro-Fuzz & CFI Verification (Alt+V)";
  verifyStatusBarItem.command = "haetae.verify";
  verifyStatusBarItem.show();
  context.subscriptions.push(verifyStatusBarItem);
  const updateResearchUI = () => {
    if (globalResearchEnabled) {
      researchStatusBarItem.text = "$(globe) Research: ON";
      researchStatusBarItem.backgroundColor = void 0;
      researchStatusBarItem.tooltip = "Web Research: ON (Click to toggle)";
    } else {
      researchStatusBarItem.text = "$(search) Research: OFF";
      researchStatusBarItem.backgroundColor = void 0;
      researchStatusBarItem.tooltip = "Web Research: OFF (Click to toggle)";
    }
  };
  const ghostDecoration = vscode.window.createTextEditorDecorationType({
    after: {
      contentText: "  Alt+C: Comment Refactor",
      color: new vscode.ThemeColor("editorCodeLens.foreground"),
      fontStyle: "italic"
    }
  });
  context.subscriptions.push(ghostDecoration);
  vscode.window.onDidChangeTextEditorSelection((e) => {
    if (e.textEditor && !e.selections[0].isEmpty) {
      const range = new vscode.Range(e.selections[0].end, e.selections[0].end);
      e.textEditor.setDecorations(ghostDecoration, [range]);
    } else if (e.textEditor) {
      e.textEditor.setDecorations(ghostDecoration, []);
    }
  }, null, context.subscriptions);
  const commentController = vscode.comments.createCommentController("haetae-comments", "Haetae Contract Review");
  context.subscriptions.push(commentController);
  commentController.commentingRangeProvider = {
    provideCommentingRanges(document) {
      return [new vscode.Range(0, 0, Math.max(document.lineCount - 1, 0), 0)];
    }
  };
  const toggleResearchCommand = vscode.commands.registerCommand("haetae.toggleResearch", () => {
    globalResearchEnabled = !globalResearchEnabled;
    updateResearchUI();
    const stateStr = globalResearchEnabled ? "Enabled (s.jina.ai)" : "Disabled";
    vscode.window.showInformationMessage(`Haetae Web Research: ${stateStr}`);
  });
  context.subscriptions.push(toggleResearchCommand);
  const applyCommentCommand = vscode.commands.registerCommand("haetae.applyComment", async (arg) => {
    const targetThread = arg && arg.haetaeRefactorCode ? arg : arg && arg.parentThread && arg.parentThread.haetaeRefactorCode ? arg.parentThread : activeRefactorThread;
    if (!targetThread || !targetThread.haetaeRefactorCode) {
      vscode.window.showWarningMessage("Haetae: No active refactor proposal to apply.");
      return;
    }
    const editor = vscode.window.visibleTextEditors.find(
      (ed) => ed.document.uri.toString() === targetThread.haetaeUri.toString()
    ) || vscode.window.activeTextEditor;
    if (editor) {
      await editor.edit((eb) => {
        eb.replace(targetThread.haetaeSelection, targetThread.haetaeRefactorCode);
      });
      vscode.window.showInformationMessage("Haetae: Refactored code applied successfully.");
    }
    targetThread.dispose();
    if (activeRefactorThread === targetThread) {
      activeRefactorThread = null;
    }
  });
  context.subscriptions.push(applyCommentCommand);
  const discardCommentCommand = vscode.commands.registerCommand("haetae.discardComment", async (arg) => {
    const targetThread = arg && arg.dispose ? arg : arg && arg.parentThread && arg.parentThread.dispose ? arg.parentThread : activeRefactorThread;
    if (targetThread) {
      targetThread.dispose();
      if (activeRefactorThread === targetThread) {
        activeRefactorThread = null;
      }
      vscode.window.showInformationMessage("Haetae: Refactor proposal dismissed.");
    }
  });
  context.subscriptions.push(discardCommentCommand);
  const inlineCommand = vscode.commands.registerCommand("haetae.inline", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showWarningMessage("Haetae: No active editor found.");
      return;
    }
    const inputBox = vscode.window.createInputBox();
    const refreshInputBox = () => {
      if (globalResearchEnabled) {
        inputBox.title = "Haetae: Inline Code Synthesis (Ctrl+I) [Web Research: ON]";
        inputBox.placeholder = "Describe code to synthesize... (Web Research: ON)";
        inputBox.buttons = [
          {
            iconPath: new vscode.ThemeIcon("globe"),
            tooltip: "Web Research: ON (Click to toggle)"
          }
        ];
      } else {
        inputBox.title = "Haetae: Inline Code Synthesis (Ctrl+I) [Web Research: OFF]";
        inputBox.placeholder = "Describe code to synthesize... (Web Research: OFF)";
        inputBox.buttons = [
          {
            iconPath: new vscode.ThemeIcon("search"),
            tooltip: "Web Research: OFF (Click to toggle)"
          }
        ];
      }
    };
    refreshInputBox();
    inputBox.onDidTriggerButton(() => {
      globalResearchEnabled = !globalResearchEnabled;
      updateResearchUI();
      refreshInputBox();
    });
    inputBox.onDidAccept(async () => {
      const prompt = inputBox.value.trim();
      inputBox.hide();
      if (!prompt) {
        return;
      }
      const configuredDefaultLang = vscode.workspace.getConfiguration("haetae").get("defaultLanguage") || CONFIG.DEFAULT_LANGUAGE;
      const docLang = editor.document.languageId || configuredDefaultLang;
      const filePath = editor.document.uri.fsPath;
      const curPos = editor.selection.active;
      const fullPrefix = editor.document.getText(new vscode.Range(new vscode.Position(0, 0), curPos));
      const fullSuffix = editor.document.getText(new vscode.Range(curPos, new vscode.Position(editor.document.lineCount, 0)));
      const safePrefix = fullPrefix.length > CONFIG.CLI_MAX_PREFIX_CHARS ? fullPrefix.slice(-CONFIG.CLI_MAX_PREFIX_CHARS) : fullPrefix;
      const safeSuffix = fullSuffix.length > CONFIG.CLI_MAX_SUFFIX_CHARS ? fullSuffix.slice(0, CONFIG.CLI_MAX_SUFFIX_CHARS) : fullSuffix;
      const tempInlineFile = path.join(
        os.tmpdir(),
        `haetae_inline_${Date.now()}_${Math.random().toString(CONFIG.RADIX_BASE_36).slice(CONFIG.TEMP_RANDOM_SLICE_START)}.tmp`
      );
      fs.writeFileSync(tempInlineFile, editor.document.getText(), "utf8");
      const cliArgs = [
        "inline",
        prompt,
        "--lang",
        docLang,
        "--file",
        fs.existsSync(filePath) ? filePath : tempInlineFile,
        "--prefix",
        safePrefix,
        "--suffix",
        safeSuffix
      ];
      if (globalResearchEnabled) {
        cliArgs.push("--research");
      }
      await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: globalResearchEnabled ? "Haetae: Synthesizing code with Web Research (s.jina.ai)..." : "Haetae: Synthesizing code with local model...",
        cancellable: false
      }, async () => {
        try {
          await ensureOllama(false);
          const res = await runHaetaeCommand(cliArgs, editor.document.getText());
          if (res.code !== 0 || !res.stdout || res.stdout.trim() === "") {
            const errMsg = res.stderr || res.stdout || "Inline synthesis failed.";
            if (errMsg.includes("Pro tier") || errMsg.includes("requires an active Pro")) {
              vscode.window.showErrorMessage("Haetae: Inline Code Synthesis requires an active Pro license. Press Ctrl+Alt+A to activate.");
            } else {
              vscode.window.showErrorMessage(`Haetae: ${errMsg.trim()}`);
            }
            return;
          }
          const codeToInsert = cleanCodeOutput(res.stdout);
          if (!codeToInsert || codeToInsert.trim() === "") {
            vscode.window.showWarningMessage("Haetae: No code was generated.");
            return;
          }
          await editor.edit((editBuilder) => {
            editBuilder.insert(editor.selection.active, codeToInsert.trim() + "\n");
          });
          vscode.window.showInformationMessage("Haetae: Synthesized code injected at cursor.");
        } finally {
          try {
            if (fs.existsSync(tempInlineFile)) {
              fs.unlinkSync(tempInlineFile);
            }
          } catch {
          }
        }
      });
    });
    inputBox.show();
  });
  context.subscriptions.push(inlineCommand);
  const commentCommand = vscode.commands.registerCommand("haetae.comment", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.selection.isEmpty) {
      vscode.window.showWarningMessage("Haetae: Select a code block first to open Comment Refactor Card.");
      return;
    }
    const selection = editor.selection;
    const selectedText = editor.document.getText(selection);
    const configuredDefaultLang = vscode.workspace.getConfiguration("haetae").get("defaultLanguage") || CONFIG.DEFAULT_LANGUAGE;
    const docLang = editor.document.languageId || configuredDefaultLang;
    const inputBox = vscode.window.createInputBox();
    const refreshCommentInput = () => {
      if (globalResearchEnabled) {
        inputBox.title = "Haetae: Selection Refactor (Alt+C) [Web Research: ON]";
        inputBox.placeholder = "Enter transformation intent... (Web Research: ON)";
        inputBox.buttons = [
          {
            iconPath: new vscode.ThemeIcon("globe"),
            tooltip: "Web Research: ON (Click to toggle)"
          }
        ];
      } else {
        inputBox.title = "Haetae: Selection Refactor (Alt+C) [Web Research: OFF]";
        inputBox.placeholder = "Enter transformation intent... (Web Research: OFF)";
        inputBox.buttons = [
          {
            iconPath: new vscode.ThemeIcon("search"),
            tooltip: "Web Research: OFF (Click to toggle)"
          }
        ];
      }
    };
    refreshCommentInput();
    inputBox.onDidTriggerButton(() => {
      globalResearchEnabled = !globalResearchEnabled;
      updateResearchUI();
      refreshCommentInput();
    });
    inputBox.onDidAccept(async () => {
      const prompt = inputBox.value.trim();
      inputBox.hide();
      if (!prompt) {
        return;
      }
      const tempTargetFile = path.join(
        os.tmpdir(),
        `haetae_refactor_${Date.now()}_${Math.random().toString(CONFIG.RADIX_BASE_36).slice(CONFIG.TEMP_RANDOM_SLICE_START)}.tmp`
      );
      fs.writeFileSync(tempTargetFile, selectedText, "utf8");
      const cliArgs = [
        "refactor",
        prompt,
        "--lang",
        docLang,
        "--file",
        tempTargetFile
      ];
      if (globalResearchEnabled) {
        cliArgs.push("--research");
      }
      await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: globalResearchEnabled ? "Haetae: Analyzing & Refactoring with Web Research (s.jina.ai)..." : "Haetae: Refactoring code proposal...",
        cancellable: false
      }, async () => {
        try {
          await ensureOllama(false);
          const res = await runHaetaeCommand(cliArgs, selectedText);
          if (res.code !== 0 || !res.stdout || res.stdout.trim() === "") {
            const errMsg = res.stderr || res.stdout || "Refactor execution failed.";
            if (errMsg.includes("Pro tier") || errMsg.includes("requires an active Pro")) {
              vscode.window.showErrorMessage("Haetae: Selection Refactor requires an active Pro license. Press Ctrl+Alt+A to activate.");
            } else {
              vscode.window.showErrorMessage(`Haetae Refactor: ${errMsg.trim()}`);
            }
            return;
          }
          const refactoredCode = cleanCodeOutput(res.stdout);
          if (!refactoredCode || refactoredCode.trim() === "") {
            vscode.window.showWarningMessage("Haetae: No refactor proposal was generated.");
            return;
          }
          const thread = commentController.createCommentThread(editor.document.uri, selection, []);
          thread.haetaeRefactorCode = refactoredCode.trim();
          thread.haetaeSelection = selection;
          thread.haetaeUri = editor.document.uri;
          thread.contextValue = "haetaeRefactorThread";
          activeRefactorThread = thread;
          const researchTag = globalResearchEnabled ? "$(globe) Research: ON" : "$(search) Research: OFF";
          const mdBody = new vscode.MarkdownString();
          mdBody.isTrusted = true;
          mdBody.supportHtml = true;
          mdBody.appendMarkdown(`### Haetae Refactor Proposal
**Instruction**: ${prompt}

`);
          mdBody.appendCodeblock(refactoredCode.trim(), docLang);
          mdBody.appendMarkdown(
            `

[$(check) **Keep (Apply)**](command:haetae.applyComment) &nbsp;&nbsp;&nbsp;&nbsp; [$(discard) **Undo (Dismiss)**](command:haetae.discardComment) &nbsp;&nbsp;&nbsp;&nbsp; [${researchTag}](command:haetae.toggleResearch)
`
          );
          const originalComment = new HaetaeComment(
            mdBody,
            "Haetae Engine",
            thread
          );
          thread.comments = [originalComment];
          thread.collapsibleState = vscode.CommentThreadCollapsibleState.Expanded;
          const action = await vscode.window.showInformationMessage(
            `Haetae: Refactor proposal ready for "${prompt}"`,
            "Keep (Apply)",
            "Undo (Dismiss)"
          );
          if (action === "Keep (Apply)") {
            await vscode.commands.executeCommand("haetae.applyComment", thread);
          } else if (action === "Undo (Dismiss)") {
            await vscode.commands.executeCommand("haetae.discardComment", thread);
          }
        } finally {
          try {
            if (fs.existsSync(tempTargetFile)) {
              fs.unlinkSync(tempTargetFile);
            }
          } catch {
          }
        }
      });
    });
    inputBox.show();
  });
  context.subscriptions.push(commentCommand);
  const verifyCommand = vscode.commands.registerCommand("haetae.verify", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showWarningMessage("Haetae: No active editor to verify.");
      return;
    }
    if (editor.document.isDirty) {
      await editor.document.save();
    }
    const filePath = editor.document.uri.fsPath;
    vscode.window.setStatusBarMessage("$(sync~spin) Haetae: Running in-memory micro-fuzz & CFI verify...", CONFIG.STATUS_BAR_MESSAGE_TIMEOUT_MS);
    const res = await runHaetaeCommand(["verify", filePath]);
    const hasHardwareCrash = res.stdout.includes("error[") || res.stderr.includes("error[");
    const hasProcessFailure = res.code !== 0;
    const hasStderrError = res.stderr && res.stderr.trim().length > 0;
    if (hasHardwareCrash || hasProcessFailure || hasStderrError) {
      verifyStatusBarItem.text = "$(error) Crash";
      diagChannel.clear();
      diagChannel.appendLine("============================================================");
      diagChannel.appendLine("Haetae Micro-Fuzzing Hardware Trap Diagnostic Report");
      diagChannel.appendLine("============================================================");
      if (res.stdout && res.stdout.trim().length > 0) {
        diagChannel.appendLine(res.stdout);
      }
      if (res.stderr && res.stderr.trim().length > 0) {
        diagChannel.appendLine(res.stderr);
      }
      diagChannel.show(true);
      if (hasHardwareCrash) {
        vscode.window.showErrorMessage("Haetae: Hardware trap / Invariant crash detected! See Haetae Diagnostics.");
      } else if (res.stderr.includes("ENOENT") || res.stderr.includes("not found")) {
        vscode.window.showErrorMessage("Haetae: Verification binary not found. Please configure haetae.binaryPath in settings.");
      } else {
        vscode.window.showErrorMessage(`Haetae: Verification failed with exit code ${res.code}. See Haetae Diagnostics.`);
      }
    } else {
      verifyStatusBarItem.text = "$(check) Safe";
      vscode.window.showInformationMessage(`Haetae: ${res.stdout.trim()}`);
    }
  });
  context.subscriptions.push(verifyCommand);
  const boostCommand = vscode.commands.registerCommand("haetae.boost", async () => {
    diagChannel.clear();
    diagChannel.appendLine("Running Haetae /boost Engine & Hardware Pipeline Benchmark...");
    diagChannel.show(true);
    const res = await runHaetaeCommand(["boost"]);
    diagChannel.appendLine(res.stdout);
    if (res.stderr) {
      diagChannel.appendLine(res.stderr);
    }
    vscode.window.showInformationMessage("Haetae: /boost benchmark completed. See Output.");
  });
  context.subscriptions.push(boostCommand);
  const authCommand = vscode.commands.registerCommand("haetae.auth", async () => {
    const key = await vscode.window.showInputBox({
      prompt: "Enter Haetae Pro License Key",
      placeHolder: "e.g. PRO-KEY-ENTERPRISE"
    });
    if (!key || key.trim() === "") return;
    const res = await runHaetaeCommand(["auth", key.trim()]);
    if (res.stdout.includes("License valid!")) {
      vscode.window.showInformationMessage(`Haetae: ${res.stdout.trim()}`);
    } else {
      vscode.window.showErrorMessage(`Haetae Auth Failed: ${res.stdout || res.stderr}`);
    }
  });
  context.subscriptions.push(authCommand);
  const setupOllamaCommand = vscode.commands.registerCommand("haetae.setupOllama", async () => {
    const reachable = await checkOllamaReachable();
    if (reachable) {
      const statusRes = await runHaetaeCommand(["model", "status"]);
      vscode.window.showInformationMessage(`Haetae: Ollama is running.
${statusRes.stdout.trim()}`);
      return;
    }
    const bin = findOllamaBinary();
    if (bin) {
      vscode.window.showInformationMessage(`Haetae: Ollama binary found at ${bin}. Starting background daemon...`);
      const ok = await startOllamaDaemon(bin);
      if (ok) {
        vscode.window.showInformationMessage("Haetae: Ollama background daemon started.");
      } else {
        vscode.window.showErrorMessage("Haetae: Failed to start Ollama background daemon.");
      }
    } else {
      await ensureOllama(true);
    }
  });
  context.subscriptions.push(setupOllamaCommand);
  const customizeShortcutsCommand = vscode.commands.registerCommand("haetae.customizeShortcuts", async () => {
    try {
      await vscode.commands.executeCommand("workbench.action.openGlobalKeybindingsFile");
    } catch (err) {
      vscode.window.showErrorMessage(`Haetae: Failed to open keybindings file: ${err.message}`);
    }
  });
  context.subscriptions.push(customizeShortcutsCommand);
}
function deactivate() {
}
module.exports = {
  activate,
  deactivate
};
