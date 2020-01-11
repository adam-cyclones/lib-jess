
extern crate wasm_bindgen;
extern crate nom;

mod jess_utils;

use jess_utils::matches::{
    name_braced_list::matches_common_curly_braced_list_comma_sep,
    css_directive::capture_common_css_directive_name
};

use std::boxed::Box;
use std::rc::Rc;

use regex::Regex;
use enum_map::{enum_map, Enum, EnumMap};


use std::collections::BTreeMap;


use wasm_bindgen::prelude::*;
use nom::{
    IResult,
    bytes::complete::{
        tag,
        take_while_m_n,
    },
    combinator::map_res,
    sequence::tuple
};





#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug,PartialEq)]
struct ImportDirective<'a> {
    path: &'a str,
    parts: Vec<String>,
    default_name: &'a str,
}


// fn parse_common_jess_path_to_file (source: &str) -> String {
//     // path within quotes, single or double
//     // path can omit ext
//     return String::from("");
// }

// fn parse_directive_import_default_name (source: &str) -> IResult<&str, String> {
//     let default_name: String = parse_common_name(source);
//     // i/o source, name if valid or errors
//     return Ok(( source, default_name ));
// }

// fn parse_directive_import_parts (source: &str) -> Vec<String> {
//     let parts: Vec<String> = vec!();
//     // For part of {a, b, c} push
//     parts.push(String::from(""));
//     parts.push(String::from(""));
//     parts.push(String::from(""));
//     return parts;
// }

// fn parse_directive_import_path (source: &str) -> String {
//     return String::from("");
// }

// fn parse_directive_import(source: &str, useDefault: bool) -> IResult<&str, ImportDirective> {

//     // when match @import get righthand side
//     let (source, _) = tag("@import")(source)?;

//     if useDefault {
//         // @import name from '<path>';
//         let (source, (defaultName, path)) = tuple((
//             parse_directive_import_default_name,
//             parse_directive_import_path
//         ))(source)?;

//         // wrapped result
//         return Ok((
//             source,
//             ImportDirective {
//                 //
//                 defaultName: defaultName,
//                 /**
//                  * does this import have parts
//                 */
//                 parts: vec!(),
//                 /**
//                  * The path to import
//                 */
//                 path: path,
//             }
//         ));
//     } else {
//         // @import {} from '<path>';
//         let (source, (parts, path)) = tuple((
//             parse_directive_import_parts,
//             parse_directive_import_path
//         ))(source)?;

//         // wrapped result
//         return Ok((
//             source,
//             ImportDirective {
//                 //
//                 defaultName: "",
//                 /**
//                  * does this import have parts
//                 */
//                 parts: parts,
//                 /**
//                  * The path to import
//                 */
//                 path: path,
//             }
//         ));
//     }
// }

pub fn grammer_import_names (source: &str) {

}

pub fn grammer_import_default (source: &str) {

}

pub fn grammer_import_get_error () {

}

pub fn grammer_import_type (source: &str) -> String {
    let (_input, directive) = capture_common_css_directive_name(&source);

    return String::from(directive);
}

#[wasm_bindgen]
pub fn compile(source: &str) -> String {
    grammer_import_get_error();
    // let import_uses_default_name = true;
    // let parseImports = parse_directive_import(source, import_uses_default_name);

    return String::from(source);
}


