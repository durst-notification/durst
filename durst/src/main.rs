mod interface;

use dbus::arg;
use dbus::blocking::LocalConnection;
use dbus::tree;
use std::time::Duration;

#[derive(Debug)]
struct Notifications {}

type Err = tree::MethodErr;

impl interface::OrgFreedesktopNotifications for Notifications {
    fn get_capabilities(&self) -> Result<Vec<String>, Err> {
        println!("get_capabilities");
        Ok(vec!["asdf".to_string()])
    }
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
        expire_timeout: i32,
    ) -> Result<u32, Err> {
        println!("app_name {:?}", app_name);
        println!("replaces_id {:?}", replaces_id);
        println!("app_icon {:?}", app_icon);
        println!("summary {:?}", summary);
        println!("body {:?}", body);
        println!("actions {:?}", actions);
        println!("hints {:?}", hints);
        println!("expire_timeout {:?}", expire_timeout);
        Ok(42)
    }
    fn close_notification(&self, id: u32) -> Result<(), Err> {
        println!("id {:?}", id);
        Ok(())
    }
    fn get_server_information(&self) -> Result<(String, String, String, String), Err> {
        println!("running getserverinformation");
        Ok((
            "durst".to_string(),
            "durst-notification.org".to_string(),
            "0.0.1".to_string(),
            "1.2".to_string(),
        ))
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let f = tree::Factory::new_fn::<()>();
    let iface = interface::org_freedesktop_notifications_server(&f, (), |_m| &Notifications {});

    let mut c = LocalConnection::new_session()?;

    c.request_name("org.freedesktop.Notifications", false, true, true)?;

    let tree = factory
        .tree(())
        // needed for introspectable of children
        .add(factory.object_path("/", ()).introspectable())
        .add(
            factory
                .object_path("/org/freedesktop/Notifications", ())
                .introspectable()
                .add(iface),
        );
    tree.start_receive(&c);

    loop {
        c.process(Duration::from_millis(1000))?;
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
