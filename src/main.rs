use i3ipc::{reply::NodeLayout, I3Connection, I3EventListener, Subscription, event::Event, event::inner::WindowChange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = I3Connection::connect()?;
    let mut listener = I3EventListener::connect()?;
    let subs = [Subscription::Window];
    listener.subscribe(&subs)?;
    for event in listener.listen() {
        if let Event::WindowEvent(info) = event?  {
            match info.container.layout {
                NodeLayout::SplitH | NodeLayout::SplitV if info.change == WindowChange::New => {
                    let (_, _, w, h) = info.container.window_rect;
                    if w > h {
                        connection.run_command("split h")?;
                    } else {
                        connection.run_command("split v")?;
                    }
                },
                _ => (),
            }
        }
    }
    Ok(())
}
