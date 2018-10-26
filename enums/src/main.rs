enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn is_it_already_friday(day: Day) -> bool {
    match day {
        // It is Friday!
        Day::Friday => true,
        // Underscore is equivalent to the default case in switch statements
        _ => false,
    }
}

enum Figure {
    Square(f32),
    Rectangle(f32, f32),
}

fn calculate_area(figure: Figure) -> f32 {
    match figure {
        Figure::Square(side) => side * side,
        Figure::Rectangle(width, height) => width * height,
    }
}

fn main() {
    println!("Hello, world!");
}
