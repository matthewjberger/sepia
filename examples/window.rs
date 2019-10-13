use sepia::app::*;

fn main() {
    let mut state: EmptyState = EmptyState;
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
