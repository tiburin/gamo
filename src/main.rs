static WRITE_BUILD: bool = true;
static WRITE_RELEASE: bool = true;

fn main() {
    let gamo = gamo::start();

    let connector = gamo::run(&gamo);
    if WRITE_BUILD {
        gamo::write_build(&gamo, false);
    }
    if WRITE_RELEASE {
        gamo::write_release(&connector, &gamo)
    }
}
