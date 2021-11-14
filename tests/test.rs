molde::molde!("tests/example.html");

#[test]
fn it_works_1() {
    let example = Example {
        title: "Court".to_owned(),
        plead: Some(ExamplePlead {
            text: "Not guilty".to_owned(),
        }),
        verdict: Some(ExampleVerdict { guilty: true }),
        valid: true,
        lawyers: vec![
            ExampleLawyers {
                name: "Alicia".to_owned(),
                color: "red".to_owned(),
            },
            ExampleLawyers {
                name: "Diane".to_owned(),
                color: "blue".to_owned(),
            },
        ],
        judges: vec![ExampleJudges {
            name: "Cuesta".to_owned(),
        }],
    };

    let output = std::fs::read_to_string("tests/output1.html").unwrap();
    assert_eq!(example.render(), output);
}

#[test]
fn it_works_2() {
    let example = Example {
        title: "Yard".to_owned(),
        plead: None,
        verdict: None,
        valid: false,
        lawyers: vec![],
        judges: vec![],
    };

    let output = std::fs::read_to_string("tests/output2.html").unwrap();
    assert_eq!(example.render(), output);
}
