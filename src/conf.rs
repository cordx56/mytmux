mod bind;
mod consts;
mod opts;
pub mod status;
mod utils;

use tmux_interface::Tmux;

pub fn conf_tmux() -> Tmux<'static> {
    let mut tmux = Tmux::new();
    for c in opts::set_opts().into_cmds() {
        tmux = tmux.add_command(c);
    }
    for c in bind::bindkeys_command().into_cmds() {
        tmux = tmux.add_command(c);
    }
    for c in status::set_status().into_cmds() {
        tmux = tmux.add_command(c);
    }
    tmux
}
