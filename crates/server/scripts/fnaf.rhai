const GAME_SAVE_FILE = CONFIG_DIR + "MMFApplications/freddy";

fn post_install() {}
fn pre_run() {
    try {
        copy_file(SAVE_DIR + "freddy", global::GAME_SAVE_FILE);
    } catch {
        // they may not've saved before
    }
}
fn post_run() {
    try {
        copy_file(global::GAME_SAVE_FILE, SAVE_DIR + "freddy");
    } catch {
        // they may not have played?
    }
}
