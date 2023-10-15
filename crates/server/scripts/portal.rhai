const GAME_SAVE_DIR = GAME_DIR + "portal/save";
const GAME_CONFIG_DIR = GAME_DIR + "portal/cfg";

fn post_install() {}
fn pre_run() {
    // kind of hacky, but it works
    // we keep both the game's save and config in the `save_dir`
    copy_dir(SAVE_DIR, global::GAME_SAVE_DIR);
    copy_dir(SAVE_DIR, global::GAME_CONFIG_DIR);
}
fn post_run() {
    copy_dir(global::GAME_SAVE_DIR, SAVE_DIR);
    copy_dir(global::GAME_CONFIG_DIR, SAVE_DIR);
}
