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
    },

    #[error("Fail to get configuration from content.")]
    FileFormatFile,


    #[error("Could not match pattern from : {match_by}")]
    RegexMatchError {
        match_by:String
    },

    #[error("Validation Error:Value: {field},Type:{field_type}")]
    VailateFieldFail {
        field:String,
        field_type:String,
    },

    #[error("Relation column doesn't equal.from:{from},to:{to}")]
    RelationColumnNotEq {
        from:String,
        to:String,
    },

    #[error("Relation schema doesn't equal.from:{from},to:{to}")]
    RelationSchemaNotEq {
        from:String,
        to:String,
    }
}
