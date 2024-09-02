use askama_axum::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn hello_world() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}
