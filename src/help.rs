pub fn search<'a>(tasks: &'a [super::Task], q: &super::Search) -> Vec<&'a super::Task> {
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