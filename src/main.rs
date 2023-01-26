
use passgen::prelude::*;

fn main() {
    // welcome();
    let config = init();
    let app = ui::build(config);

    // app.run();

    // gen(config);

}
