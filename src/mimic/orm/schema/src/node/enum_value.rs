use crate::{
    node::{Arg, Def, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use lib_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use types::ErrorVec;

///
/// EnumValue
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumValue {
    pub def: Def,

    #[serde(default, skip_serializing)]
    pub variants: Vec<EnumValueVariant>,
}

impl MacroNode for EnumValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for EnumValue {}

impl VisitableNode for EnumValue {
    fn route_key(&self) -> String {
        self.def.path()
    }
}

///
/// EnumValueVariant
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumValueVariant {
    pub name: String,
    pub value: Arg,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unspecified: bool,
}

impl ValidateNode for EnumValueVariant {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // name
        if !self.name.is_case(Case::UpperCamel) {
            errs.add(format!(
                "variant name '{}' must be in UpperCamelCase",
                self.name
            ));
        }

        errs.result()
    }
}

impl VisitableNode for EnumValueVariant {
    fn drive<V: Visitor>(&self, v: &mut V) {
        self.value.accept(v);
    }
}
