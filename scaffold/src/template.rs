#[derive(Debug, Clone, Copy)]
pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
}

pub fn catalog() -> &'static [Template] {
    &[
        Template {
            name: "bin",
            description: "Dependency-free binary application",
        },
        Template {
            name: "lib",
            description: "Library with a unit test",
        },
    ]
}

pub fn lookup(name: &str) -> Result<&'static Template, String> {
    catalog()
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| format!("unknown template {name:?}; see `rusty templates`"))
}
