#[macro_use]
extern crate clap;

mod cli;
mod config;
mod interface;
mod notification;
mod test;

use dbus::arg;
use dbus::blocking::stdintf::org_freedesktop_dbus::RequestNameReply;
use dbus::blocking::LocalConnection;
use dbus_tree as tree;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::env::var;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Duration;

use config::Rule;
use notification::Notification;

#[derive(Debug)]
struct Container {
    queue: Vec<Notification>,
    config: Vec<Rule>,
}

type Err = tree::MethodErr;

impl interface::OrgFreedesktopNotifications for Mutex<Container> {
    fn get_capabilities(&self) -> Result<Vec<String>, Err> {
        debug!("get_capabilities");
        Ok(vec!["test".to_string()])
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
        let mut data = self.lock().unwrap();
        (*data).queue.push(new_notification);
        Ok((*data).queue.len() as u32)
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

impl AsRef<dyn interface::OrgFreedesktopNotifications + 'static> for Rc<Mutex<Container>> {
    fn as_ref(&self) -> &(dyn interface::OrgFreedesktopNotifications + 'static) {
        &**self
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_home = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();
    let tmp = config::load_config(format!("{}/durst/config.yml", config_home));

    let container_rc = Rc::new(Mutex::new(Container {
        queue: Vec::<Notification>::new(),
        config: tmp,
    }));

    let factory = tree::Factory::new_fn::<()>();
    let iface = interface::org_freedesktop_notifications_server(&factory, (), move |_| {
        Rc::clone(&container_rc)
    });

    let c = LocalConnection::new_session()?;

    let r = c.request_name("org.freedesktop.Notifications", false, true, true)?;
    if r != RequestNameReply::PrimaryOwner {
        panic!("Another notification daemon is running!");
    }

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
    env_logger::init();

    let parsed_cli = cli::build_cli().get_matches();
    if let Some(mode) = parsed_cli.value_of("mode") {
        match mode {
            "wayland" => println!("You are using wayland"),
            "stdout" => println!("You are using stdout"),
            _ => unreachable!(),
        }
    }
    if let Err(e) = run() {
        println!("{}", e);
    }
}
