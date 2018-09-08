use nom::{anychar, line_ending, non_empty, not_line_ending, rest, space0, types::CompleteStr};

#[derive(PartialEq, Debug)]
pub enum Directive {
    Include(Vec<String>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
}
#[derive(PartialEq, Debug, Clone)]
pub enum VariableType {
    Override(Variable),
    Append(Variable),
    NotDefined(Variable),
    Expansion(Variable),
    Shell(Variable),
}

#[derive(PartialEq, Debug, Clone)]
pub enum MakeFileTokens {
    ExplicitRule,
    ImplicitRule,
    VariableDefinition(VariableType),
    Directive,
    Comment(String),
    EmptyLine,
    Unknown(String),
}

named!(pub single_line_comment<CompleteStr, MakeFileTokens>,
    do_parse!(
      tag!("#") >>
      opt!(space0) >>
      comment: not_line_ending >>
      opt!(line_ending) >>
      (MakeFileTokens::Comment(comment.to_string()))
    )
  );

named!(pub unknown<CompleteStr,MakeFileTokens>,
    do_parse!(
      unknown: map!(many0!(anychar),|v| v.iter().collect::<String>()) >>
      (MakeFileTokens::Unknown(unknown.to_string()))
    )
  );

named!(pub empty<CompleteStr,MakeFileTokens>, do_parse!(not!(non_empty) >> (MakeFileTokens::EmptyLine)));

named!(pub variable_defenition<CompleteStr,MakeFileTokens>,
  alt!(
    variable_defenition_override | 
    variable_defenition_expansion |
    variable_defenition_notdefined
  )
);

named!(variable_defenition_override<CompleteStr,MakeFileTokens>,
  do_parse!(
    opt!(space0) >>
    varible_name: map!(many1!(none_of!(":#= ")),|v| v.iter().collect::<String>()) >>
    opt!(space0) >>
    tag!("=") >>
    opt!(space0) >>
    variable_value: rest >>
    (
      MakeFileTokens::VariableDefinition(
        VariableType::Override(
          Variable { 
            name:varible_name.to_string(),
            value: variable_value.to_string()
          }
        )
      )
    )
  )
);

named!(variable_defenition_expansion<CompleteStr,MakeFileTokens>,
  do_parse!(
    opt!(space0) >>
    varible_name: map!(many1!(none_of!(":#= ")),|v| v.iter().collect::<String>()) >>
    opt!(space0) >>
    tag!(":=") >>
    opt!(space0) >>
    variable_value: rest >>
    (
      MakeFileTokens::VariableDefinition(
        VariableType::Expansion(
          Variable { 
            name:varible_name.to_string(),
            value: variable_value.to_string()
          }
        )
      )
    )
  )
);

named!(variable_defenition_notdefined<CompleteStr,MakeFileTokens>,
  do_parse!(
    opt!(space0) >>
    varible_name: map!(many1!(none_of!(":#= ")),|v| v.iter().collect::<String>()) >>
    opt!(space0) >>
    tag!("?=") >>
    opt!(space0) >>
    variable_value: rest >>
    (
      MakeFileTokens::VariableDefinition(
        VariableType::NotDefined(
          Variable { 
            name:varible_name.to_string(),
            value: variable_value.to_string()
          }
        )
      )
    )
  )
);

