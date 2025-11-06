
fn main() {
    println!("=== PART 1: .iter() - BORROWING ===\n");
    demo_iter_borrowing();
    
    println!("\n=== PART 2: .into_iter() - CONSUMING ===\n");
    demo_into_iter_consuming();
    
    println!("\n=== PART 3: Where did it go? ===\n");
    demo_memory_explanation();
}

fn demo_iter_borrowing() {
    let numbers = vec![1, 2, 3];
    
    println!("Original vector: {:?}", numbers);
    println!("Memory address: {:p}\n", &numbers);
    
    // .iter() just LOOKS at the values
    for num in numbers.iter() {
        println!("  Looking at: {} (address: {:p})", num, num);
        // num is &i32 - just a pointer, doesn't own anything
    }
    
    // Numbers still exists!
    println!("\n✅ After .iter(), vector still exists:");
    println!("Original vector: {:?}", numbers);
    println!("Memory address: {:p}", &numbers);
}

fn demo_into_iter_consuming() {
    let numbers = vec![1, 2, 3];
    
    println!("Original vector: {:?}", numbers);
    println!("Memory address: {:p}\n", &numbers);
    
    // .into_iter() TAKES OWNERSHIP
    println!("Calling .into_iter()...\n");
    
    for num in numbers.into_iter() {
        println!("  Took ownership of: {}", num);
        // num is i32 - we OWN this value now
        // The vector is being dismantled piece by piece!
    }
    
    // Try to use numbers here - UNCOMMENT TO SEE ERROR:
    // println!("Original vector: {:?}", numbers);
    // ERROR: borrow of moved value: `numbers`
    
    println!("\n❌ After .into_iter(), vector is GONE!");
    println!("Can't use 'numbers' anymore!");
}

fn demo_memory_explanation() {
    // Let's see what REALLY happens with into_iter
    
    println!("Creating a vector of Strings (expensive type):");
    let words = vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ];
    
    println!("Before into_iter: words = {:?}\n", words);
    
    // What into_iter does:
    // 1. Takes the Vec<String>
    // 2. Gives you each String one by one
    // 3. Vec is empty/gone at the end
    
    println!("During into_iter:");
    for (i, word) in words.into_iter().enumerate() {
        println!("  Step {}: Got '{}' (this String is now MINE!)", i + 1, word);
        // 'word' is now owned by this iteration
        // When this iteration ends, 'word' is dropped (memory freed)
    }
    
    println!("\nAfter loop: All Strings have been moved out and dropped!");
    
    // words is GONE - can't use it anymore
    // Uncomment to see error:
    // println!("words = {:?}", words);
    
    println!("\n=== VISUAL EXPLANATION ===\n");
    visual_explanation();

}

// BONUS: Visual representation
fn visual_explanation() {
    println!("\n=== VISUAL EXPLANATION ===\n");
    
    println!("Memory before:");
    println!("┌─────────────┐");
    println!("│ Vec 'numbers'│");
    println!("├─────────────┤");
    println!("│     1       │ ← owns this");
    println!("│     2       │ ← owns this");
    println!("│     3       │ ← owns this");
    println!("└─────────────┘");
    
    println!("\nWith .iter():");
    println!("┌─────────────┐");
    println!("│ Vec 'numbers'│");
    println!("├─────────────┤");
    println!("│     1       │ ← still owns");
    println!("│     2       │ ← still owns");
    println!("│     3       │ ← still owns");
    println!("└─────────────┘");
    println!("     ↑");
    println!("     └── Iterator just POINTS to these");
    
    println!("\nWith .into_iter():");
    println!("┌─────────────┐");
    println!("│ Vec 'numbers'│ ← DESTROYED!");
    println!("└─────────────┘");
    println!("     ↓");
    println!("     1 → given to you (you own it now)");
    println!("     2 → given to you (you own it now)");
    println!("     3 → given to you (you own it now)");
}
