const GAME_SAVE_DIR = CONFIG_DIR + "3909/PapersPlease";

fn post_install() {}
fn pre_run() {
    copy_dir(SAVE_DIR, global::GAME_SAVE_DIR);
}
fn post_run() {
    copy_dir(global::GAME_SAVE_DIR, SAVE_DIR);
}
