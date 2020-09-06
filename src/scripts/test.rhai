fn start() {
    t_update_x(5.0);

    return true
}

fn update() {
    t_update_x(t_get_x() + 0.2 * delta);

    return true
}