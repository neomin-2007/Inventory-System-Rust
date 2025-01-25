use std::collections::HashMap;

pub struct Inventory {
    storage: HashMap<String, u8>,
    max_size: u8,
    size: u8
}

pub fn create_inventory(max: u8) -> Inventory {
    let inv = Inventory {
        storage: HashMap::new(),
        max_size: max,
        size: 0,
    };
    return inv;
}

pub fn send_to_inventory(inv: &mut Inventory, item: String, amount: u8) -> bool {
    
    if inv.size >= inv.max_size && !inv.storage.contains_key(&item) {
        println!("[WARN] You cannot pickup this item. Your inventory is full!");
        return false;
    }

    println!("[SUCCESS] You have stored {} from this item in your inventory!", amount);

    if inv.storage.contains_key(&item) {
        let item_amount = inv.storage
        .get(&item)
        .expect("[WARN] Has ocorred an error to get the item amount!");

        inv.storage.insert(item, item_amount + amount);
        return true;
    }

    inv.storage.insert(item, amount);
    return true;
}

pub fn remove_from_inventory(inv: &mut Inventory, item: String, amount: u8) -> bool {

    if !inv.storage.contains_key(&item) {
        println!("[WARN] The item doesn't exist in your inventory!");
        return false;
    }

    let item_amount = inv.storage
    .get(&item)
    .expect("[WARN] Has ocorred an error to get the item amount!");

    if amount >= *item_amount {
        println!("[SUCCESS] The item {}, has been deleted from your inventory!", &item);
        inv.storage.remove(&item);
        return true;
    }

    println!("[SUCCESS] The quantity of {} was removed and {} was left!", &item, amount);
    inv.storage.insert(item, item_amount - amount);

    return true;
}

pub fn get_storage(inv: Inventory) -> HashMap<String, u8> {
    return inv.storage;
}
