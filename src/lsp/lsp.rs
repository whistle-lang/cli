use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct WhistleBackend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for WhistleBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Whistle language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "workspace folders changed!")
    //         .await;
    // }

    // async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "configuration changed!")
    //         .await;
    // }

    // async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "watched files have changed!")
    //         .await;
    // }
    // async fn did_open(&self, _: DidOpenTextDocumentParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "file opened!")
    //         .await;
    // }

    // async fn did_change(&self, _: DidChangeTextDocumentParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "file changed!")
    //         .await;
    // }

    // async fn did_save(&self, _: DidSaveTextDocumentParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "file saved!")
    //         .await;
    // }

    // async fn did_close(&self, _: DidCloseTextDocumentParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "file closed!")
    //         .await;
    // }
    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            //types
            CompletionItem::new_simple("i32".to_string(), "32-bit signed integer type".to_string()),
            CompletionItem::new_simple("i64".to_string(), "64-bit signed integer type".to_string()),
            CompletionItem::new_simple(
                "u32".to_string(),
                "32-bit unsigned integer type".to_string(),
            ),
            CompletionItem::new_simple(
                "u64".to_string(),
                "64-bit unsigned integer type".to_string(),
            ),
            CompletionItem::new_simple("f32".to_string(), "32-bit floating point type".to_string()),
            CompletionItem::new_simple("f64".to_string(), "64-bit floating point type".to_string()),
            CompletionItem::new_simple("str".to_string(), "string".to_string()),
            CompletionItem::new_simple("char".to_string(), "single character".to_string()),
            CompletionItem::new_simple("bool".to_string(), "boolean".to_string()),
            CompletionItem::new_simple("none".to_string(), "no value".to_string()),
            //keywords
            CompletionItem::new_simple("import".to_string(), "import declaration".to_string()),
            CompletionItem::new_simple("extern".to_string(), "import external functions from a namespace".to_string()),
            CompletionItem::new_simple("fn".to_string(), "declares a function".to_string()),
            CompletionItem::new_simple("export".to_string(), "export declaration".to_string()),
            CompletionItem::new_simple("return".to_string(), "return statement".to_string()),
            CompletionItem::new_simple("if".to_string(), "if statement".to_string()),
            CompletionItem::new_simple("else".to_string(), "else statement".to_string()),
            CompletionItem::new_simple("while".to_string(), "while statement".to_string()),
            CompletionItem::new_simple("break".to_string(), "break statement".to_string()),
            CompletionItem::new_simple("continue".to_string(), "continue statement".to_string()),
            CompletionItem::new_simple(
                "var".to_string(),
                "declares a function-scoped or globally-scoped variable".to_string(),
            ),
            CompletionItem::new_simple(
                "val".to_string(),
                "declares a constant variable".to_string(),
            ),
        ])))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        // let _markdown = MarkupContent {
        //     kind: MarkupKind::Markdown,
        //     value: [
        //         "# Header",
        //         "Some text",
        //     ]
        //     .join("\n"),
        // };
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("".to_string())),
            /// HoverContents::Markup(markdown),
            range: None,
        }))
    }
}
