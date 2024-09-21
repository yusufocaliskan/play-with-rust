use std::io;

#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    health: i32,
    inventory: Vec<Item>,
}

#[derive(Debug, PartialEq)]
enum Item {
    HealthPotion,
    Sword,
}

#[derive(Debug, PartialEq)]
enum Room {
    Entrance,
    Hallway,
    TreasureRoom,
    MonsterRoom,
}

#[derive(Debug)]
struct Monster {
    name: String,
    damage: i32,
}

fn create_player(name: &str) -> Player {
    Player {
        name: name.to_string(),
        health: 100,
        inventory: Vec::new(),
    }
}

fn get_player_choice(options: i32) -> i32 {
    let mut choice = String::new();
    loop {
        println!("Please enter your choice:");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>() {
            Ok(num) if num > 0 && num <= options => return num,
            _ => println!("Invalid choice, try again."),
        }
        choice.clear();
    }
}

fn monster_room(player: &mut Player) {
    let monster = Monster {
        name: String::from("Goblin"),
        damage: 20,
    };

    println!("A wild {} appears!", monster.name);
    player.health -= monster.damage;

    if player.health > 0 {
        player.inventory.push(Item::Sword);
        println!("You defeated the {} and found a sword!", monster.name);
    } else {
        println!("You were defeated by the {}. Game over.", monster.name);
    }
}

fn main() {
    let mut player = create_player("Hero");
    let mut current_room = Room::Entrance;

    loop {
        println!("\nCurrent Room: {:?}", current_room);
        match current_room {
            Room::Entrance => {
                println!("You are at the entrance of a dark cave.");
                println!("1. Move to the hallway");
                println!("2. Leave the cave");
                match get_player_choice(2) {
                    1 => current_room = Room::Hallway,
                    2 => {
                        println!("You chose to leave. Game over.");
                        break;
                    }
                    _ => continue,
                }
            }
            Room::Hallway => {
                println!("You are in a long hallway.");
                println!("1. Go to the treasure room");
                println!("2. Enter the monster room");
                match get_player_choice(2) {
                    1 => current_room = Room::TreasureRoom,
                    2 => current_room = Room::MonsterRoom,
                    _ => continue,
                }
            }
            Room::TreasureRoom => {
                println!("You found a room full of treasure! You win!");
                break;
            }
            Room::MonsterRoom => {
                monster_room(&mut player);
                if player.health <= 0 {
                    break;
                }
                current_room = Room::Hallway;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_player() {
        let player = create_player("Alice");
        assert_eq!(player.name, "Alice");
        assert_eq!(player.health, 100);
        assert_eq!(player.inventory.len(), 0);
    }

    #[test]
    fn test_room_transitions() {
        let mut current_room = Room::Entrance;

        current_room = Room::Hallway;
        assert_eq!(current_room, Room::Hallway);

        current_room = Room::TreasureRoom;
        assert_eq!(current_room, Room::TreasureRoom);
    }

    #[test]
    fn test_monster_battle() {
        let mut player = create_player("Bob");

        assert_eq!(player.health, 100);
        assert_eq!(player.inventory.len(), 0);

        monster_room(&mut player);

        assert_eq!(player.health, 80);
        assert_eq!(player.inventory, vec![Item::Sword]);
    }

    #[test]
    fn test_player_defeat() {
        let mut player = Player {
            name: String::from("Charlie"),
            health: 10,
            inventory: Vec::new(),
        };

        monster_room(&mut player);

        assert!(player.health <= 0);
        assert!(player.inventory.is_empty());
    }
}
