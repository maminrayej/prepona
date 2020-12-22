pub enum ErrorKind {
    VertexNotFound,
    EdgeNotFound,
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::VertexNotFound => "Vertex not found".to_string(),
            ErrorKind::EdgeNotFound => "Edge not found".to_string(),
        }
    }
}

pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Error { kind, msg }
    }

    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<(ErrorKind, usize)> for Error {
    fn from((kind, id): (ErrorKind, usize)) -> Self {
        Error {
            msg: match kind {
                ErrorKind::VertexNotFound => format!("Vertex not found: {}", id),
                ErrorKind::EdgeNotFound => format!("Edge not found: {}", id),
            },
            kind,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
