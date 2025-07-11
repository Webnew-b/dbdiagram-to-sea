use thiserror::Error;


#[derive(Debug,Error)]
pub enum GenerationErrorKind {
    #[error("Could not create the folder {0}.")]
    CouldNotCreateFolder(String),

    #[error("Could not create the file {0}.")]
    CouldNoteCreateFile(String),

    #[error("Could not render the context.")]
    CouldNoteRenderContext,

    #[error("Could not load the file template {0}.")]
    CouldNotLoadTemplate(String),
}
