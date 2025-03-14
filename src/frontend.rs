// our parser ig for now
peg::parser!{
  grammar inven_parser() for str {


    // 1. MAIN EXPRESSIONS

    // Imports: Importing packages
    pub rule import() -> (String, Option<String>, bool)
      = "unpack" __ pkg_manager:("box" ":")? module:module_identifier() _ modulechild:module_sub_identifier()? _ ";" { (module, modulechild, pkg_manager.is_some()) }

    // Var Declaration: Declare vars (UNFINISHED)
    pub rule declaration() -> (String, bool)
      = variable:variable_type() __ mutable:("mut")? __ variable:identifier() (":" _ data_type:types())? _ ("=" _ value:identifier())? _ ";" { (variable, mutable.is_some()) }

    // Expressions: placeholder for function calls, etc. (UNFINISHED)
    pub rule expression() -> (String)
      = expression:identifier() _ "()"? _ ";" { expression }


    // 2. LITERALS / VALUES

    // (Placeholder for future rules handling strings, arrays, objects, etc.)



    // 3. IDENTIFIER RULES

    // Rules for what function and variable names are allowed as
    rule identifier() -> String
      = quiet!{ n:$(
            ['a'..='z' | 'A'..='Z' | '_']
            ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*
        ) { n.to_owned() } }
      / expected!("identifier")

    // Rules for what types can look like (UNFINISHED)
    rule types() -> String
      = quiet!{ n:$(
            (['a'..='z' | 'A'..='Z' | '0'..='9']+)
            ("{" ("8"/"18"/"32"/"64"/"128"/"any") "}")?
          ) { n.to_owned() } }
      / expected!("type")

    // Rules for what variables can be declared with
    rule variable_type() -> String
      = quiet!{ n:$((("local" / "lo") / ("scope" / "sc"))) { n.to_owned() } }
      / expected!("variable")

      
    // Rules for what modules are allowed to be written as
    rule module_identifier() -> String
      = quiet!{ n:$(
            (identifier()*) ** "."
          ) { n.to_owned() } }
      / expected!("module identifier")

    // Rules for the module syntax
    rule module_sub_identifier() -> String
      = precedence!{
        n:$("{" _ $( (identifier() (_ "as" __ identifier())? ) ** (_ "," _) ) _ "}" ) {n.to_owned()}
        --
        n:$("{" _ "*" _ "}") {n.to_owned()}
      }
      / expected!("sub module identifier")


    // 4. UTILITY RULES

    // Optional whitepace (zero or more)
    rule _() = quiet!{[' ' | '\t']*}

    // Obligatory whitespace (one or more)
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

  // Unpack
  // TODO: Make Multiline possible. EOF issue
  #[test]
  fn unpack() {
    let res = inven_parser::import("unpack module;").expect("Should be Ok");
    assert_eq!(res.0, "module");
    assert_eq!(res.1, None);
    assert_eq!(res.2, false); // box:
  }
  #[test]
  fn unpack_nested() {
    let res = inven_parser::import("unpack module.nested;").expect("Should be Ok");
    assert_eq!(res.0, "module.nested");
    assert_eq!(res.1, None);
    assert_eq!(res.2, false);
  }
  #[test]
  fn unpack_curly() {
    let res = inven_parser::import("unpack module { child };").expect("Should be Ok");
    assert_eq!(res.0, "module");
    assert_eq!(res.1, Some("{ child }".to_string()));
    assert_eq!(res.2, false);
  }
  #[test]
  fn unpack_box() {
    let res = inven_parser::import("unpack box:module;").expect("Should be Ok");
    assert_eq!(res.0, "module");
    assert_eq!(res.1, None);
    assert_eq!(res.2, true);
  }
  #[test]
  fn unpack_extended() {
    let res = inven_parser::import("unpack box:module.nested { child as cat };").expect("Should be Ok");
    assert_eq!(res.0, "module.nested");
    assert_eq!(res.1, Some("{ child as cat }".to_string()));
    assert_eq!(res.2, true);
  }

  // Variables
  #[test]
  fn var_decl() {
    let res = inven_parser::declaration("local mut var:int{64} = value;").expect("Should be Ok");
    assert_eq!(res.0, "var");
    assert_eq!(res.1, true);
  }
}