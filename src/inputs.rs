use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(&path)
        .expect(&format!("Failed to read file {}", &path))
}

pub fn test_input(day: u32) -> String {
    read_file(&format!("inputs/test/{}.txt", day))
}
pub fn task_input(day: u32) -> String {
    read_file(&format!("inputs/task/{}.txt", day))
}


mod tests {
    use super::*;

    #[test]
    fn test_inputs() {
        assert_eq!(test_input(1).len(), 54);
        assert_eq!(task_input(1).len(), 10459);
    }
}
