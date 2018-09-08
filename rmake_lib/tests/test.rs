#[macro_use]
extern crate nom;
extern crate rmake_lib;

mod tests {

  use nom::types::CompleteStr;
  use rmake_lib::collapse_lines;
  use rmake_lib::parser::*;

  use std::fs;

  named!(makefile<CompleteStr,MakeFileTokens>,
    alt!(
      variable_defenition |
      single_line_comment |
      empty |
      unknown
    )
  );

  #[test]
  pub fn parse_single_comment_line() {
    let line = CompleteStr::from("# This a single line comment");
    let expected = Ok((
      CompleteStr(""),
      MakeFileTokens::Comment(String::from("This a single line comment")),
    ));
    assert_eq!(single_line_comment(line), expected);
  }

  #[test]
  pub fn parse_variable_definition() {
    let line = CompleteStr("prefix = /usr/local");
    let expected = Ok((
      CompleteStr(""),
      MakeFileTokens::VariableDefinition(VariableType::Override(Variable {
        name: String::from("prefix"),
        value: String::from("/usr/local"),
      })),
    ));

    assert_eq!(variable_defenition(line), expected);
  }

  #[test]
  pub fn read_file() {
    let body = fs::read_to_string("examples/Makefile").unwrap();
    let mut lines = body.lines();

    while let Some(line) = collapse_lines(&mut lines) {
      let res = makefile(CompleteStr(line.as_ref()));
      println!("\r\n{:?}", res);
      // match res {
      //   Ok((rest, token)) => println!("{:?}", (rest, token)),
      //   Err(err) => println!("{:?}", err),
      // }
    }
  }
}
