fn next_language<'a> (languages: & 'a [String], current: &str) ->  &'a str{

    let mut found = false;

    for language in languages {
        if found {
            return language;
        }
        if language == current {
            found =true
        }
    }
        languages.last().unwrap()
}

fn last_language (languages: &[String]) -> &str {
    languages.last().unwrap()

}

fn longest_language<'a> (lang_a: &'a str, lang_b: &'a str) -> &'a str{
    if lang_a.len() >= lang_b.len() {
        lang_a
    }else {
        lang_b
    }
}


fn main() {

    let languages = vec![
        String::from("Rust"),
        String::from("Go"),
        String::from("Typescript")
    ];
    let next_language = next_language(&languages, "Go");
    let last_language = last_language(&languages);
    let  longest_language = longest_language("go", "typescript");

    println!("{:#?}", next_language);
    println!("{:#?}", last_language);
    println!("{:#?}", longest_language)
}
