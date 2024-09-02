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

pub async fn login_page() -> LoginTemplate {
    LoginTemplate
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;

pub async fn register_page() -> RegisterTemplate {
    RegisterTemplate
}
