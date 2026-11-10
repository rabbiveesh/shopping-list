//! Items in the change log: their fields, and the items and conflicts the log adds up to.
//!
//! The app comes with starter items. They need no changes to exist: the log only records
//! what the user does to them (picking, ticking off, renaming), and items the user adds.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::changes::{self, Change, Edit, Record};
use crate::items::{after_shop, picked, Item, StarterItem, Status};

/// One field of an item, set to a new value.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "field", content = "value", rename_all = "camelCase")]
pub enum ItemEdit {
    Name(String),
    Aisle(String),
    Status(Status),
}

#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum ItemField {
    Name,
    Aisle,
    Status,
}

/// A change to an item, as the app and storage see it.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ItemChange(pub Change<ItemEdit>);

/// An edit to an item: one field, or deleting it.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ItemOp(pub Edit<ItemEdit>);

/// The edits to make to one item.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemEdits {
    pub item_id: String,
    pub edits: Vec<ItemOp>,
}

/// What the log says about an item; a starter item fills in what it doesn't.
#[derive(Default, Debug, PartialEq)]
pub struct ItemDraft {
    name: Option<String>,
    aisle: Option<String>,
    status: Option<Status>,
}

impl Record for ItemDraft {
    type Edit = ItemEdit;
    type Field = ItemField;

    fn field(edit: &ItemEdit) -> ItemField {
        match edit {
            ItemEdit::Name(_) => ItemField::Name,
            ItemEdit::Aisle(_) => ItemField::Aisle,
            ItemEdit::Status(_) => ItemField::Status,
        }
    }

    fn apply(&mut self, edit: &ItemEdit) {
        match edit {
            ItemEdit::Name(name) => self.name = Some(name.clone()),
            ItemEdit::Aisle(aisle) => self.aisle = Some(aisle.clone()),
            ItemEdit::Status(status) => self.status = Some(*status),
        }
    }
}

impl ItemDraft {
    /// The item, once it has a name (its own or its starter item's).
    fn finish(self, id: &str, starter: Option<&StarterItem>) -> Option<Item> {
        Some(Item {
            id: id.to_string(),
            name: self.name.or_else(|| starter.map(|s| s.name.clone()))?,
            aisle: self
                .aisle
                .or_else(|| starter.map(|s| s.aisle.clone()))
                .unwrap_or_default(),
            status: self.status.unwrap_or_default(),
        })
    }
}

/// Something the user has to settle because two devices disagreed.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Conflict {
    /// The same field was changed on two devices; `options` are the competing changes,
    /// newest first. The item shows the first until the user picks.
    #[serde(rename_all = "camelCase")]
    Field {
        item_id: String,
        options: Vec<ItemChange>,
    },
    /// One device deleted the item while another was still changing (or restoring) it.
    /// The item stays until the user decides.
    #[serde(rename_all = "camelCase")]
    DeletedWhileEdited { item_id: String },
}

/// Every item as the change log says it is now, and the conflicts waiting for the user.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ledger {
    /// Items the user added first, oldest first, then the starter items in their order.
    pub items: Vec<Item>,
    pub conflicts: Vec<Conflict>,
}

/// Replays the log over the starter items.
pub fn ledger(starter: &[StarterItem], changes: &[Change<ItemEdit>]) -> Ledger {
    let mut merged: BTreeMap<String, changes::Merged<ItemDraft>> = changes::merge(changes)
        .into_iter()
        .map(|m| (m.id.clone(), m))
        .collect();
    let mut first_at: BTreeMap<&str, f64> = BTreeMap::new();
    for change in changes {
        let at = first_at.entry(&change.record_id).or_insert(change.at);
        *at = at.min(change.at);
    }
    let mut added: Vec<&str> = first_at
        .keys()
        .copied()
        .filter(|id| !starter.iter().any(|s| s.id == *id))
        .collect();
    added.sort_by(|a, b| first_at[a].total_cmp(&first_at[b]).then_with(|| a.cmp(b)));

    let mut ledger = Ledger {
        items: Vec::new(),
        conflicts: Vec::new(),
    };
    let order = added
        .into_iter()
        .map(|id| (id.to_string(), None))
        .chain(starter.iter().map(|s| (s.id.clone(), Some(s))));
    for (id, starter) in order {
        let Some(merged) = merged.remove(&id) else {
            // A starter item nobody has touched yet.
            ledger
                .items
                .extend(ItemDraft::default().finish(&id, starter));
            continue;
        };
        let mut keep_or_delete = merged.deleted_while_edited;
        for options in merged.clashes {
            if options.iter().any(|c| matches!(c.edit, Edit::Deleted(_))) {
                // Deleted on one device, restored on another: the same keep-or-delete call.
                keep_or_delete = true;
            } else {
                ledger.conflicts.push(Conflict::Field {
                    item_id: id.clone(),
                    options: options.into_iter().map(ItemChange).collect(),
                });
            }
        }
        if keep_or_delete {
            ledger.conflicts.push(Conflict::DeletedWhileEdited {
                item_id: id.clone(),
            });
        }
        if !merged.deleted || keep_or_delete {
            ledger.items.extend(merged.record.finish(&id, starter));
        }
    }
    ledger
}

