#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn hello(name: String) -> String {
    let str = String::from(["hello from? ", &name].concat());
    str
}
