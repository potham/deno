// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use crate::swc_ecma_ast;
use serde::Serialize;

use super::parser::DocParser;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumMemberDef {
  pub name: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumDef {
  pub members: Vec<EnumMemberDef>,
}

pub fn get_doc_for_ts_enum_decl(
  _doc_parser: &DocParser,
  enum_decl: &swc_ecma_ast::TsEnumDecl,
) -> (String, EnumDef) {
  let enum_name = enum_decl.id.sym.to_string();
  let mut members = vec![];

  for enum_member in &enum_decl.members {
    use crate::swc_ecma_ast::TsEnumMemberId::*;

    let member_name = match &enum_member.id {
      Ident(ident) => ident.sym.to_string(),
      Str(str_) => str_.value.to_string(),
    };

    let member_def = EnumMemberDef { name: member_name };
    members.push(member_def);
  }

  let enum_def = EnumDef { members };

  (enum_name, enum_def)
}
