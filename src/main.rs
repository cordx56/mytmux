mod conf;

fn main() {
    let _ = conf::conf_tmux().spawn();
}
