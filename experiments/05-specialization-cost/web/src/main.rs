fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<specialization_cost_web::app::App>::new().render();
}
