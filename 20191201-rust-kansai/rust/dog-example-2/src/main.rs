#[derive(PartialEq, Clone)]
struct Dog {
    name: String,
    owner: String,
    children: Vec<Dog>,
    close_friends: Vec<Dog>,
    age: u64,
}

impl Dog {
    pub fn hoge_close_friend(&mut self, age: u64, dog: Dog, owner1: String, owner2: String) {
        let mut children = self
            .children
            .iter_mut()
            .filter(|child| child.owner == owner1);
        let mut friends = self
            .close_friends
            .iter_mut()
            .filter(|friend| friend.owner == owner2);
        if let child1 = children.next() {
            if let friend = friends.next() {
                //
            }
        }
    }
}

fn main() {
    let mut dog = Dog {
        name: "kevin".to_string(),
        owner: "Owner1".to_string(),
        age: 3,
        children: Vec::new(),
        close_friends: Vec::new(),
    };
    let mut child_dog = Dog {
        name: "jack".to_string(),
        owner: "Owner2".to_string(),
        age: 3,
        children: Vec::new(),
        close_friends: Vec::new(),
    };
}
