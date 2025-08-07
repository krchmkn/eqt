use std::env;

#[derive(Debug)]
enum Input<'a> {
    Number(f64),
    Text(&'a str),
}

impl<'a> Input<'a> {
    fn get(s: &'a str) -> Self {
        if let Ok(n) = s.parse::<f64>() {
            Input::Number(n)
        } else {
            Input::Text(s)
        }
    }
}

trait Compare {
    fn compare_texts(a: &str, b: &str) -> String;

    fn compare_numbers(a: f64, b: f64) -> String;

    fn compare(a: Input, b: Input) -> String;
}

impl Compare for Input<'_> {
    fn compare_texts(a: &str, b: &str) -> String {
        if a == b {
            return format!("{} == {}", a, b);
        }
        format!("{} != {}", a, b)
    }

    fn compare_numbers(a: f64, b: f64) -> String {
        if a == b {
            return format!("{} == {}", a, b);
        } else if a > b {
            return format!("{} > {}", a, b);
        }
        format!("{} < {}", a, b)
    }

    fn compare(a: Input, b: Input) -> String {
        match (a, b) {
            (Input::Number(a), Input::Number(b)) => format!("{}", Input::compare_numbers(a, b)),
            (Input::Text(a), Input::Text(b)) => format!("{}", Input::compare_texts(a, b)),
            (Input::Text(a), Input::Number(b)) => {
                format!("{}", Input::compare_texts(a, &b.to_string()))
            }
            (Input::Number(a), Input::Text(b)) => {
                format!("{}", Input::compare_texts(&a.to_string(), b))
            }
        }
    }
}

static MESSAGE: &str = "Two arguments required.";

fn main() {
    let mut args = env::args().skip(1);

    if args.len() < 2 {
        eprintln!("{}", MESSAGE);
        return;
    }

    let arg1 = args.next().unwrap_or_else(|| panic!("{}", MESSAGE));
    let arg2 = args.next().unwrap_or_else(|| panic!("{}", MESSAGE));

    println!("{:?}", Input::compare(Input::get(&arg1), Input::get(&arg2)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        let _num = Input::get("1");
        assert!(matches!(Input::Number(1.0), _num));

        let _num = Input::get("0.1");
        assert!(matches!(Input::Number(0.1), _num));

        let _str = Input::get("abc");
        assert!(matches!(Input::Text("abc"), _str));

        let _str = Input::get("acb");
        assert!(matches!(Input::Text("acb"), _str));
    }

    #[test]
    fn test_compare_numbers() {
        assert_eq!("0.1 == 0.1", Input::compare_numbers(0.1, 0.1));
        assert_eq!("1.2 > 0.3", Input::compare_numbers(1.2, 0.3));
        assert_eq!("0 < 1", Input::compare_numbers(0.0, 1.0));
    }

    #[test]
    fn test_compare_texts() {
        assert_eq!("abc == abc", Input::compare_texts("abc", "abc"));
        assert_eq!("abc != acb", Input::compare_texts("abc", "acb"));
        assert_eq!("abc != 1", Input::compare_texts("abc", "1"));
        assert_eq!("0 != acb", Input::compare_texts("0", "acb"));
    }

    #[test]
    fn test_compare() {
        assert_eq!(
            "1 != abc",
            Input::compare(Input::get("1"), Input::get("abc"))
        );
        assert_eq!(
            "abc == abc",
            Input::compare(Input::get("abc"), Input::get("abc"))
        );
        assert_eq!("1 == 1", Input::compare(Input::get("1"), Input::get("1")));
    }
}
