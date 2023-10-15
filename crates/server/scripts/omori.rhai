const GAME_SAVE_DIR = GAME_DIR + "www/save";

fn post_install() {}
fn pre_run() {
    copy_dir(SAVE_DIR, global::GAME_SAVE_DIR);
}
fn post_run() {
    copy_dir(global::GAME_SAVE_DIR, SAVE_DIR);
}
