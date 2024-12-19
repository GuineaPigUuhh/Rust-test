fn main()
{
    let programming_languages = vec!["Haxe", "Factorio", "Ruby", "Rust", "Lua", "God Slime"];
    let real_programming_languages = vec!["Haxe", "Ruby", "Rust", "Lua"];
    
    for i in programming_languages.into_iter()
    {
        if real_programming_languages.contains(&i)
        {
            println!("{}", i);
            continue;
        }
        println!("INVALID: {}", i);
    }
}