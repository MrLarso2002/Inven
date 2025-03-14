// our parser ig for now
peg::parser!{
  grammar inven_parser() for str {
    pub rule import() -> (String, Option<String>, bool)
      = "unpack" __ pkg_manager:("box" ":")? module:module_identifier() _ modulechild:module_sub_identifier()? _ ";" { (module, modulechild, pkg_manager.is_some()) }
      pub rule importmin() -> String
        = "unpack" __ module:module_identifier() _ ";" { module }

    // TODO: finish variable declaration.
    // for the value, its currently set to identifier rule,this is placeholder cuz this will be a bit harder to implement for all the possible cases [] {} "" --.
    // The types are set to be very broad. ur welcome to setting up ur specific array thing, but this should be more uniform. plus we allow custom types so..
    pub rule declaration() -> (String, bool)
      = variable:variable_type() __ mutable:("mut")? __ variable:identifier() (":" _ data_type:types())? _ ("=" _ value:identifier())? _ ";" { (variable, mutable.is_some()) }

    // TODO: Finish the entire logic. there should prob be a rule for parantehthis and values you can assign so we dont have to rewrite
    pub rule expression() -> (String)
      = expression:identifier() _ "()"? _ ";" { expression }

    rule types() -> String
    = quiet!{ n:$((['a'..='z' | 'A'..='Z' | '0'..='9']+)("{" ("8"/"18"/"32"/"64"/"128"/"any") "}")?) {n.to_owned()}}
      / expected!("type")
    rule variable_type() -> String
      = quiet!{ n:$((("local" / "lo") / ("scope" / "sc"))) {n.to_owned()}} 
      / expected!("variable")

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
    
    // TODO: Make Multiline possible. EOF issue
    let res = inven_parser::import("unpack module;").expect("Should be Ok");
    assert_eq!(res.0, "module");
    assert_eq!(res.1, None);
    assert_eq!(res.2, false); // box:
  }

  #[test]
  fn var_decl() {
    let res = inven_parser::declaration("local mut var:int{64} = value;").expect("Should be Ok");
    assert_eq!(res.0, "var");
    assert_eq!(res.1, true);
  }
}