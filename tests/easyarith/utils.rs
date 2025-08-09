use std::fs;
use std::panic;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Expect {
    Ok,
    Err,
}

pub fn test_dir<T>(dir: &str, expect: Expect, test_fn: &T)
where
    T: Fn(&str) -> anyhow::Result<()> + panic::RefUnwindSafe,
{
    let mut entries = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| { path.is_file() })
        .map(|path| {
            let body = fs::read_to_string(&path).unwrap();
            (path, body)
        })
        .collect::<Vec<_>>();
    entries.sort();

    for (path, body) in entries {
        print!("Testing {:?} ... ", path);
        let result = panic::catch_unwind(|| test_fn(&body).unwrap());
        match result {
            Ok(_) if expect == Expect::Err => {
                println!("Failed (expected Error, but got Ok)");
                panic!("");

            }
            Err(e) if expect == Expect::Ok => {
                println!("expected Ok, but got Error.");
                panic!("{}", e.downcast_ref::<anyhow::Error>().unwrap());
            }
            _ => println!("Ok"),
        }
    }
}
