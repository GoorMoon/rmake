#[macro_use]
extern crate nom;
extern crate rmake_lib;

mod tests {
  use nom::Err as NomError;
  use nom::{
    alpha, alphanumeric0, anychar, digit, eol, is_alphanumeric, line_ending, not_line_ending,
    print, rest, space0, types::CompleteStr, AtEof, Err, IResult, Needed,
  };

  use rmake_lib::parser::MakeFileTokens;
  use std::env;
  use std::fs;
  use std::str;

  named!(single_line_comment<&str, MakeFileTokens>,
    do_parse!(
      tag!("#") >>
      opt!(space0) >>
      comment: not_line_ending >>
      line_ending >>
      (MakeFileTokens::Comment(comment.to_string()))
    )
  );

  named!(unknown<&str,MakeFileTokens>,
    do_parse!(
      unknown: map!(many0!(anychar),|v| v.iter().collect::<String>()) >>
      (MakeFileTokens::Unknown(unknown))
    )
  );
  named!(makefile<&str,MakeFileTokens>,
    alt!(
      single_line_comment |
      unknown
    )
  );

  #[test]
  fn parse_comment_char() {
    let line = "# This a single line comment\r\n";
    assert_eq!(
      char!(line, '#'),
      Ok((" This a single line comment\r\n", '#'))
    );
  }

  #[test]
  pub fn parse_single_comment_line() {
    let line = "# This a single line comment\r\n";
    let expected = Ok((
      "",
      MakeFileTokens::Comment(String::from("This a single line comment")),
    ));
    assert_eq!(single_line_comment(line), expected);
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
    for i in (0..3) {
      let line = lines.next();
      println!("\r\n{:?}", line);
      let res = makefile(line.unwrap());
      println!("\r\n{:?}", res);
      // match res {
      //   Ok((rest, token)) => println!("{:?}", (rest, token)),
      //   Err(err) => println!("{:?}", err),
      // }
    }
  }

  pub fn comment_parser(input: &str) -> IResult<&str, String> {
    do_parse!(
      input,
      char!('#') >> space0
        >> comment:
          many_till!(
            anychar,
            alt!(map!(pair!(tag!("\\"), line_ending), |(x, _y)| x) | line_ending | eol)
          ) >> ({
        let (first, second) = comment;
        match second {
          "\\" => return Err(NomError::Incomplete(Needed::Unknown)),
          _ => first.iter().collect::<String>(),
        }
      })
    )
  }

  #[test]
  pub fn parse_multi_line_comment() {
    let line = "#!/usr/bin/make -f \
Generated automatically from Makefile.in by configure.
# Un*x Makefile for GNU tar program.";
    let expected = Ok((
      "# Un*x Makefile for GNU tar program.".as_ref(),
      "!/usr/bin/make -f Generated automatically from Makefile.in by configure.".to_string(),
    ));
    assert_eq!(comment_parser(line), expected);
  }
}
