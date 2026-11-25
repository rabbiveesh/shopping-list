//! Things to buy, and how they move between the lists: the catalogue to pick from, the
//! shopping list, the cart and the fill-in list.

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// Where an item is in the shopping cycle.
#[derive(Tsify, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// Only in the catalogue: not needed right now.
    #[default]
    Off,
    /// On the shopping list.
    Needed,
    /// Ticked off on this shop.
    Got,
    /// Couldn't be found on a shop; waits on the fill-in list.
    Missing,
}

/// Something the user buys, as the change log says it is now.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub name: String,
    /// The group it is shown under, such as "Dairy". Free text.
    pub aisle: String,
    pub status: Status,
}

/// An item the app comes with, before the user has changed it.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StarterItem {
    pub id: String,
    pub name: String,
    pub aisle: String,
}

/// Items shown together under one heading, in the order they first appear.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Aisle {
    pub name: String,
    pub items: Vec<Item>,
}

/// The lists the app shows, all from one set of items.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Lists {
    /// Everything, to pick from before a shop.
    pub catalogue: Vec<Aisle>,
    /// Still to get on this shop.
    pub to_buy: Vec<Aisle>,
    /// Ticked off on this shop.
    pub in_cart: Vec<Item>,
    /// Couldn't be found; for another shop.
    pub fill_in: Vec<Item>,
}

/// The status after tapping an item while picking: on the list, or off it again.
/// Something on the fill-in list goes back on the list.
pub fn picked(status: Status) -> Status {
    match status {
        Status::Off | Status::Missing => Status::Needed,
        Status::Needed | Status::Got => Status::Off,
    }
}

/// Whether an item shows as chosen while picking.
pub fn is_on_list(status: Status) -> bool {
    matches!(status, Status::Needed | Status::Got)
}

/// Finishing a shop takes what's in the cart off the list. What wasn't got stays on it,
/// and the fill-in list is kept for another shop.
pub fn after_shop(status: Status) -> Status {
    match status {
        Status::Got => Status::Off,
        other => other,
    }
}

/// Groups items by aisle, keeping the order items come in.
pub fn aisles(items: &[Item]) -> Vec<Aisle> {
    let mut aisles: Vec<Aisle> = Vec::new();
    for item in items {
        match aisles.iter_mut().find(|a| a.name == item.aisle) {
            Some(aisle) => aisle.items.push(item.clone()),
            None => aisles.push(Aisle {
                name: item.aisle.clone(),
                items: vec![item.clone()],
            }),
        }
    }
    aisles
}

/// The catalogue, shopping list, cart and fill-in list.
pub fn lists(items: &[Item]) -> Lists {
    let with = |status: Status| -> Vec<Item> {
        items
            .iter()
            .filter(|i| i.status == status)
            .cloned()
            .collect()
    };
    Lists {
        catalogue: aisles(items),
        to_buy: aisles(&with(Status::Needed)),
        in_cart: with(Status::Got),
        fill_in: with(Status::Missing),
    }
}

/// The item already called `name`, ignoring case and spaces around it, so adding
/// something twice picks the one there instead of making a copy.
pub fn item_named<'a>(items: &'a [Item], name: &str) -> Option<&'a Item> {
    let wanted = name.trim().to_lowercase();
    items
        .iter()
        .find(|i| i.name.trim().to_lowercase() == wanted)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, aisle: &str, status: Status) -> Item {
        Item {
            id: id.to_string(),
            name: id.to_string(),
            aisle: aisle.to_string(),
            status,
        }
    }

    #[test]
    fn tapping_a_food_puts_it_on_the_list_and_tapping_again_takes_it_off() {
        assert_eq!(picked(Status::Off), Status::Needed);
        assert_eq!(picked(Status::Needed), Status::Off);
    }

    #[test]
    fn picking_something_from_the_fill_in_list_puts_it_back_on_the_list() {
        assert_eq!(picked(Status::Missing), Status::Needed);
    }

    #[test]
    fn finishing_a_shop_clears_the_cart_but_keeps_what_was_not_got() {
        assert_eq!(after_shop(Status::Got), Status::Off);
        assert_eq!(after_shop(Status::Needed), Status::Needed);
        assert_eq!(after_shop(Status::Missing), Status::Missing);
    }

    #[test]
    fn each_list_shows_only_its_own_items_grouped_by_aisle() {
        let items = [
            item("Milk", "Dairy", Status::Needed),
            item("Apples", "Fruit", Status::Got),
            item("Eggs", "Dairy", Status::Missing),
            item("Butter", "Dairy", Status::Off),
            item("Pears", "Fruit", Status::Needed),
        ];
        let lists = lists(&items);
        let names = |aisles: &[Aisle]| -> Vec<(String, Vec<String>)> {
            aisles
                .iter()
                .map(|a| {
                    (
                        a.name.clone(),
                        a.items.iter().map(|i| i.name.clone()).collect(),
                    )
                })
                .collect()
        };
        assert_eq!(
            names(&lists.to_buy),
            vec![
                ("Dairy".to_string(), vec!["Milk".to_string()]),
                ("Fruit".to_string(), vec!["Pears".to_string()]),
            ]
        );
        assert_eq!(lists.catalogue.len(), 2);
        assert_eq!(lists.catalogue[0].items.len(), 3);
        assert_eq!(lists.in_cart, vec![items[1].clone()]);
        assert_eq!(lists.fill_in, vec![items[2].clone()]);
    }

    #[test]
    fn adding_something_already_there_finds_it_whatever_the_case() {
        let items = [item("Rice cakes", "Bread", Status::Off)];
        assert_eq!(
            item_named(&items, "  rice CAKES ").map(|i| i.id.as_str()),
            Some("Rice cakes")
        );
        assert_eq!(item_named(&items, "Rice"), None);
    }
}
