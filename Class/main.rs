struct RsWindow {
    title: String, 
    icon: String,
    width: f32,
    height: f32
}

impl RsWindow
{
    fn new(title: String, width: f32,
        height: f32) -> RsWindow
    {
        println!("New window created...");
        RsWindow {title, icon:"Null.ico".to_string(), width, height}
    }

    fn set_icon(&mut self, new_icon: String)
    {
        self.icon = new_icon;
        println!("New Icon=\"{}\"", self.icon);
    }

    fn print_variables(&self)
    {
        println!("Title=\"{}\" Icon=\"{}\" Width=\"{}\" Height=\"{}\"",
             self.title, self.icon, self.width, self.height);
    }
}

fn main()
{
    let _window_title = String::from("Rust Application...");
    let _window_icon = String::from("Rust.ico");

    let mut window = RsWindow::new(_window_title, 1280.0, 720.0);
    window.set_icon(_window_icon);
    window.print_variables();
}