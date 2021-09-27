use crate::parser::grammar::{argument, input, name};
use crate::{Parser, SyntaxKind, TokenKind, S, T};

/// See: https://spec.graphql.org/June2018/#DirectiveDefinition
///
/// ```txt
/// DirectiveDefinition
///     Description(opt) directive @ Name ArgumentsDefinition(opt) repeatable(opt) on DirectiveLocations
/// ```
pub(crate) fn directive_definition(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::DIRECTIVE_DEFINITION);
    // TODO @lrlna: parse Description
    p.bump(SyntaxKind::directive_KW);
    match p.peek() {
        Some(T![@]) => p.bump(S![@]),
        _ => p.err("expected @ symbol"),
    }
    name::name(p);

    if let Some(T!['(']) = p.peek() {
        let _g = p.start_node(SyntaxKind::ARGUMENTS_DEFINITION);
        p.bump(S!['(']);
        input::input_value_definition(p, false);
        p.expect(T![')'], S![')']);
    }

    if let Some(node) = p.peek_data() {
        if node.as_str() == "repeatable" {
            p.bump(SyntaxKind::repeatable_KW);
        }
    }

    if let Some(node) = p.peek_data() {
        match node.as_str() {
            "on" => p.bump(SyntaxKind::on_KW),
            _ => p.err("expected Directive Locations"),
        }
    }

    if let Some(TokenKind::Name | T![|]) = p.peek() {
        let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATIONS);
        directive_locations(p, false);
    } else {
        p.err("expected valid Directive Location");
    }
}

/// See: https://spec.graphql.org/June2018/#DirectiveLocations
pub(crate) fn directive_locations(p: &mut Parser, is_location: bool) {
    if let Some(T![|]) = p.peek() {
        p.bump(S![|]);
        directive_locations(p, is_location)
    }

    if let Some(TokenKind::Name) = p.peek() {
        let loc = p.peek_data().unwrap();
        match loc.as_str() {
            "MUTATION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::QUERY_KW);
            }
            "SUBSCRIPTION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::SUBSCRIPTION_KW);
            }
            "FIELD" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::FIELD_KW);
            }
            "FRAGMENT_DEFINITION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::FRAGMENT_DEFINITION_KW);
            }
            "FRAGMENT_SPREAD" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::FRAGMENT_DEFINITION_KW);
            }
            "INLINE_FRAGMENT" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::INLINE_FRAGMENT_KW);
            }
            "SCHEMA" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::SCHEMA_KW);
            }
            "SCALAR" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::SCALAR_KW);
            }
            "OBJECT" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::OBJECT_KW);
            }
            "FIELD_DEFINITION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::FIELD_DEFINITION_KW);
            }
            "ARGUMENT_DEFINITION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::ARGUMENT_DEFINITION_KW);
            }
            "INTERFACE" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::INTERFACE_KW);
            }
            "UNION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::UNION_KW);
            }
            "ENUM" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::ENUM_KW);
            }
            "ENUM_VALUE" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::ENUM_VALUE_KW);
            }
            "INPUT_OBJECT" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::INPUT_OBJECT_KW);
            }
            "INPUT_FIELD_DEFINITION" => {
                let _g = p.start_node(SyntaxKind::DIRECTIVE_LOCATION);
                p.bump(SyntaxKind::INPUT_FIELD_DEFINITION_KW);
            }
            _ => {
                if !is_location {
                    p.err("expected valid Directive Location");
                }
                return;
            }
        }
        if p.peek_data().is_some() {
            return directive_locations(p, true);
        }
    }
    if !is_location {
        p.err("expected Directive Locations");
    }
}

/// See: https://spec.graphql.org/June2018/#Directive
///
/// ```txt
/// Directive
///     @ Name Arguments
/// ```
pub(crate) fn directive(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::DIRECTIVE);

    p.expect(T![@], S![@]);
    name::name(p);

    if let Some(T!['(']) = p.peek() {
        argument::arguments(p);
    }
}

pub(crate) fn directives(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::DIRECTIVES);
    while let Some(T![@]) = p.peek() {
        directive(p);
    }
}

// TODO @lrlna: inlined collapsed AST should live in a 'fixtures' dir for ease of testing
#[cfg(test)]
mod test {
    use crate::parser::utils;

    #[test]
    fn it_returns_errors_and_full_ast_when_location_is_missing() {
        utils::check_ast(
            "directive @example on",
            r#"
            - DOCUMENT@0..21
                - DIRECTIVE_DEFINITION@0..21
                    - directive_KW@0..9 "directive"
                    - WHITESPACE@9..10 " "
                    - AT@10..11 "@"
                    - NAME@11..19
                        - IDENT@11..18 "example"
                        - WHITESPACE@18..19 " "
                    - on_KW@19..21 "on"
            - ERROR@0:3 "expected valid Directive Location"
            "#,
        );
    }

