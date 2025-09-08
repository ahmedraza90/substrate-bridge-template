let my_string = String::from("hello");
println!("   Result: {:?}\n", my_string.as_str());


 println!("=== WHAT as_deref() ACTUALLY DOES ===\n");
    
    println!("as_deref() is available on:");
    println!("✅ Result<T, E> where T implements Deref");
    println!("✅ Option<T> where T implements Deref");
    println!();
    println!("Common T types that implement Deref:");
    println!("• String → &str");
    println!("• Vec<T> → &[T]");
    println!("• Box<T> → &T");
    println!("• PathBuf → &Path");
    println!("• OsString → &OsStr");