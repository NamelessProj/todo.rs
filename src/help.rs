use crate::command::Search;

pub fn search<'a>(tasks: &'a [super::Task], q: &Search) -> Vec<&'a super::Task> {
    if let Some(id) = q.id {
        return tasks.iter().filter(|t| t.id == id).collect();
    }

    let text = q.text.as_deref().unwrap_or("").to_lowercase();

    let (in_name, in_desc) = if !q.name && !q.description {
        (true, true)
    } else {
        (q.name, q.description)
    };

    tasks.iter().filter(|t| {
        (in_name && t.name.to_lowercase().contains(&text)) || (in_desc && t.description.as_ref().is_some_and(|d| d.to_lowercase().contains(&text)))
    }).collect()
}

pub fn print_tasks(tasks: &[super::Task]) {
    if tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    for task in tasks {
        println!("{task}");
    }
}

pub fn next_id(todos: &[super::Task]) -> u32 {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}