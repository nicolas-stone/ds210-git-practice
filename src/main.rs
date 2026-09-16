/// Your crew's name. Both of you are going to change this line.
const CREW_NAME: &str = "git master crew";

/// Your crew's motto. You will both change this one too, earlier and separately.
const MOTTO: &str = "new motto is evelyn";

fn main() {
    println!("=== {} ===", CREW_NAME);
    println!();
    println!("Crew roster:");

    // ROSTER: replace the line below with one for yourself.
    println!("  -Nicolas Stone");

    println!();
    println!("Motto: {}", MOTTO);
    println!("Report any problems to whoever merged last.");
}
