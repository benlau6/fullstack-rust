use askama_axum::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn hello_world() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;
