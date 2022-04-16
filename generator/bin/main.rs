use generator_lib::c_preprocessor::PreprocessorContext;

fn main() {
    let a = r#"
    #if (3 * 5 + 5) * 2 - 5 * 8
    typedef honk_t u84_t;
    #else
    help
    #endif
    "#;

    let mut tokens = generator_lib::c_preprocessor::lex(a);
    // println!("{:#?}", tokens);
    let mut pr = PreprocessorContext::new();

    pr.process(&mut tokens);

    println!("{:#?}", tokens);
    // let (parsed, errors) = vk_parse::parse_file(Path::new("./vk.xml")).unwrap();
    // println!("{:#?}", parsed);
}
