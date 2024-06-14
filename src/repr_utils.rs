use std::fmt::Display;

use starlark::syntax::DialectTypes;

#[derive(Clone, Copy)]
pub(crate) struct PyReprBool(pub bool);

impl Display for PyReprBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            write!(f, "True")
        } else {
            write!(f, "False")
        }
    }
}

pub(crate) struct PyReprDialectTypes(pub DialectTypes);

impl Display for PyReprDialectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: keep consistent with PyDialectTypes.__str__
        match self.0 {
            DialectTypes::Disable => write!(f, "DialectTypes.DISABLE"),
            DialectTypes::ParseOnly => write!(f, "DialectTypes.PARSE_ONLY"),
            DialectTypes::Enable => write!(f, "DialectTypes.ENABLE"),
        }
    }
}
