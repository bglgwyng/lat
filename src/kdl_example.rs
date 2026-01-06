// let doc_str = r#"
// hello 1 2 3

// // Comment
// world prop=string-value {
// child 1
// child 2
// child #inf
// }
// "#;

// let doc: KdlDocument = doc_str.parse().expect("failed to parse KDL");

// assert_eq!(
//     doc.iter_args("hello").collect::<Vec<&KdlValue>>(),
//     vec![&1.into(), &2.into(), &3.into()]
// );

// assert_eq!(
//     doc.get("world").map(|node| &node["prop"]),
//     Some(&"string-value".into())
// );

// // Documents fully roundtrip:
// assert_eq!(doc.to_string(), doc_str);
// println!("{}", doc);
