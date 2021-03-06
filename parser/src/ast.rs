use getset::{Getters, CopyGetters, Setters};

type Id = u64;

#[derive(Clone, Constructor, Getters, CopyGetters, Setters, Debug, PartialEq)]
#[get = "pub"]
pub struct CodeGeneratorRequest {
    nodes: Vec<Node>,
    requested_files: Vec<code_generator_request::RequestedFile>
}

pub mod code_generator_request {
    use super::*;

    #[derive(Clone, Constructor, Getters, CopyGetters, Setters, Default, Debug, PartialEq)]
    pub struct RequestedFile {
        #[get_copy = "pub"]
        id: Id,

        #[get = "pub"]
        filename: String,

        #[get = "pub"]
        imports: Vec<requested_file::Import>
    }

    pub mod requested_file {
        use super::*;

        #[derive(Clone, Constructor, Getters, CopyGetters, Setters, Default, Debug, PartialEq)]
        pub struct Import {
            #[get_copy = "pub"]
            id: Id,

            #[get = "pub"]
            name: String
        }
    }
}

#[derive(Clone, Constructor, Getters, CopyGetters, Setters, Debug, PartialEq)]
pub struct Annotation {
    #[get_copy = "pub"]
    id: Id,

    #[get = "pub"]
    value: Value
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Void,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Text,
    Data,
    List(Box<Type>),
    Enum { type_id: Id },
    Struct { type_id: Id },
    Interface { type_id: Id },
    AnyPointer
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Unknown
}

#[derive(Clone, Constructor, Getters, CopyGetters, Setters, Debug, PartialEq)]
pub struct Node {
    #[get_copy = "pub"]
    id: Id,

    #[get = "pub"]
    display_name: String,

    #[get_copy = "pub"]
    display_name_prefix_length: usize,

    #[get_copy = "pub"]
    scope_id: Id,

    #[get = "pub"]
    nested_nodes: Vec<node::NestedNode>,

    #[get = "pub"]
    annotations: Vec<Annotation>,

    #[get = "pub"]
    which: node::Which
}

pub mod node {
    use getset::{Getters, CopyGetters, Setters};
    use crate::ast;

    #[derive(Clone, Constructor, Getters, CopyGetters, Setters, Debug, PartialEq)]
    pub struct NestedNode {
        #[get_copy = "pub"]
        id: super::Id,

        #[get = "pub"]
        name: String
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Which {
        File,
        Struct {
            is_group: bool,
            discriminant_count: u16,
            discriminant_offset: u32,
            fields: Vec<ast::Field>
        },
        Enum(Vec<super::Enumerant>),
        Interface,
        Const,
        Annotation
    }
}

#[derive(Clone, Constructor, Getters, CopyGetters, Setters, Debug, PartialEq)]
pub struct Field {
    #[get = "pub"]
    name: String,

    #[get_copy = "pub"]
    discriminant_value: u16,

    #[get = "pub"]
    which: field::Which
}

pub mod field {
    pub const NO_DISCRIMINANT : u16 = 0xFFFF;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Which {
        Slot(super::Type),
        Group(u64)
    }
}

    
#[derive(Clone, Constructor, Getters, CopyGetters, Setters, Default, Debug, PartialEq)]
pub struct Enumerant {
    #[get = "pub"]
    name: String
}