use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let code = r#"
    class Test {

        @Test
        void testSomething() {
            new Expectations() {{
             mock1.mockMethod1(arg1, arg2);
             result = Mono.just("weird");

             mock2.mockMethod2(arg1, arg2);
             result = new RuntimeException("some reason");
            }};
        }


        @Test
        void testSomething2() {
           log.info("We're done");
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
  body: (class_body
    (method_declaration
      (modifiers (marker_annotation name: (identifier) @annotation))
      name: (identifier) @the-method-name))
    "#).expect("trying to build a query");

    let test_annotation_idx = query.capture_index_for_name("annotation").expect("capture for annotation must exist in a query");
    for my_match in query_cursor.matches(&query, parsed.root_node(), code.as_bytes()) {
        for capture in my_match.captures.iter().filter(|&c| c.index != test_annotation_idx) {
            let range = capture.node.range();
            let text = &code[range.start_byte..range.end_byte];
            let line = range.start_point.row;
            let col = range.start_point.column;
            println!(
                "[Line: {}, Col: {}] Test method name: `{}`",
                line, col, text
            );
        }
    }

}
