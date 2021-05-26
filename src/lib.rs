// use anyhow::Result;

// pub fn find_matches(reader: impl std::io::BufRead, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
//     for line in reader.lines() {
//         if let Ok(l) = line {
//             if l.contains(pattern) {
//                 writeln!(writer, "{}", l)?;
//             }
//         }
//     }

//     Ok(())
// }

// #[test]
// fn find_a_match() {
//     let mut result = vec![];
//     find_matches("In principio erat Verbum,\net Verbum erat apud Deum,\net Deus erat Verbum.".as_bytes(), "Deum", &mut result);
//     assert_eq!(result, b"et Verbum erat apud Deum,\n");
// }
