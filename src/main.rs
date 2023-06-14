use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let code = r#"
    class Test {

        @Test
        void shouldMatch() {
           log.info("some initialization");
            new Expectations() {{
             mock1.mockMethod1(arg1, arg2);
             result = Mono.just("weird");

             mock2.mockMethod2(arg1, arg2);
             result = new RuntimeException("some reason");
            }};
           log.info("assertions go here");
        }

        @CanHaveAllTheOtherAnnotations
        @ParametrizedTest(value = "xxx")
        @CanHaveAllTheOtherAnnotations(withParameters = true)
        void shouldMatch() {
           log.info("some initialization");
            new Expectations() {{
             mock1.mockMethod1(arg1, arg2);
             result = Mono.just("weird");

             mock2.mockMethod2(arg1, arg2);
             result = new RuntimeException("some reason");
            }};
           log.info("assertions go here");
        }

        @CanHaveAllTheOtherAnnotations
        void shouldNotMatch() {
            new Expectations() {{
             mock1.mockMethod1(arg1, arg2);
             result = Mono.just("weird");

             mock2.mockMethod2(arg1, arg2);
             result = new RuntimeException("some reason");
            }};
        }

        @Test
        void shouldNotMatch() {
         new _Expectations() {{
         }};
        }

        void shouldNotMatch() {
            new Expectations() {{
             mock1.mockMethod1(arg1, arg2);
             result = Mono.just("weird");

             mock2.mockMethod2(arg1, arg2);
             result = new RuntimeException("some reason");
            }};
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
(method_declaration
  (modifiers
   [
    (marker_annotation name: (identifier) @marker-annotation (#match? @marker-annotation "^[A-Za-z]*Test$"))
    (annotation name: (identifier) @annotation (#match? @annotation "^[A-Za-z]*Test$"))
   ]
  )
  name: (identifier) @method
  (block (expression_statement)* (expression_statement (object_creation_expression type: (type_identifier) @type-name (#eq? @type-name "Expectations"))) (expression_statement)*)
)
    "#).expect("trying to build a query");
    //
    //()
    println!("Query pattern count: {}", query.pattern_count());
    for i in 0..query.pattern_count() {
        println!("Query pattern {}: {}", i, query.start_byte_for_pattern(i));
    }
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
        }
    }

}
