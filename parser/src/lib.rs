extern crate capnp;
#[macro_use] extern crate derive_more;
extern crate getset;

pub mod ast;
#[allow(dead_code)]
mod schema_capnp;

trait ParseFrom<R> : Sized {
    fn parse(reader: R) -> capnp::Result<Self>;
}

impl ParseFrom<schema_capnp::type_::Reader<'_>> for ast::Type {
    fn parse(reader: schema_capnp::type_::Reader<'_>) -> capnp::Result<ast::Type> {
        Ok(
            match reader.which()? {
                schema_capnp::type_::Which::AnyPointer(_) => ast::Type::AnyPointer,
                schema_capnp::type_::Which::Bool(_) => ast::Type::Bool,
                schema_capnp::type_::Which::Data(_) => ast::Type::Data,
                schema_capnp::type_::Which::Enum(e) => ast::Type::Enum { type_id: e.get_type_id() },
                schema_capnp::type_::Which::Float32(_) => ast::Type::Float32,
                schema_capnp::type_::Which::Float64(_) => ast::Type::Float64,
                schema_capnp::type_::Which::Int16(_) => ast::Type::Int16,
                schema_capnp::type_::Which::Int32(_) => ast::Type::Int32,
                schema_capnp::type_::Which::Int64(_) => ast::Type::Int64,
                schema_capnp::type_::Which::Int8(_) => ast::Type::Int8,
                schema_capnp::type_::Which::Interface(i) => ast::Type::Interface { type_id: i.get_type_id() },
                schema_capnp::type_::Which::List(t) => ast::Type::List(Box::new(ast::Type::parse(t.get_element_type()?)?)),
                schema_capnp::type_::Which::Struct(s) => ast::Type::Struct { type_id: s.get_type_id() },
                schema_capnp::type_::Which::Text(_) => ast::Type::Text,
                schema_capnp::type_::Which::Uint16(_) => ast::Type::Uint16,
                schema_capnp::type_::Which::Uint32(_) => ast::Type::Uint32,
                schema_capnp::type_::Which::Uint64(_) => ast::Type::Uint64,
                schema_capnp::type_::Which::Uint8(_) => ast::Type::Uint8,
                schema_capnp::type_::Which::Void(_) => ast::Type::Void
            }
        )
    }
}

impl ParseFrom<schema_capnp::field::WhichReader<'_>> for ast::field::Which {
    fn parse(reader: schema_capnp::field::WhichReader<'_>) -> capnp::Result<ast::field::Which> {
        Ok(
            match reader {
                schema_capnp::field::Which::Group(g) => ast::field::Which::Group(g.get_type_id()),
                schema_capnp::field::Which::Slot(s) => ast::field::Which::Slot(
                    ast::Type::parse(s.get_type()?)?
                )
            }
        )
    }
}

impl ParseFrom<schema_capnp::field::Reader<'_>> for ast::Field {
    fn parse(reader: schema_capnp::field::Reader<'_>) -> capnp::Result<ast::Field> {
        Ok(
            ast::Field::new(
                String::from(reader.get_name()?),
                reader.get_discriminant_value(),
                ast::field::Which::parse(reader.which()?)?
            )
        )
    }
}

impl ParseFrom<schema_capnp::enumerant::Reader<'_>> for ast::Enumerant {
    fn parse(reader: schema_capnp::enumerant::Reader<'_>) -> capnp::Result<ast::Enumerant> {
        Ok(
            ast::Enumerant::new(
                String::from(reader.get_name()?)
            )
        )
    }
}

impl ParseFrom<schema_capnp::node::WhichReader<'_>> for ast::node::Which {
    fn parse(reader: schema_capnp::node::WhichReader) -> capnp::Result<ast::node::Which> {
        Ok(
            match reader {
                schema_capnp::node::Which::File(_) => ast::node::Which::File,
                schema_capnp::node::Which::Struct(s) => {
                    let mut fields = vec!();
                    for field in s.get_fields()?.iter() {
                        fields.push(ast::Field::parse(field)?);
                    }
                    ast::node::Which::Struct {
                        is_group: s.get_is_group(),
                        discriminant_count: s.get_discriminant_count(),
                        discriminant_offset: s.get_discriminant_offset(),
                        fields: fields
                    }
                },
                schema_capnp::node::Which::Enum(e) => {
                    let mut enums = vec!();
                    for enumerant in e.get_enumerants()?.iter() {
                        enums.push(ast::Enumerant::parse(enumerant)?);
                    }
                    ast::node::Which::Enum(enums)
                },
                schema_capnp::node::Which::Interface(_) => ast::node::Which::Interface,
                schema_capnp::node::Which::Const(_) => ast::node::Which::Const,
                schema_capnp::node::Which::Annotation(_) => ast::node::Which::Annotation,
            }
        )
    }
}

