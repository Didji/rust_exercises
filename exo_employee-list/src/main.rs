use regex::Regex;
use std::collections::HashMap;

struct COMMAND {
    id: String,
    pattern: Regex,
}

fn main() {
    let input: Vec<&str> = vec![
        "Add Xavier to Dev",
        "Add Cynthia to HR",
        "Add Zoe to HR",
        "Add Clara to HR",
        "Add Luc to Direction",
        "Add Homer to Management",
        "Add Oliver to Management",
        "Add Alfred to Attendance",
        "Add Max to Attendance",
        "List Department Maintenance",
        "List Department Attendance",
        "List All",
        "Gloubiboulga",
    ];

    let available_commands = initialize_commands();

    //let mut instruction: String = String::new();
    let mut employees_by_department: HashMap<String, Vec<String>> = HashMap::new();

    for instruction in input {
        let used_command = match_instruction(instruction, &available_commands);

        if let None = used_command {
            println!("Please use a valid command");
            return;
        }

        let used_command = used_command.unwrap();
        if used_command.id == String::from("ADD_EMPLOYEE_TO_DEPARTMENT_COMMAND") {
            let result = find_employee_and_dep_to_add(instruction, used_command);
            match result {
                Ok((name, dep)) => {
                    employees_by_department
                        .entry(dep.clone())
                        .or_insert(Vec::new())
                        .push(name.clone());
                    println!("{} added to the department {}", name, dep);
                }
                Err(msg) => println!("{}", msg),
            }
        } else if used_command.id == String::from("LIST_DEPARTMENT_COMMAND") {
            let result = find_department_to_list(instruction, used_command);
            match result {
                Ok(dep) => println!("{} contains {:?}", &dep, employees_by_department.get(&dep).unwrap_or(&Vec::new())),
                Err(msg) => println!("{}", msg),
            }
        } else if used_command.id == String::from("LIST_ALL_COMMAND") {
            println!("Here are all the employees of the cmpany: {:?}", employees_by_department)
        }
    }
}

fn initialize_commands() -> Vec<COMMAND> {
    vec![
        COMMAND {
            id: String::from("ADD_EMPLOYEE_TO_DEPARTMENT_COMMAND"),
            pattern: Regex::new(r"^Add\s+(?P<name>.+)\s+to\s+(?P<dep>.+)$").unwrap(),
        },
        COMMAND {
            id: String::from("LIST_DEPARTMENT_COMMAND"),
            pattern: Regex::new(r"^List Department\s+(?P<dep>.+)$").unwrap(),
        },
        COMMAND {
            id: String::from("LIST_ALL_COMMAND"),
            pattern: Regex::new("^List All$").unwrap(),
        },
    ]
}

fn match_instruction<'a>(instruction: &str, available_commands: &'a Vec<COMMAND>) -> Option<&'a COMMAND> {
    available_commands
        .into_iter()
        .find(|command| command.pattern.is_match(instruction))
}

fn find_employee_and_dep_to_add(
    instruction: &str,
    command: &COMMAND,
) -> Result<(String, String), String> {
    let captures = command.pattern.captures(instruction);

    if let None = captures {
        return Err(String::from("Impossible to parse this command, cancelling"));
    }
    let captures = captures.unwrap();
    Ok((
        captures.name("name").map_or("", |m| m.as_str()).to_string(),
        captures.name("dep").map_or("", |m| m.as_str()).to_string(),
    ))
}

fn find_department_to_list(instruction: &str, command: &COMMAND) -> Result<String, String> {
    let captures = command.pattern.captures(instruction);

    if let None = captures {
        return Err(String::from("Impossible to parse this command, cancelling"));
    }

    let captures = captures.unwrap();
    Ok(captures.name("dep").map_or("", |m| m.as_str()).to_string())
}
