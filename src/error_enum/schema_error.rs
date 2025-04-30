use thiserror::Error;


#[derive(Debug,Error)]
pub enum SchemaErrorKind {
    #[error("Input '{input}' doesn't match pattern '{pattern}'")]
    NoMatch {
        input:String,
        pattern:String,
    },
    #[error("Input column type {colum_type} doesn't be contained.")]
    NoContained {
        colum_type:String,
    }
}
