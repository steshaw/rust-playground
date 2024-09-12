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
    let rows = diagram.lines().collect::<Vec<&str>>();
    if rows.len() != 2 {
        panic!("Illegal diagram, length was {} instead of 2", rows.len());
    }
    // Pick out the student's plants in each row.
    let row1 = rows[0][student_index * 2..student_index * 2 + 2]
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let row2 = rows[1][student_index * 2..student_index * 2 + 2]
        .to_string()
        .chars()
        .collect::<Vec<char>>();

    // Convert those plants to friendly names.
    vec![row1[0], row1[1], row2[0], row2[1]]
        .iter()
        .map(|c| letter_to_plant(*c))
        .collect::<Vec<&'static str>>()
}
