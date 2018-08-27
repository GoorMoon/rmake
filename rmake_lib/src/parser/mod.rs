#[derive(PartialEq,Debug)]
pub enum Directive {
  Include(Vec<String>),
}

#[derive(PartialEq,Debug)]
pub enum MakeFileTokens {
  ExplicitRule,
  ImplicitRule,
  VariableDefinition,
  Directive,
  Comment(String),
}
