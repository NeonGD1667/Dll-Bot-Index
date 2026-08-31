use std::fs;

fn main() {
    println!("🤖 Dll Bot Macro Scanner");
    println!("Scanning for macros...\n");

    let folders = ["gdr", "gdr2", "json", "slc2", "slc3"];
    let mut total = 0;

    for folder in folders {
        let path = format!("macros/{folder}");

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    println!("📦 Found: {}", entry.path().display());
                    total += 1;
                }
            }
        }
    }

    println!("\n🔥 Total macros: {total}");

    if total == 0 {
        println!("💀 Bro has no macros.");
    } else {
        println!("✅ Macro Index is alive!");
    }
}
