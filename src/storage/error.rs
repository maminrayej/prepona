pub enum ErrorKind {
    VertexNotFound,
    EdgeNotFound,
}

pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Error { kind, msg }
    }

    pub fn new_vnf(vertex_id: usize) -> Self {
        Error {
            kind: ErrorKind::VertexNotFound,
            msg: format!("Vertex with id: {} not found", vertex_id),
        }
    }

    pub fn new_enf(edge_id: usize) -> Self {
        Error {
            kind: ErrorKind::EdgeNotFound,
            msg: format!("Edge with id: {} not found", edge_id),
        }
    }

    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::error::Error for Error {}
