#![allow(unused_must_use, dead_code, unused_variables)]

#[derive(PartialEq, Clone)]
struct Dog {
    name: String,
    owner: String,
    children: Vec<Dog>,
    close_friends: Vec<Dog>,
    age: u64,
}

impl Dog {
    pub fn get_close_friends_of_owner(close_friends: &mut Vec<Dog>, owner: String) -> Vec<&mut Dog> {
        close_friends
            .iter_mut()
            .filter(|child| child.owner == owner)
            .collect()
    }

    pub fn get_children_of_owner(children: &mut Vec<Dog>, owner: String) -> Vec<&mut Dog> {
        children
            .iter_mut()
            .filter(|child| child.owner == owner)
            .collect()
    }

    pub fn hoge_close_friend(&mut self, age: u64, dog: Dog, owner1: String, owner2: String) {
        let friends = Self::get_close_friends_of_owner(&mut self.close_friends, owner1.clone());
        let children = Self::get_children_of_owner(&mut self.children, owner2);
        if let Some(child1) = children.into_iter().next() {
            if let Some(friend) = friends.into_iter().next() {
                child1.owner = owner1.clone();
                friend.owner = owner1.clone();
            }
        }
    }
}

fn main() {}
