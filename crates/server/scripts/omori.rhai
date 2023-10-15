const GAME_SAVE_DIR = GAME_DIR + "www/save";

fn post_install() {
    try {
        remove_dir_all(global::GAME_SAVE_DIR); // I left some save files in there... whoops.
    } catch {
        // idk what would happen here
        // but I don't need to find out
    }
}
fn pre_run() {
    copy_dir(SAVE_DIR, global::GAME_SAVE_DIR);
}
fn post_run() {
    copy_dir(global::GAME_SAVE_DIR, SAVE_DIR);
}
