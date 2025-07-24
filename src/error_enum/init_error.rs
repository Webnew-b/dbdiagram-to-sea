use thiserror::Error;

#[derive(Debug,Error)]
pub enum InitErrorKind {
    #[error("The input file is not found.")]
    InputFileNotFound,
    #[error("The input must be a flie.")]
    InputMustBeFlie,
    #[error("The configuration could not be created.")]
    ConfigrationCouldNotCreated,
    #[error("The output folder could not be created.")]
    OutputFolderCouldNotCreated,
    #[error("The output must be a floder.")]
    OutputMustBeFolder,
    #[error("The configuration folder could not be created.")]
    TemplateFolderConldNotCreated,
    #[error("The output folder is unavailable.")]
    TemplateFileUnavailable,
}
