
extern crate wasm_bindgen;
extern crate nom;

mod jess_utils;

use jess_utils::matches::{
    name_braced_list::matches_common_curly_braced_list_comma_sep,
    css_directive::capture_common_css_directive_name,
    common_name::matches_common_name,
    quotes::matches_common_quoted
};

use std::boxed::Box;
use std::rc::Rc;

use regex::Regex;
use enum_map::{enum_map, Enum, EnumMap};


use std::collections::HashMap;
use std::fmt;

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

#[derive(Debug)]
pub enum LANGUAGE_CONTEXT {
    JS,
    CSS,
    BOTH
}

impl fmt::Display for LANGUAGE_CONTEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

#[derive(Debug, PartialEq)]
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
    // Ambiguous
    NAME,
    PANIC,
    AMBIGUOUS,
    IGNORE,
    EOF,
    // Directive
    DIRECTIVE_IMPORT,
    // Variable
    LET,
    CONST,
    // Values
    VAL_STRING
}
// allow toString
impl fmt::Display for GRAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
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

fn matcher_no_op (_self: &ASTNode, token: &str) -> String {
    return GRAM::IGNORE.to_string()
}

pub struct ASTNodeParams {
    id: GRAM,
    allow_next: Option<Vec<GRAM>>,
    token: Option<&'static str>,
    description: Option<&'static str>,
    matcher: Callback<String>
}

impl Default for ASTNodeParams {
    fn default() -> Self {
        return Self {
            id: GRAM::PANIC,
            token: None,
            allow_next: None,
            description: Some("No description provided"),
            matcher: Callback::from(matcher_no_op),
        }
    }
}

pub struct ASTNode {
    id: GRAM,
    allow_next: Option<Vec<GRAM>>,
    token: Option<&'static str>,
    description: &'static str,
    matcher: Callback<String>
}

impl ASTNode {
    // Standard definition
    pub fn def(p: ASTNodeParams) -> ASTNode {
        return ASTNode {
            id: p.id,
            allow_next: p.allow_next,
            token: p.token,
            description: p.description.unwrap_or_default(),
            matcher: p.matcher
        }
    }
}


