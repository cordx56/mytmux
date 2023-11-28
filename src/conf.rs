mod bind;
mod consts;
mod style;
mod utils;

use tmux_interface::Tmux;

pub fn conf_tmux() -> Tmux<'static> {
    let mut tmux = Tmux::new();
    for c in bind::bindkeys_command().into_cmds() {
        tmux = tmux.add_command(c);
    }
    for c in style::options_command().into_cmds() {
        tmux = tmux.add_command(c);
    }
    tmux
}
