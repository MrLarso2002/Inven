// our parser ig for now
peg::parser!{
  grammar inven_parser() for str {
    pub rule import() -> (String, Option<String>, bool)
      = "unpack" __ pkg_manager:("box" ":")? module:module_identifier() _ modulechild:module_sub_identifier()? _ ";" { (module, modulechild, pkg_manager.is_some()) }
      pub rule importmin() -> String
        = "unpack" _ module:module_identifier() _ ";" { module }

    rule identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {n.to_owned()}}
      / expected!("identifier")

    rule module_identifier() -> String
      = quiet!{ n:$((['a'..='z' | 'A'..='Z' | '_' | '@']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*)** ".") {n.to_owned()}}
      / expected!("module identifier")

    rule module_sub_identifier() -> String
      = precedence!{
        n:$("{" _ $( (identifier() (_ "as" __ identifier())? ) ** (_ "," _) ) _ "}" ) {n.to_owned()}
        --
        n:$("{" _ "*" _ "}") {n.to_owned()}
      }
      / expected!("sub module identifier")

    rule _() = quiet!{[' ' | '\t']*}
    rule __() = quiet!{[' ' | '\t']+}
  }
}

pub fn main() {
// e
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unpack() {
    // TODO: This should pass but currently it doesn't
    let res = inven_parser::import("unpack box: module.child;").expect("Should be Ok");
    assert_eq!(res.0, "module.child");
    assert_eq!(res.1, None);
    assert_eq!(res.2, true);
  }
}