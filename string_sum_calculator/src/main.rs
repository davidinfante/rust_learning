mod add_functions;

fn main() {
    let numbers_string = String::from("");
    add_functions::test_add(numbers_string, 0);
    let numbers_string = String::from("4");
    add_functions::test_add(numbers_string, 4);
    let numbers_string = String::from("1,2");
    add_functions::test_add(numbers_string, 3);
    let numbers_string = String::from("1,2,3,4,5,6,7,8,9");
    add_functions::test_add(numbers_string, 45);
    let numbers_string = String::from("1\n2,3");
    add_functions::test_add(numbers_string, 6);
    let numbers_string = String::from("//;\n1;2");
    add_functions::test_add(numbers_string, 3);
    let numbers_string = String::from("1,-2,-3");
    add_functions::test_add(numbers_string, 1);
    let numbers_string = String::from("1001,2");
    add_functions::test_add(numbers_string, 2);
    let numbers_string = String::from("1001,2,999,777");
    add_functions::test_add(numbers_string, 1778);
    let numbers_string = String::from("//[***]\n1***2***3");
    add_functions::test_add(numbers_string, 6);
    let numbers_string = String::from("//[*][%]\n1*2%3");
    add_functions::test_add(numbers_string, 6);
    let numbers_string = String::from("//[foo][bar]\n1foo2bar3");
    add_functions::test_add(numbers_string, 6);
}
