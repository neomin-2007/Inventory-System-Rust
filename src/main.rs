mod inventory;

fn main() {

    let mut inv = inventory::create_inventory(32);
    inventory::send_to_inventory(&mut inv, String::from("Espada de Gelo"), 1);
    inventory::send_to_inventory(&mut inv, String::from("Espada de Fogo"), 1);
    inventory::send_to_inventory(&mut inv, String::from("Amuleto de Tasmân"), 1);
    inventory::send_to_inventory(&mut inv, String::from("Pedra de Fogo"), 3);
    inventory::send_to_inventory(&mut inv, String::from("Pedra de Raio"), 2);
    inventory::send_to_inventory(&mut inv, String::from("Pedra de Porra"), 5);

    inventory::remove_from_inventory(&mut inv, String::from("Pedra de Fogo"), 2);
    inventory::remove_from_inventory(&mut inv, String::from("Cu de Leão Fogo"), 2);
    
    let mut counter: u32 = 0;
    for (k, v) in inventory::get_storage(inv) {
        println!("{} - {}x {}", counter, v, k);
        counter += 1;
    }

}
