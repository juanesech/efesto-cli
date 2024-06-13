use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

pub fn create(name: String) {
    println!("Creating data source {}DS", name);
    let mut template = TinyTemplate::new();
    template
        .add_template("create", include_str!("data_source.py.tpl"))
        .unwrap();
    let rendered = template
        .render(
            "create",
            &TemplateContext {
                name: name.to_string(),
            },
        )
        .unwrap();
    std::fs::write(
        format!("src/data_sources/{}.py", name.to_ascii_lowercase()),
        rendered,
    )
    .unwrap();
}

//pub fn create(name: &str) -> Result<(), Box<dyn Error>> {
//    let mut template = TinyTemplate::new();
//    template
//        .add_template("create", include_str!("create.tmpl"))
//        .unwrap();
//    let rendered = template.render(
//        "create",
//        &TemplateContext {
//            name: name.to_string(),
//        },
//    )?;
//    std::fs::write(format!("src/data_sources/{}.rs", name), rendered)?;
//    Ok(())
//}
