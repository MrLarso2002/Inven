peg::parser!{
  grammar list_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    pub rule list() -> Vec<u32>
      = "[" l:(number() ** ",") "]" { l }
  }
}

// our parser ig for now
peg::parser!{
  grammar inven_parser() for str {
    pub rule import() -> () = "unpack" _ module:module_identifier _ ";" { (module) }

/*
unpack module.thing.thang.theng.funcA
unpack module.thing.thang.theng.funcB
npack module.thing.thang.theng.funcC

unpack module.thing.thang.theng. {func, func as e, func}
*/
// fuck my mom wants me to study.... but ill try to write here as often as can- if u want dot, at least make it like this
// lmao we both studying. its alr 
// i can't be very active, have to study also...
    rule identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {n.to_owned()}}
      / expected!("identifier")
    rule module_identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_' | '@']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.']*['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {n.to_owned()}}
      / expected!("module_identifier")
    rule _() = quiet!{[' ' | '\t']*}
  }
}

pub fn main() {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unpack() {

  }
}