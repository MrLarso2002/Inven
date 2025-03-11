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
    pub rule import() -> () = "unpack" _ module:module_identifier _ modulechild:module_sub_identifier? _ ";" { (module, modulechild) }

    rule identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {n.to_owned()}}
      / expected!("identifier")
    rule module_identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_' | '@']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]** ".") {n.to_owned()}}
      / expected!("module identifier")
    rule module_sub_identifier() -> String
      = quiet!{ n:$("{" _ [ "*" | [module_child:identifier | module_child:identifier _ "as" alt_name:identifier ]** "," ] _ "}" )}
      / expected!("sub_module identifier")

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