impl ParseFrom<schema_capnp::node::nested_node::Reader<'_>> for ast::node::NestedNode {
    fn parse(reader: schema_capnp::node::nested_node::Reader<'_>) -> capnp::Result<ast::node::NestedNode> {
        Ok(
            ast::node::NestedNode::new(reader.get_id(), String::from(reader.get_name()?))
        )
    }
}

impl ParseFrom<schema_capnp::value::Reader<'_>> for ast::Value {
    fn parse(reader: schema_capnp::value::Reader) -> capnp::Result<ast::Value> {
        Ok(
            match reader.which()? {
                schema_capnp::value::Which::Text(t) => {
                    ast::Value::Text(String::from(t?))
                },
                _ => {
                    ast::Value::Unknown
                }
            }
        )
    }
}

impl ParseFrom<schema_capnp::annotation::Reader<'_>> for ast::Annotation {
    fn parse(reader: schema_capnp::annotation::Reader) -> capnp::Result<ast::Annotation> {
        Ok(
            ast::Annotation::new(
                reader.get_id(),
                ast::Value::parse(reader.get_value()?)?
            )
        )
    }
}

impl ParseFrom<schema_capnp::node::Reader<'_>> for ast::Node {
    fn parse(reader: schema_capnp::node::Reader<'_>) -> capnp::Result<ast::Node> {
        let mut nested_nodes = vec!();
        for nested_node in reader.get_nested_nodes()?.iter() {
            nested_nodes.push(ast::node::NestedNode::parse(nested_node)?)
        }

        let mut annotations = vec!();
        for annotation in reader.get_annotations()?.iter() {
            annotations.push(ast::Annotation::parse(annotation)?)
        }

        return Ok(
            ast::Node::new(
                reader.get_id(),
                String::from(reader.get_display_name()?),
                reader.get_display_name_prefix_length() as usize,
                reader.get_scope_id(),
                nested_nodes,
                annotations,
                ast::node::Which::parse(reader.which()?)?
            )
        )
    }
}

impl ParseFrom<schema_capnp::code_generator_request::requested_file::import::Reader<'_>> for ast::code_generator_request::requested_file::Import {
    fn parse(reader: schema_capnp::code_generator_request::requested_file::import::Reader<'_>) -> capnp::Result<ast::code_generator_request::requested_file::Import> {
        return Ok(
            ast::code_generator_request::requested_file::Import::new(
                reader.get_id(),
                String::from(reader.get_name()?),
            )
        )
    }
}

impl ParseFrom<schema_capnp::code_generator_request::requested_file::Reader<'_>> for ast::code_generator_request::RequestedFile {
    fn parse(reader: schema_capnp::code_generator_request::requested_file::Reader<'_>) -> capnp::Result<ast::code_generator_request::RequestedFile> {
        let mut imports = vec!();
        for import in reader.get_imports()?.iter() {
            imports.push(ast::code_generator_request::requested_file::Import::parse(import)?);
        }
        return Ok(
            ast::code_generator_request::RequestedFile::new(
                reader.get_id(),
                String::from(reader.get_filename()?),
                imports
            )
        )
    }
}

impl ParseFrom<schema_capnp::code_generator_request::Reader<'_>> for ast::CodeGeneratorRequest {
    fn parse(reader: schema_capnp::code_generator_request::Reader) -> capnp::Result<ast::CodeGeneratorRequest> {
        let mut nodes = vec!();
        for node in reader.get_nodes()?.iter() {
            nodes.push(ast::Node::parse(node)?);
        }
        let mut requested_files = vec!();
        for requested_file in reader.get_requested_files()?.iter() {
            requested_files.push(ast::code_generator_request::RequestedFile::parse(requested_file)?);
        }
        return Ok(ast::CodeGeneratorRequest::new(nodes, requested_files));
    }
}

pub fn parse(request: schema_capnp::code_generator_request::Reader) -> capnp::Result<ast::CodeGeneratorRequest> {
    return ast::CodeGeneratorRequest::parse(request);
}

pub fn read_message(mut reader: &mut dyn std::io::Read) -> ast::CodeGeneratorRequest {
    let msg_raw = capnp::serialize::read_message(&mut reader, capnp::message::ReaderOptions::new()).unwrap();
    let msg_capnp = msg_raw.get_root::<schema_capnp::code_generator_request::Reader>().unwrap();
    return parse(msg_capnp).unwrap();
}