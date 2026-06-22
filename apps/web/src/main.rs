#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

mod app;
mod components;
mod i18n;
mod pages;
mod storage;

#[cfg(target_arch = "wasm32")]
fn main() {
    let root = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("app"))
        .expect("index.html must contain #app");

    yew::Renderer::<app::App>::with_root(root).render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Rust 学习站前端需要使用 wasm32-unknown-unknown 目标构建。");
}
