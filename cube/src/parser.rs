use nom::types::CompleteStr as Input;
use nom::*;

use crate::{Face, Move, Rotation};

named!(mv<Input, Move>,
  map!(
    one_of!("UFRLBDMxy"),
      |ly| match ly {
        'U' => Move::Face(Face::U, 1),
        'F' => Move::Face(Face::F, 1),
        'R' => Move::Face(Face::R, 1),
        'L' => Move::Face(Face::L, 1),
        'B' => Move::Face(Face::B, 1),
        'D' => Move::Face(Face::D, 1),
        'M' => Move::Slice(crate::Slice::M, 1),
        'x' => Move::Rotation(Rotation::X, 1),
        'y' => Move::Rotation(Rotation::Y, 1),
        _ => unreachable!()
      }
  )
);

named!(suffix<Input, u8>,
  map!(
    opt!(one_of!("'2")),
      |or| match or {
        Some('\'') => 3,
        Some('2') => 2,
        None => 1,
        _ => unreachable!()
      }
  )
);

named!(move_<Input, Move>,
  do_parse!(
    multispace0 >> m: mv >> amt: suffix >> (m.with_amount(amt))
  )
);

named!(moves<Input, Vec<Move>>,
  many0!( move_ )
);

pub fn parse_moves(data: &str) -> Result<Vec<Move>, String> {
  let (etc, moves) = moves(Input(data)).expect("unknown parser error");
  let etc = etc.to_string();
  if !etc.is_empty() {
    let pos = data.len() - etc.len();
    Err(format!(
      "{}\n{}^ parse error at position {}",
      data,
      String::from_utf8(vec![b' '; pos + 1]).unwrap(),
      pos,
    ))
  } else {
    Ok(moves)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid() {
    assert_eq!(vec![Move::Face(Face::U, 1)], parse_moves("U").unwrap());
    assert_eq!(vec![Move::Face(Face::U, 2)], parse_moves("U2").unwrap());
    assert_eq!(vec![Move::Face(Face::U, 3)], parse_moves("U'").unwrap());
    assert_eq!(
      vec![
        Move::Face(Face::U, 3),
        Move::Rotation(Rotation::X, 2),
        Move::Slice(crate::Slice::M, 1)
      ],
      parse_moves("U'x2 M").unwrap()
    );
  }

  #[test]
  fn invalid() {
    assert!(parse_moves("Foo").is_err());
    assert!(parse_moves("F3").is_err());
  }
}
