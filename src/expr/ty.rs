use std::{collections::HashMap, fmt::Display as DisplayTrait};
use derive_more::Display;

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{0}ty", self.0)]
pub struct TypeID(usize);

#[derive(Debug, Clone)]
pub struct TypeArena {
    types: HashMap<TypeID, Type>
}

impl TypeArena {
    pub fn new() -> TypeArena {
        TypeArena { types: HashMap::new() }
    }

    pub fn insert(&mut self, ty: Type) -> TypeID {
        let id = TypeID(self.types.len());
        self.types.insert(id, ty);
        id
    }

    pub fn get(&self, id: TypeID) -> &Type {
        self.types.get(&id).expect("Type ID is not in the arena.") // we are all responsible adults here
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    None,
    Array(Box<Type>),
    Any,
    Function {
        this: Option<Box<Type>>,
        params: Vec<Box<Type>>,
        ret: Box<Type>
    },
    Object(TypeID),
    Union(Vec<Type>),
    Unknown
}


impl DisplayTrait for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "str"),
            Type::None => write!(f, "none"),
            Type::Array(ty_box) => write!(f, "list[{0}]", ty_box.as_ref()),
            Type::Any => write!(f, "any"),
            Type::Function { this: _, params, ret } => {
                write!(
                    f,
                    "({0}) -> {1}",
                    params.iter().map(|b| format!("{}", b.as_ref())).collect::<Vec<String>>().join(", "),
                    ret
                )
            },
            Type::Object(object_info) => write!(f, "{}", object_info),
            Type::Union(types) => write!(f, "{}", types.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(" | ")),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectInfo {
    pub fields: HashMap<String, Box<Type>>,
    pub proto: TypeID
}

impl DisplayTrait for ObjectInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::with_capacity(3 + self.fields.len());
        parts.push("{".to_string());
        for (key, ty) in &self.fields {
            parts.push(format!("{key}: {ty}"));
        }
        parts.push("}".to_string());
        write!(f, "{}", parts.join(" "))
    }
}