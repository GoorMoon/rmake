#[derive(PartialEq, Debug)]
pub enum Directive {
  Include(Vec<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum MakeFileTokens {
  ExplicitRule,
  ImplicitRule,
  VariableDefinition,
  Directive,
  Comment(String),
  EmptyLine,
  Unknown(String),
}
