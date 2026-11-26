fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<recursive_optimizer_web::app::App>::new().render();
}