#[wasm_bindgen]
pub fn ast(source: &str) -> String {

    // <token, GRAM.toString>
    let mut token_map: HashMap<&str, String> = HashMap::new();
    let mut AST_def_map: HashMap<String, &ASTNode> = HashMap::new();

    macro_rules! register_ast_node {
        ($ast_definition:ident) => {
            token_map.insert($ast_definition.token.unwrap(), $ast_definition.id.to_string());
            AST_def_map.insert($ast_definition.id.to_string(), &$ast_definition);
        };
    }

    //
    // EOF
    //
    let ast_def_eof = ASTNode::def(ASTNodeParams {
        token: Some("EOF"),
        id: GRAM::EOF,
        description: Some("End of file"),
        ..Default::default()
    });

    register_ast_node!(ast_def_eof);

    //
    // UNKNOWN grammer found
    // panic!
    let ast_def_jess_unknown = ASTNode::def(ASTNodeParams {
        id: GRAM::PANIC,
        description: Some("Uncaught SyntaxError: Unexpected token"),
        ..Default::default()
    });

    //
    // AMBIGUOUS grammer found
    // use matcher
    let ast_def_jess_ambiguous = ASTNode::def(ASTNodeParams {
        token: Some(""),
        id: GRAM::AMBIGUOUS,
        description: Some("Token is ambiguous, further processing required to identify or panic."),
        matcher: Callback::from(|_self: &ASTNode, token: &str| -> String {
            //TBA
            let (_input, is_common_identifier_name) = matches_common_name(token);
            let (_input, is_string) = matches_common_quoted(token);
            if is_common_identifier_name {
                log("Sub match:");
                return GRAM::NAME.to_string();
            } else if is_string {
                log("Sub match:");
                return GRAM::VAL_STRING.to_string();
            } else {
                log("");
                return GRAM::PANIC.to_string();
            }
        }),
        ..Default::default()
    });
    register_ast_node!(ast_def_jess_ambiguous);

    let ast_def_jess_import_directive = ASTNode::def(ASTNodeParams {
        token: Some("@import"),
        id: GRAM::DIRECTIVE_IMPORT,
        description: Some("Import things from another file"),
        ..Default::default()
    });
    register_ast_node!(ast_def_jess_import_directive);

    // PUNCTUATION defs

    //
    //  L_C_BRACE
    //
    let ast_def_l_c_brace = ASTNode::def(ASTNodeParams {
        token: Some("{"),
        id: GRAM::L_C_BRACE,
        description: Some("Grammer to open, or encapsulate blocks and notations."),
        ..Default::default()
    });
    register_ast_node!(ast_def_l_c_brace);

    //
    //  R_C_BRACE
    //
    let ast_def_r_c_brace = ASTNode::def(ASTNodeParams {
        token: Some("}"),
        id: GRAM::R_C_BRACE,
        description: Some("Grammer to close, or end encapsulate blocks and notations."),
        ..Default::default()
    });
    register_ast_node!(ast_def_r_c_brace);

    //
    // COLON
    //
    let ast_def_colon = ASTNode::def(ASTNodeParams {
        token: Some(":"),
        id: GRAM::COLON,
        description: Some("Used in terary operators, and to devide key values in object like constucts"),
        ..Default::default()
    });
    register_ast_node!(ast_def_colon);


    #[derive(Debug)]
    struct Token {
        value: &'static str,
        tokenID: &'static str,
    }

    impl ToString for Token {
        fn to_string(&self) -> String {
            return format!(r#"
            {{
                "value": "{value}",
                "tokenID": "{tokenID}"
            }},
            "#,
                value = self.value,
                tokenID = self.tokenID
            );
        }
    }

    let test = Token {
        value: "@",
        tokenID: "AT"
    };


    let mut collected_stream: Vec<Token> = vec!();

    fn token_stream_dump(token_stream: Vec<Token>) -> String {
        let json_incomplete: String = format!("[{:?}", token_stream.into_iter().map(|item: Token| -> String {
            item.to_string()
        }).collect::<String>());
        return json_incomplete;
    }
    collected_stream.push(test);
    log(&token_stream_dump(collected_stream));


    // Parse token streem
    let mut ittr_tokens = source.split_whitespace().peekable();
    let eof_token_id = "EOF";
    let panic_token_id = "PANIC";
    let ambiguous_token_id = GRAM::AMBIGUOUS.to_string();
    // Flag to enable panics, should be turned on at release
    let dont_panic = true;
    #[allow(irrefutable_let_patterns)]
    while let token = ittr_tokens.next() {

        let token_id = token_map.get(token.unwrap_or(eof_token_id)).unwrap_or(&ambiguous_token_id);

        // token was recognized
        log(&format!("{}\t\t{}", &token.unwrap_or(eof_token_id), &token_id));

        // This token is either not deterministic, eg names, strings and values or it is unknown and should panic!
        if token_id.to_string() == ambiguous_token_id {
            let nondeterministic_token_id = ast_def_jess_ambiguous.matcher.call(&ast_def_jess_ambiguous, &token.unwrap());
            log(&nondeterministic_token_id);

            // Throw Unexpected token error
            if &nondeterministic_token_id == panic_token_id {
                let err_msg = format!("{err} {token}", err = &ast_def_jess_unknown.description, token = &token.unwrap());
                log(&err_msg);
                if !dont_panic {
                    panic!();
                }
            }
        } else {
            // deterministic known token
        }

        if token_id == eof_token_id {
            break;
        }
    }

    //     log(token.unwrap());

    //     let next_token = ittr_tokens.peek();

    //     log(next_token.unwrap());

    //     // AMBIGUOUS
    //     // is used to determine values names, strings and more or even invalid syntax, anything not predictable.
    //     // ambiguous uses a match statment in its matcher callback
    //     let ambiguous_token = GRAM::AMBIGUOUS.to_string();
    //     let token_id_enum_as_string = token_map.get(token.unwrap()).unwrap_or(&ambiguous_token);
    //     let matched_token_AST_node: &ASTNode = AST_def_map.get(token_id_enum_as_string).unwrap();

    //     // further investivation required
    //     if matched_token_AST_node.id.to_string() == GRAM::AMBIGUOUS.to_string() {
    //         matched_token_AST_node.matcher.unwrap().call(matched_token_AST_node, token.unwrap());
    //     }

    //     log(matched_token_AST_node.description);
    //     log("\n");
    //     log("\n");
    // }
    // for token in source.split_whitespace().peekable() {

        // match token {
        //      => v,
        //     _:(v) => v
        // }
        // if ast_def_jess_import_directive.mathcer.call(&ast_def_jess_import_directive, &token) {
        //     log("found JESS_DIRECTIVE_IMPORT");
        //     continue;
        // }
    // }

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