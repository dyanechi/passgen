
use passgen::prelude::*;
use gtk::prelude::*;

fn main() {
    // welcome();
    let config = init();
    let app = ui::build(config);
    app.run();

    // app.run();

    // gen(config);

}
 