// let tokens_patern: String = format!(r"
//     (
//         ({L_P_BRACKET}) | ({R_P_BRACKET}) |
//         ({L_C_BRACE}) | ({R_C_BRACE}) |
//         ({ADDITION_ASSIGNMENT}) | ({DIVISION_ASSIGNMENT}) | ({MINUS_ASSIGNMENT}) |
//         ({MODULUS_ASSIGNMENT}) | ({MULTIPLICATION_ASSIGNMENT}) |
//         ({GREATER_THAN_OR_EQUAL_TO}) | ({LESS_THAN_OR_EQUAL_TO}) |
//         ({BIT_L_SHIFT}) | ({LESS_THAN}) |
//         ({BIT_R_SHIFT}) | ({GREATER_THAN}) |
//         ({OR}) | ({BIT_OR}) |
//         ({EQUAL_VALUE}) | ({EQUAL_TO}) | ({EQUALS}) |
//         ({NOT_EQUAL_VALUE}) | ({NOT_EQUAL}) | ({NOT}) |
//         ({INCREMENT}) | ({ADDITION}) |
//         ({DECREMENT}) | ({SUBTRACTION}) |
//         ({BIT_NOT}) |
//         ({AND}) | ({BIT_AND}) |
//         ({BIT_XOR}) |
//         ({DELETE}) |
//         ({INSTANCEOF}) |
//         ({IN}) |
//         ({TYPEOF}) |
//         ({VOID}) |
//         ({COLON}) |
//         ({COMMA}) |
//         ({PERIOD}) |
//         ({SEMI_COLON}) |
//         ({MULTIPLICATION}) |
//         ({DIVISION}) |
//         ({MODULUS})
//     )
// ",
//     // Assignment
//     EQUALS = "\\={1}",
//     // Assignment Math
//     ADDITION_ASSIGNMENT = "\\+\\={1}",
//     DIVISION_ASSIGNMENT = "\\//\\={1}",
//     MINUS_ASSIGNMENT = "\\-\\={1}",
//     MODULUS_ASSIGNMENT = "\\%\\={1}",
//     MULTIPLICATION_ASSIGNMENT = "\\*\\={1}",
//     // Brace pairs
//     L_C_BRACE = "\\{{1}",
//     R_C_BRACE = "\\}{1}",
//     L_P_BRACKET = "\\({1}",
//     R_P_BRACKET = "\\){1}",
//     // Comparison Operators
//     EQUAL_TO = "\\={2}",
//     EQUAL_VALUE = "\\={3}",
//     GREATER_THAN = "\\>{1}",
//     GREATER_THAN_OR_EQUAL_TO = "\\>\\={1}",
//     LESS_THAN = "\\<{1}",
//     LESS_THAN_OR_EQUAL_TO = "\\<\\={1}",
//     NOT_EQUAL = "\\!\\={1}",
//     NOT_EQUAL_VALUE = "\\!\\=\\={1}",
//     // Operators: Bitwise
//     BIT_AND = "\\&{1}",
//     BIT_OR = "\\|{1}",
//     BIT_NOT = "\\~{1}",
//     BIT_XOR = "\\^{1}",
//     BIT_L_SHIFT = "\\<{2}",
//     BIT_R_SHIFT = "\\>{2}",
//     // Operators: Logical
//     AND = "\\&{2}",
//     OR = "\\|{2}",
//     NOT = "\\!{1}",
//     // Operators: Math
//     ADDITION = "\\+{1}",
//     SUBTRACTION = "-{1}",
//     MULTIPLICATION = "\\*{1}",
//     DIVISION = "\\//{1}",
//     MODULUS = "%{1}",
//     INCREMENT = "\\+{2}",
//     DECREMENT = "\\-{2}",
//     // Punctuation
//     COLON = ":{1}",
//     COMMA = ",{1}",
//     PERIOD = "\\.{1}",
//     SEMI_COLON = ";{1}",
//     // Word Oporators
//     DELETE = "\\bdelete",
//     IN = "\\bin",
//     INSTANCEOF = "\\binstanceof",
//     TYPEOF = "\\btypeof",
//     VOID = "\\bvoid"
// );
// strip whitespace from pattern


#[derive(Debug, Enum)]
pub enum GRAM {
    //     // Assignment
    EQUALS,
    // ASSIGNMENT MATH
    ADDITION_ASSIGNMENT,
    DIVISION_ASSIGNMENT,
    MINUS_ASSIGNMENT,
    MODULUS_ASSIGNMENT,
    MULTIPLICATION_ASSIGNMENT,
    // BRACE PAIRS
    L_C_BRACE,
    R_C_BRACE,
    L_P_BRACKET,
    R_P_BRACKET,
    // COMPARISON OPERATORS
    EQUAL_TO,
    EQUAL_VALUE,
    GREATER_THAN,
    GREATER_THAN_OR_EQUAL_TO,
    LESS_THAN,
    LESS_THAN_OR_EQUAL_TO,
    NOT_EQUAL,
    NOT_EQUAL_VALUE,
    // OPERATORS: BITWISE
    BIT_AND,
    BIT_OR,
    BIT_NOT,
    BIT_XOR,
    BIT_L_SHIFT,
    BIT_R_SHIFT,
    // OPERATORS: LOGICAL
    AND,
    OR,
    NOT,
    // OPERATORS: MATH
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    MODULUS,
    INCREMENT,
    DECREMENT,
    // PUNCTUATION
    COLON,
    COMMA,
    PERIOD,
    SEMI_COLON,
    // WORD OPORATORS
    OP_DELETE,
    OP_IN,
    OP_INSTANCEOF,
    OP_TYPEOF,
    OP_VOID,
    //ambiguous
    NAME,
    ANY,
    ANY_STRING,
    NONE,
    // Directive
    DIRECTIVE_IMPORT,
}

pub struct Callback<T> {
    f: Rc<dyn Fn(&ASTNode, &str) -> T>,
}

impl<T> Callback<T> {
    /// Call this callback
    pub fn call(&self, this: &ASTNode, token: &str) -> T {
        return (self.f)(this, token);
    }
}

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self {
            f: Rc::clone(&self.f),
        }
    }
}

impl<T, F: Fn(&ASTNode, &str) -> T + 'static> From<F> for Callback<T> {
    fn from(func: F) -> Self {
        Self { f: Rc::new(func) }
    }
}

pub struct ASTNode {
    id: GRAM,
    allow_next: Vec<GRAM>,
    token: &'static str,
    description: &'static str,
    mathcer: Callback<bool>
}