    // TODO @lrlna: these tests need to check for indentation as part of the
    // output, not just the nodes of the tree
    #[test]
    fn it_parses_directive_definition() {
        utils::check_ast(
            "directive @example(isTreat: Boolean, treatKind: String) on FIELD | MUTATION",
            r#"
            - DOCUMENT@0..75
                - DIRECTIVE_DEFINITION@0..75
                    - directive_KW@0..9 "directive"
                    - WHITESPACE@9..10 " "
                    - AT@10..11 "@"
                    - NAME@11..18
                        - IDENT@11..18 "example"
                    - ARGUMENTS_DEFINITION@18..55
                        - L_PAREN@18..19 "("
                        - INPUT_VALUE_DEFINITION@19..35
                            - NAME@19..26
                                - IDENT@19..26 "isTreat"
                            - COLON@26..27 ":"
                            - WHITESPACE@27..28 " "
                            - TYPE@28..35
                                - NAMED_TYPE@28..35
                                    - NAME@28..35
                                        - IDENT@28..35 "Boolean"
                        - COMMA@35..36 ","
                        - WHITESPACE@36..37 " "
                        - INPUT_VALUE_DEFINITION@37..54
                            - NAME@37..46
                                - IDENT@37..46 "treatKind"
                            - COLON@46..47 ":"
                            - WHITESPACE@47..48 " "
                            - TYPE@48..54
                                - NAMED_TYPE@48..54
                                    - NAME@48..54
                                        - IDENT@48..54 "String"
                        - R_PAREN@54..55 ")"
                    - WHITESPACE@55..56 " "
                    - on_KW@56..58 "on"
                    - WHITESPACE@58..59 " "
                    - DIRECTIVE_LOCATIONS@59..75
                        - DIRECTIVE_LOCATION@59..65
                            - FIELD_KW@59..64 "FIELD"
                            - WHITESPACE@64..65 " "
                        - PIPE@65..66 "|"
                        - WHITESPACE@66..67 " "
                        - DIRECTIVE_LOCATION@67..75
                            - QUERY_KW@67..75 "MUTATION"
            "#,
        );
    }

    // TODO @lrlna: enable the "repeatable" graphql extension
    //
    // See: https://spec.graphql.org/draft/#sec-Type-System.Directives
    #[test]
    fn it_parses_repeatable_nodes() {
        utils::check_ast(
            "directive @example(isTreat: Boolean, treatKind: String) repeatable on FIELD | MUTATION",
            r#"
            - DOCUMENT@0..86
                - DIRECTIVE_DEFINITION@0..86
                    - directive_KW@0..9 "directive"
                    - WHITESPACE@9..10 " "
                    - AT@10..11 "@"
                    - NAME@11..18
                        - IDENT@11..18 "example"
                    - ARGUMENTS_DEFINITION@18..55
                        - L_PAREN@18..19 "("
                        - INPUT_VALUE_DEFINITION@19..35
                            - NAME@19..26
                                - IDENT@19..26 "isTreat"
                            - COLON@26..27 ":"
                            - WHITESPACE@27..28 " "
                            - TYPE@28..35
                                - NAMED_TYPE@28..35
                                    - NAME@28..35
                                        - IDENT@28..35 "Boolean"
                        - COMMA@35..36 ","
                        - WHITESPACE@36..37 " "
                        - INPUT_VALUE_DEFINITION@37..54
                            - NAME@37..46
                                - IDENT@37..46 "treatKind"
                            - COLON@46..47 ":"
                            - WHITESPACE@47..48 " "
                            - TYPE@48..54
                                - NAMED_TYPE@48..54
                                    - NAME@48..54
                                        - IDENT@48..54 "String"
                        - R_PAREN@54..55 ")"
                    - WHITESPACE@55..56 " "
                    - repeatable_KW@56..66 "repeatable"
                    - WHITESPACE@66..67 " "
                    - on_KW@67..69 "on"
                    - WHITESPACE@69..70 " "
                    - DIRECTIVE_LOCATIONS@70..86
                        - DIRECTIVE_LOCATION@70..76
                            - FIELD_KW@70..75 "FIELD"
                            - WHITESPACE@75..76 " "
                        - PIPE@76..77 "|"
                        - WHITESPACE@77..78 " "
                        - DIRECTIVE_LOCATION@78..86
                            - QUERY_KW@78..86 "MUTATION"
            "#,
        );
    }
}