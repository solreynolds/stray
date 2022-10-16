//! # DBus interface proxy for: `com.canonical.dbusmenu`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `org.cannonical.indicator.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use std::collections::HashMap;

use zbus::dbus_proxy;
use zbus::zvariant::OwnedValue;

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct MenuLayout {
    pub id: u32,
    pub fields: SubMenuLayout,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct SubMenuLayout {
    pub id: i32,
    pub fields: HashMap<String, OwnedValue>,
    pub submenus: Vec<OwnedValue>,
}

#[allow(dead_code)]
type GroupProperties = Vec<(i32, HashMap<String, zbus::zvariant::OwnedValue>)>;

#[dbus_proxy(interface = "com.canonical.dbusmenu")]
trait DBusMenu {
    fn about_to_show(&self, id: i32) -> zbus::Result<bool>;

    fn event(
        &self,
        id: i32,
        event_id: &str,
        data: &zbus::zvariant::Value<'_>,
        timestamp: u32,
    ) -> zbus::Result<()>;

    fn get_group_properties(
        &self,
        ids: &[i32],
        property_names: &[&str],
    ) -> zbus::Result<(u32, GroupProperties)>;

    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: &[&str],
    ) -> zbus::Result<MenuLayout>;

    fn get_property(&self, id: i32, name: &str) -> zbus::Result<zbus::zvariant::OwnedValue>;

    #[dbus_proxy(signal)]
    fn item_activation_requested(&self, id: i32, timestamp: u32) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn items_properties_updated(
        &self,
        updated_props: Vec<(
            i32,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        )>,
        removed_props: Vec<(i32, Vec<&str>)>,
    ) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn layout_updated(&self, revision: u32, parent: i32) -> zbus::Result<()>;

    #[dbus_proxy(property)]
    fn status(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<u32>;
}
