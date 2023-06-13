use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let code = r#"
    class Test {

        @ParametrizedTest
        void testSomething() {
           log.info("some initialization");
            new Expectations() {{
             mock1.mockMethod1(arg1, arg2);
             result = Mono.just("weird");

             mock2.mockMethod2(arg1, arg2);
             result = new RuntimeException("some reason");
            }};
           log.info("assertions go here");
        }


        @Test
        void testSomething2() {
        }
    }
"#;

    let mut parser = Parser::new();
    let java = tree_sitter_java::language();
    parser.set_language(java).expect("Error loading Java grammar");
    let parsed = parser.parse(code, None).expect("Failed to parse sample snippet");
    println!("{:?}", parsed.root_node().to_sexp());

    let mut query_cursor = QueryCursor::new();

    let query = Query::new(java, r#"
(class_body
  (method_declaration (modifiers (marker_annotation name: (identifier) @annotation (#eq? @annotation "Test")))
    name: (identifier) @the-method-name))
    body: (block (expression_statement)* (expression_statement (object_creation_expression type: (type_identifier) @type-name)) (expression_statement)*)
    "#).expect("trying to build a query");
    for my_match in query_cursor.matches(&query, parsed.root_node(), code.as_bytes()) {
        println!("Match with index #{}: ", my_match.pattern_index);
        for capture in my_match.captures.iter() {
            let range = capture.node.range();
            let node = capture.node;
            let text = &code[range.start_byte..range.end_byte];
            let line = range.start_point.row;
            let col = range.start_point.column;
            println!(
                "\t[Line: {}, Col: {}] Capture `{}` with index {} is `{}` with node {:?}",
                line, col, query.capture_names()[capture.index as usize], capture.index, text, node
            );

            if capture.index == 1 {
                println!("{:?}", node.parent().unwrap());
            }
        }
    }

}
