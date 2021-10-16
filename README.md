# i3-alternating
~~~~rust
     1	use i3ipc::{reply::NodeLayout, I3Connection, I3EventListener, Subscription, event::{Event, WindowEventInfo}, event::inner::WindowChange};
       
     2	fn main() -> Result<(), Box<dyn std::error::Error>> {
     3	    let mut connection = I3Connection::connect()?;
     4	    let mut listener = I3EventListener::connect()?;
     5	    let subs = [Subscription::Window, Subscription::Binding];
     6	    listener.subscribe(&subs)?;
     7	    for event in listener.listen() {
     8	        let relevant_event = |i : &WindowEventInfo| 
     9	            i.container.layout == NodeLayout::SplitH 
    10	            || i.container.layout == NodeLayout::SplitV
    11	            && i.change == WindowChange::Focus;
       
    12	        let (_, _, w, h) = match event? {
    13	            Event::WindowEvent(info) if relevant_event(&info) => {
    14	                info.container.window_rect
    15	            },
    16	            Event::BindingEvent(binding) if binding.binding.command.starts_with("resize") => {
    17	                let mut node = &connection.get_tree()?;
    18	                while !node.focused {
    19	                    node = node.nodes.iter().find(|n| node.focus.first().map(|&id| id == n.id).unwrap_or(false)).expect("no focused node");
    20	                }
    21	                node.window_rect
    22	            },
    23	            _ => continue,
    24	        };
       
    25	        if w > h {
    26	            connection.run_command("split h")?;
    27	        } else {
    28	            connection.run_command("split v")?;
    29	        }
    30	    }
    31	    Ok(())
    32	}

~~~~

That's the whole thing. I just put it in the i3 config as:
`exec_always --no-startup-id /home/username/bin/i3-alternating &`
so if you have problems, you can probably find them in the ~~5(?) lines of non-boilerplate code.~~

Due to feature requests, the code size has grown prodigiously, and contains a very suspicious line (19) that personally I just don't trust at all. If you hit a bug, it's probably on that line.
