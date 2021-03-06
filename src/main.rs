use i3ipc::{reply::NodeLayout, I3Connection, I3EventListener, Subscription, event::{Event, WindowEventInfo}, event::inner::WindowChange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = I3Connection::connect()?;
    let mut listener = I3EventListener::connect()?;
    let subs = [Subscription::Window, Subscription::Binding];
    listener.subscribe(&subs)?;
    for event in listener.listen() {
        let relevant_event = |i : &WindowEventInfo| 
            i.container.layout == NodeLayout::SplitH 
            || i.container.layout == NodeLayout::SplitV
            && i.change == WindowChange::Focus;

        let (_, _, w, h) = match event? {
            Event::WindowEvent(info) if relevant_event(&info) => {
                info.container.window_rect
            },
            Event::BindingEvent(binding) if binding.binding.command.starts_with("resize") => {
                let mut node = &connection.get_tree()?;
                while !node.focused {
                    node = node.nodes.iter().find(|n| node.focus.first().map(|&id| id == n.id).unwrap_or(false)).expect("no focused node");
                }
                node.window_rect
            },
            _ => continue,
        };

        if w > h {
            connection.run_command("split h")?;
        } else {
            connection.run_command("split v")?;
        }
    }
    Ok(())
}
