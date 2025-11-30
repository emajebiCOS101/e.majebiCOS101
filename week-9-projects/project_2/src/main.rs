use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Student Management Information Sytem.txt").expect("create failed");

    file.write_all("\t\t\tPAU SMIS\n\n".as_bytes()).unwrap();
    let student_name = vec!["Student Name", "Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_number = vec!["Matric. Number", "ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Department", "Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["Level", "300", "100", "200", "200", "100"];

    let max_len = student_name.len().max(matric_number.len()).max(department.len()).max(level.len());

    for i in 0..max_len {
        if i < student_name.len() {
            file.write_all(student_name[i].as_bytes()).unwrap();
        }
        file.write_all("\t\t".as_bytes()).unwrap();

        if i < matric_number.len() {
            file.write_all(matric_number[i].as_bytes()).unwrap();
        }
        file.write_all("\t\t".as_bytes()).unwrap();

        if i < department.len() {
            file.write_all(department[i].as_bytes()).unwrap();
        }
        file.write_all("\t\t".as_bytes()).unwrap();

        if i < level.len() {
            file.write_all(level[i].as_bytes()).unwrap();
        }

        file.write_all("\n".as_bytes()).unwrap();
    }
    println!("Student data has been updated to the system");
}