fn set_status(status: Status) -> ItemOp {
    ItemOp(Edit::Set(ItemEdit::Status(status)))
}

/// A new item, straight onto the shopping list.
pub fn new_item_edits(name: &str, aisle: &str) -> Vec<ItemOp> {
    vec![
        ItemOp(Edit::Set(ItemEdit::Name(name.trim().to_string()))),
        ItemOp(Edit::Set(ItemEdit::Aisle(aisle.to_string()))),
        set_status(Status::Needed),
    ]
}

/// Tapping an item while picking.
pub fn pick_edit(item: &Item) -> ItemOp {
    set_status(picked(item.status))
}

/// Moving an item to another list: ticked off, can't find, back on the list.
pub fn status_edit(status: Status) -> ItemOp {
    set_status(status)
}

/// The edits that finish a shop, one per item whose list changes.
pub fn finish_shop_edits(items: &[Item]) -> Vec<ItemEdits> {
    items
        .iter()
        .filter(|i| after_shop(i.status) != i.status)
        .map(|i| ItemEdits {
            item_id: i.id.clone(),
            edits: vec![set_status(after_shop(i.status))],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::changes::{record, Stamp};

    fn stamp(device: &str, at: f64) -> Stamp {
        Stamp {
            at,
            device: device.to_string(),
        }
    }

    fn starter() -> Vec<StarterItem> {
        ["Milk", "Rice cakes"]
            .iter()
            .map(|name| StarterItem {
                id: name.to_lowercase(),
                name: name.to_string(),
                aisle: "Shop".to_string(),
            })
            .collect()
    }

    fn ops(edits: Vec<ItemOp>) -> Vec<Edit<ItemEdit>> {
        edits.into_iter().map(|op| op.0).collect()
    }

    fn status_of(ledger: &Ledger, id: &str) -> Option<Status> {
        ledger.items.iter().find(|i| i.id == id).map(|i| i.status)
    }

    #[test]
    fn a_fresh_app_shows_every_starter_item_off_the_list() {
        let ledger = ledger(&starter(), &[]);
        let names: Vec<&str> = ledger.items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["Milk", "Rice cakes"]);
        assert!(ledger.items.iter().all(|i| i.status == Status::Off));
    }

    #[test]
    fn picking_a_starter_item_puts_it_on_the_list() {
        let items = ledger(&starter(), &[]).items;
        let log = record(
            &[],
            "rice cakes",
            ops(vec![pick_edit(&items[1])]),
            &stamp("phone", 1.0),
        );
        assert_eq!(
            status_of(&ledger(&starter(), &log), "rice cakes"),
            Some(Status::Needed)
        );
    }

    #[test]
    fn items_the_user_adds_come_first_oldest_first() {
        let mut log = record(
            &[],
            "b",
            ops(new_item_edits(" Hummus ", "My items")),
            &stamp("phone", 2.0),
        );
        log.extend(record(
            &log.clone(),
            "a",
            ops(new_item_edits("Tahini", "My items")),
            &stamp("phone", 3.0),
        ));
        let ledger = ledger(&starter(), &log);
        let names: Vec<&str> = ledger.items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["Hummus", "Tahini", "Milk", "Rice cakes"]);
        assert_eq!(status_of(&ledger, "b"), Some(Status::Needed));
    }

    #[test]
    fn finishing_a_shop_takes_only_the_cart_off_the_list() {
        let mut log = Vec::new();
        for (id, status, at) in [
            ("milk", Status::Got, 1.0),
            ("rice cakes", Status::Missing, 2.0),
        ] {
            let new = record(
                &log,
                id,
                ops(vec![status_edit(status)]),
                &stamp("phone", at),
            );
            log.extend(new);
        }
        let edits = finish_shop_edits(&ledger(&starter(), &log).items);
        assert_eq!(
            edits,
            vec![ItemEdits {
                item_id: "milk".to_string(),
                edits: vec![status_edit(Status::Off)],
            }]
        );
    }

    #[test]
    fn ticked_off_on_one_phone_and_missing_on_another_asks_which_is_right() {
        let log = record(
            &[],
            "milk",
            ops(vec![status_edit(Status::Needed)]),
            &stamp("phone", 1.0),
        );
        let seen = log.clone();
        let mut log = log;
        log.extend(record(
            &seen,
            "milk",
            ops(vec![status_edit(Status::Got)]),
            &stamp("phone", 2.0),
        ));
        log.extend(record(
            &seen,
            "milk",
            ops(vec![status_edit(Status::Missing)]),
            &stamp("tablet", 3.0),
        ));
        let ledger = ledger(&starter(), &log);
        assert_eq!(status_of(&ledger, "milk"), Some(Status::Missing));
        assert!(matches!(
            ledger.conflicts.as_slice(),
            [Conflict::Field { item_id, options }] if item_id == "milk" && options.len() == 2
        ));
    }

    #[test]
    fn a_deleted_item_disappears() {
        let log = record(&[], "milk", vec![Edit::Deleted(true)], &stamp("phone", 1.0));
        assert_eq!(status_of(&ledger(&starter(), &log), "milk"), None);
    }
}
