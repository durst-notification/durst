mod interface;
mod notification;
mod test;

use dbus::arg;
use dbus::blocking::LocalConnection;
use dbus::tree;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::time::Duration;

use notification::Notification;

#[derive(Debug)]
struct Notifications {
    pub queue: Vec<Notification>,
}

type Err = tree::MethodErr;

impl interface::OrgFreedesktopNotifications for Notifications {
    fn get_capabilities(&self) -> Result<Vec<String>, Err> {
        debug!("get_capabilities");
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
        let new_notification = Notification::new(
            app_name,
            replaces_id,
            app_icon,
            summary,
            body,
            actions,
            hints,
            expire_timeout,
        );
        debug!("notify {:?}", new_notification);
        println!("{:?}", self.queue);
        Ok(42)
    }
    fn close_notification(&self, id: u32) -> Result<(), Err> {
        debug!("close_notification {:?}", id);
        Ok(())
    }
    fn get_server_information(&self) -> Result<(String, String, String, String), Err> {
        debug!("getserverinformation");
        Ok((
            env!("CARGO_PKG_NAME").to_string(),
            "durst-notification.org".to_string(),
            env!("CARGO_PKG_VERSION").to_string(),
            "1.2".to_string(),
        ))
    }
}

impl AsRef<dyn interface::OrgFreedesktopNotifications + 'static> for Rc<Notifications> {
    fn as_ref(&self) -> &(dyn interface::OrgFreedesktopNotifications + 'static) {
        &**self
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let notifications_rc = Rc::new(Notifications {
        queue: Vec::<Notification>::new(),
    });

    let factory = tree::Factory::new_fn::<()>();
    let iface = interface::org_freedesktop_notifications_server(&factory, (), move |_| {
        notifications_rc.clone()
    });

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
