fn student_to_index(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Invalid student {student}"),
    }
}

fn letter_to_plant(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Invalid plant letter, {c}"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = student_to_index(student);
    let cup_index = student_index * 2;
    let rows: Vec<&str> = diagram.lines().collect::<Vec<_>>();
    if rows.len() != 2 {
        panic!("Illegal diagram, length was {} instead of 2", rows.len());
    }
    rows.iter()
        .flat_map(|row| {
            // Pick out the student's plants in each row and convert to friendly names.
            row[cup_index..cup_index + 2].chars().map(letter_to_plant)
        })
        .collect::<Vec<&'static str>>()
}
