molde::molde!("tests/example.html");

#[test]
fn it_works() {
    let example = Example {
        title: "Usuários".to_owned(),
        warning: Some(ExampleWarning {
            text: "Socorro".to_owned(),
        }),
        empty: true,
        users: vec![
            ExampleUsers {
                name: "Alicia".to_owned(),
                color: "red".to_owned(),
            },
            ExampleUsers {
                name: "Diane".to_owned(),
                color: "blue".to_owned(),
            },
        ],
    };

    let output = std::fs::read_to_string("tests/output.html").unwrap();
    assert_eq!(example.render(), output);
}
