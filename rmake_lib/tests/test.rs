#[macro_use]
extern crate nom;
extern crate rmake_lib;

mod tests {
  use nom::Err as NomError;
  use nom::{
    alpha, alphanumeric0, anychar, digit, eol, is_alphanumeric, line_ending, non_empty,
    not_line_ending, print, rest, space0, types::CompleteStr, AtEof, Err, IResult, Needed,
  };

  use rmake_lib::parser::MakeFileTokens;
  use std::fs;
  use std::str;
  use std::str::Lines;

  named!(single_line_comment<CompleteStr, MakeFileTokens>,
    do_parse!(
      tag!("#") >>
      opt!(space0) >>
      comment: not_line_ending >>
      opt!(line_ending) >>
      (MakeFileTokens::Comment(comment.to_string()))
    )
  );

  named!(unknown<CompleteStr,MakeFileTokens>,
    do_parse!(
      unknown: map!(many0!(anychar),|v| v.iter().collect::<String>()) >>
      (MakeFileTokens::Unknown(unknown.to_string()))
    )
  );

  named!(empty<CompleteStr,MakeFileTokens>, do_parse!(not!(non_empty) >> (MakeFileTokens::EmptyLine)));

  named!(makefile<CompleteStr,MakeFileTokens>,
    alt!(
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
  pub fn collapse_lines(iter: &mut Lines) -> String {
    let mut new_str = String::new();

    loop {
      if let Some(new_line) = iter.next() {
        new_str.push_str(new_line);
        if new_line.chars().last() != Some('\\') {
          break;
        }
        new_str = new_str[..new_str.len() - 1].to_string();
      }
    }
    new_str
  }

  #[test]
  pub fn read_file() {
    let body = fs::read_to_string("examples/Makefile").unwrap();
    println!("\r\n");
    println!("\r\n");
    println!("\r\n");
    println!("\r\n");
    println!("\r\n");
    println!("\r\n");
    println!("\r\n");
    let mut lines = body.lines();

    for _i in 0..80 {
      let line = collapse_lines(&mut lines);

      //println!("\r\n{:?}", line);
      let res = makefile(CompleteStr(line.as_ref()));
      println!("\r\n{:?}", res);
      // match res {
      //   Ok((rest, token)) => println!("{:?}", (rest, token)),
      //   Err(err) => println!("{:?}", err),
      // }
    }
    // let input = CompleteStr("");
    // println!("\r\n{:?}", empty(input));
  }
}
