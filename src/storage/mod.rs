mod common;
mod edge;
mod gen;
mod vertex;

use self::common::Token;
use self::edge::{EdgeManipulation, EdgeReport};
use self::gen::Init;
use self::vertex::{VertexManipulation, VertexReport};

// High-level traits
trait Manipulation<V, VA, VT: Token, E, EA, ET: Token>:
    VertexManipulation<V, VA, VT> + EdgeManipulation<VT, E, EA, ET>
{
}

trait Report<'a, V, VA: 'a, VT: Token, E, EA: 'a, ET: Token>:
    VertexReport<'a, V, VA, VT> + EdgeReport<'a, VT, EA, ET>
{
}

trait StorageAPI<'a, SA, V, VA: 'a, VT: Token, E, EA: 'a, ET: Token>:
    Init<SA> + Manipulation<V, VA, VT, E, EA, ET> + Report<'a, V, VA, VT, E, EA, ET>
{
}