pub fn def_AST_node (
    id: GRAM, 
    token: &'static str,
    description: &'static str,
    allow_next: Vec<GRAM>,
    callback_matches: Callback<bool>
) -> ASTNode {
    return ASTNode{
        id: id,
        token: token,
        description: description,
        allow_next: allow_next,
        mathcer: Callback::from(callback_matches),
    };
}

#[wasm_bindgen]
pub fn ast(source: &str) -> String {

    let ast_def_jess_unknown = def_AST_node(
        GRAM::NONE,
        "",
        "Enexpected token",
        vec!(),
        Callback::from(is_none_of_jess)
    );

    fn is_none_of_jess (_self: &ASTNode, token: &str) -> bool {
        panic!(format!("Enexpected token {token} at pos:{line}:{col}", token = token, line = 0, col = 0));
        return false;
    }

    let ast_def_jess_import_directive = def_AST_node(
        GRAM::DIRECTIVE_IMPORT,
        "@import",
        "Directive import informs the compiler that another file's contents are required for this file to function.",
        vec!(
            GRAM::L_C_BRACE,
            GRAM::ANY_STRING,
            GRAM::NAME
        ),
        // Matcher callback
        Callback::from(is_jess_import_directive)
    );

    fn is_jess_import_directive (_self: &ASTNode, token: &str) -> bool {
        return token == _self.token;
    };

    let mut AST_GRAMMER = enum_map! {

        // GRAM::EQUALS => 0,
        // ASSIGNMENT MATH
            // GRAM::ADDITION_ASSIGNMENT => 0,
            // GRAM::DIVISION_ASSIGNMENT => 0,
            // GRAM::MINUS_ASSIGNMENT => 0,
            // GRAM::MODULUS_ASSIGNMENT => 0,
            // GRAM::MULTIPLICATION_ASSIGNMENT => 0,
        // BRACE PAIRS
            // GRAM::L_C_BRACE => 0,
            // GRAM::R_C_BRACE => 0,
            // GRAM::L_P_BRACKET => 0,
            // GRAM::R_P_BRACKET => 0,
        // COMPARISON OPERATORS
            // GRAM::EQUAL_TO => 0,
            // GRAM::EQUAL_VALUE => 0,
            // GRAM::GREATER_THAN => 0,
            // GRAM::GREATER_THAN_OR_EQUAL_TO => 0,
            // GRAM::LESS_THAN => 0,
            // GRAM::LESS_THAN_OR_EQUAL_TO => 0,
            // GRAM::NOT_EQUAL => 0,
            // GRAM::NOT_EQUAL_VALUE => 0,
        // OPERATORS: BITWISE
            // GRAM::BIT_AND => 0,
            // GRAM::BIT_OR => 0,
            // GRAM::BIT_NOT => 0,
            // GRAM::BIT_XOR => 0,
            // GRAM::BIT_L_SHIFT => 0,
            // GRAM::BIT_R_SHIFT => 0,
        // OPERATORS: LOGICAL
            // GRAM::AND => 0,
            // GRAM::OR => 0,
            // GRAM::NOT => 0,
        // OPERATORS: MATH
            // GRAM::ADDITION => 0,
            // GRAM::SUBTRACTION => 0,
            // GRAM::MULTIPLICATION => 0,
            // GRAM::DIVISION => 0,
            // GRAM::MODULUS => 0,
            // GRAM::INCREMENT => 0,
            // GRAM::DECREMENT => 0,
        // PUNCTUATION
            // GRAM::COLON => 0,
            // GRAM::COMMA => 0,
            // GRAM::PERIOD => 0,
            // GRAM::SEMI_COLON => 0,
        // WORD OPORATORS
            // GRAM::OP_DELETE => 0,
            // GRAM::OP_IN => 0,
            // GRAM::OP_INSTANCEOF => 0,
            // GRAM::OP_TYPEOF => 0,
            // GRAM::OP_VOID => 0,

        //ambiguous
            // GRAM::NAME => 0,
            // GRAM::ANY => 0,
            // GRAM::ANY_STRING => 0,
        // Directive
        DIRECTIVE_IMPORT => &ast_def_jess_import_directive,
        // What the heck is this, Panic!?
        NONE => &ast_def_jess_unknown
    };




    for token in source.split_whitespace() {
        log(token);

        // match token {
        //      => v,
        //     _:(v) => v
        // }
        // if ast_def_jess_import_directive.mathcer.call(&ast_def_jess_import_directive, &token) {
        //     log("found JESS_DIRECTIVE_IMPORT");
        //     continue;
        // }
    }

    // let tokens_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();


    // tokens_patern.replace_all(source, " $1");

    return String::from("");
}

mod tests {
    use super::{
        grammer_import_get_error,
        grammer_import_type,
        grammer_import_default,
        grammer_import_names
    };

    #[test]
    fn test_import_type_requested_import () {

        let import_type = grammer_import_type("@import");
        assert_eq!(import_type, "import");
    }
}