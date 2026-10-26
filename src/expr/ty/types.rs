use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeID(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub params: Vec<ParamType>,
    pub ret: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParamType {
    One(Type),
    Var(Type),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    Bool,
    None,

    String,
    Object(TypeID),
    Maybe(Box<Type>),
    List(Box<Type>),

    Function { name: String, func: Function },

    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcreteObjectType {
    pub fields: HashMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeTable {
    pub objects: HashMap<TypeID, ConcreteObjectType>,
}

impl TypeTable {
    pub fn get_object(&self, ty: TypeID) -> Option<&ConcreteObjectType> {
        self.objects.get(&ty)
    }

    pub fn get_attr(&self, ty: &Type, attr: String) -> Option<Type> {
        match ty {
            Type::Object(type_id) => {
                let concrete = self.get_object(*type_id)?;
                concrete.fields.get(&attr).map(|v| v.clone())
            }
            _ => None,
        }
    }
}

impl Type {
    pub fn is_bool(&self) -> bool {
        match *self {
            Type::Int
            | Type::Float
            | Type::Bool
            | Type::None
            | Type::String
            | Type::Maybe(_)
            | Type::List(_) => true,
            Type::Object(_) | Type::Function { .. } | Type::Unknown => false,
        }
    }

    fn describe_function(func: &Function, name: &String, types: &TypeTable) -> String {
        format!(
            "{}({}) -> {}",
            name,
            func.params
                .iter()
                .map(|v| match v {
                    ParamType::One(inner) => inner.describe(types),
                    ParamType::Var(inner) => format!("...{}", inner.describe(types)),
                })
                .collect::<Vec<String>>()
                .join(", "),
            func.ret.describe(types)
        )
    }

    pub fn describe(&self, types: &TypeTable) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::String => "str".to_string(),
            Type::None => "null".to_string(),
            Type::Object(type_id) => {
                let object = types.get_object(*type_id).unwrap();
                "{".to_string()
                    + &object
                        .fields
                        .iter()
                        .map(|(name, ty)| format!("{}: {}", name, ty.describe(types)))
                        .collect::<Vec<String>>()
                        .join(", ")
                    + "}"
            }
            Type::Maybe(inner) => format!("{}?", inner.describe(types)),
            Type::List(inner) => format!("{}[]", inner.describe(types)),
            Type::Function { name, func } => Self::describe_function(func, name, types),
            Type::Unknown => "unknown".to_string(),
        }
    }

    pub fn subtypes(&self, other: Type, types: &TypeTable) -> bool {
        match (self, other) {
            (a, b) if a == &b => true,
            (Type::Int, Type::Float) => true,
            (Type::Bool, Type::Int) => true,
            (Type::List(a), Type::List(b)) => a.subtypes(*b, types),
            (Type::None, Type::Maybe(_)) => true,
            _ => false,
        }
    }

    pub fn lub(a: &Type, b: &Type, types: &TypeTable) -> Result<Type, String> {
        // least upper bound
        match (a, b) {
            (a, b) if a == b => Ok(a.clone()),
            (Type::Unknown, t) | (t, Type::Unknown) => Ok(t.clone()),
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => Ok(Type::Float),
            (Type::String, Type::Bool | Type::Float | Type::Int)
            | (Type::Bool | Type::Float | Type::Int, Type::String) => Ok(Type::String),
            (Type::List(a), Type::List(b)) => Ok(Type::List(Box::new(Self::lub(a, b, types)?))),
            (Type::None, Type::Maybe(a)) | (Type::Maybe(a), Type::None) => {
                Ok(Type::Maybe(a.clone()))
            }
            (Type::Maybe(a), Type::Maybe(b)) => Ok(Type::Maybe(Box::new(Self::lub(a, b, types)?))),
            (a, b) => Err(format!(
                "cannot join together {} and {}",
                a.describe(types),
                b.describe(types)
            )),
        }
    }
}
