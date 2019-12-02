# Pythonista が Rust を使ってハマった点

2019-12-01 Rust Kansai

---

## 自己紹介

大野 大阪の自営業のプログラマ

- Python 歴 5年
- C / C++ 歴 8年
- Ruby歴 2年

Python でのデータ解析と自動化が主な仕事

---

## 勉強会主催してます．

---

## Osaka Python User Group in English

英語で話す Python の勉強会．

PyMCの勉強会で会った外国人と開催

Python Rust Binding について次回発表予定

2019-12-13 (金) 19:00 本町にて． meetup や connpass で公開中

---

## Financial Python
 - クローズドな金融の勉強会
 - 先日三周年を迎えました．

---

@snap[span-30 north]
![Logo](20191201-Rust-Study-Event/assets/img/programming_rust.jpg)
@snapend

@snap[south span-100]
## Rust の学習レベル

**Programming Rust** を読んだ程度
@snapend

---

英語の勉強を兼ねて電子書籍で原著で読み込んでみたけど，復習の難しさからこういうのは日本語，少なくとも紙の本で買うべきだったなぁと少し後悔．

---

@snap[span-30 north]
![Logo](20191201-Rust-Study-Event/assets/img/rust-cycling.jpg)
@snapend

@snap[south span-100]
## 結局自転車本買いました．

まだ読んでません．
@snapend

---

入門書は読んだけど，実際に実装を始めると躓く点だらけ．

---

今日の発表は知識のすり合わせを目的としています．

気になるところや間違ったところがあれば，どんどん発言してください．

---

## 今日は借用の話が多いです．

参加者の中でRust の所有権や借用について全く知らない人ってどのぐらい？

---

借用：変数に対して一つの借用しか作れない．借用中は参照もできないし元の変数の操作もできない．

参照：変数に対していくつでも参照を作成できる．もとの変数の参照も可能だが変更は不可．

---

## 一番躓いているところ

---

### Rust は従来のオブジェクト指向言語と同様の思想では書けない

---

継承がないとかの問題は聞き及んではいたけど，それだけで済む甘い問題ではなかった．

---

## Python のクラスメソッドを Rust で書くには

---

### Python

```python zoom-09
class Dog:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def description(self):
        return "{}, a {} years old dog".format(self.name, self.age)

    def speak_with_description(self, sound):
        return "\"{}\" said {}.".format(sound, self.description())


if __name__ == '__main__':
    dog = Dog("Kevin", 3)
    print(dog.speak_with_description("Bow!"))
```

---
### Rust

```rust zoom-06
struct Dog {
    name: String,
    age: u64,
}

impl Dog {
    pub fn description(&self) -> String {
        format!("{}, a {} years old dog", self.name, self.age)
    }

    pub fn speak_with_description(&self, sound: String){
        println!("\"{}\" said {}.", sound, self.description());
    }
}


fn main() {
    let dog = Dog{ name:"kevin".to_string(), age:3 };
    dog.speak_with_description("Bow!".to_string());
}

```
---

## 参照だけなら問題がない

---

## 借用が入ると話が変わる

---

「owner1 が飼い主の仲良しの友達とowner2 が飼い主の子供に対して何かしらの処理する」という内容をメソッドで実装してみる．

---

field をそのまま使えばエラーが起こらない例

---

```rust code-wrap code-scrollable zoom-09
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
```

@snap[south span-100]
@[1,2,4,5,6,8,zoom-10](2つのVec<Dog>)
@[11-25, zoom-10](filterにマッチした各フィールドに対してループを適応する処理)
@[11-25, zoom-10](特に問題なく動く)
@snapend
---

filter の適応をメソッドにした場合

---

```rust code-wrap code-scrollable zoom-09
#[derive(PartialEq, Clone)]
struct Dog {
    name: String,
    owner: String,
    children: Vec<Dog>,
    close_friends: Vec<Dog>,
    age: u64,
}

impl Dog {
    pub fn get_close_friends_of_owner(&mut self, owner: String) -> Vec<&mut Dog> {
        self.close_friends
            .iter_mut()
            .filter(|friend| friend.owner == owner)
            .collect()
    }

    pub fn get_children_of_owner(&mut self, owner: String) -> Vec<&mut Dog> {
        self.children
            .iter_mut()
            .filter(|child| child.owner == owner)
            .collect()
    }

    pub fn hoge_close_friend(&mut self, age: u64, dog: Dog, owner1: String, owner2: String) {
        let friends = self.get_close_friends_of_owner(owner1.clone());
        let children = self.get_children_of_owner(owner2);
        if let Some(child1) = children.into_iter().next() {
            if let Some(friend) = friends.into_iter().next() {
                child1.owner = owner1.clone();
                friend.owner = owner1.clone();
            }
        }
    }
}
```

@snap[south span-100]
@[11-23, zoom-10](それぞれのフィールドの選択をメソッドにした．)
@[26-27, zoom-10](メソッドを呼び出して要素を同時に取り出した．)
@snapend

---

@snap[span-100]
![Rust Code](20191201-Rust-Study-Event/assets/img/Selection_047.png)
@snapend

### Mutiple borrowing error

構造体のフィールドに対して借用が残った状態で，他のメソッドの呼び出しができない．

---

```rust code-wrap code-scrollable zoom-09
pub fn hoge_close_friend(&mut self, age: u64, dog: Dog, owner1: String, owner2: String) {
    let friends = self.get_close_friends_of_owner(owner1.clone());
    let children = self.get_children_of_owner(owner2);
}
```

ちなみに，これだけだとエラーにならない．friends の変数が children の変数の作成のあとに使われていないから．

---

借用の問題を考えると，Python で書いていたようにクラスのフィールドの内容の選択のためにクラスのメソッドの中で他のメソッドを使うことはできるだけやめたほうが良さそう．

---

じゃあどうすればいいのか？

---

&mut self をできるだけやめる．

構造体のスタティックメソッドを使うことにした．

---

```rust code-wrap code-scrollable zoom-07
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
            .filter(|friend| friend.owner == owner)
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

```

@snap[south span-100]
@[11-28, zoom-10](構造体のスタティックメソッドに直接 field を渡すことにして対応)
@[11-28, zoom-10](こうすることで self 自体に借用が残っている状態を回避する)
@snapend

---

構造体を外部から操作するメソッド（API的な利用）以外の内部のプライベートなメソッドとして使うものはできるだけスタティックメソッドにしてしまったほうが良いかも知れない

---

これがベストプラクティスとは思えない・・・．

みなさんならどうしますか？

[RustLang での同様の例](https://users.rust-lang.org/t/how-to-partially-borrow-from-struct/32221)


---

このように所有権や参照，借用の問題を理解したつもりでも，オブジェクト指向で書かれたものをRustで再実装するには根本的なプログラミングデザインの見直しが必要になる．

---

そうであったとしても，Rustの所有権の適応は設計する上で考慮したほうが良いとは感じる．

---

## 次の問題

---

### 条件分岐と一緒に借用スライスを使いたい

---

ある条件を満たすものを探してみて，それが見つからなければ別の条件で満たすものがあるかを探す．

そしてどちらかの条件で見つかった場合に見つかった対象に特定の処理を行う．

---

```rust code-wrap code-scrollable zoom-08
fn main() {
    let mut v: Vec<i64> = vec![1, 2, 3, 4, 5];
    let mut s: Vec<&mut i64> = v
        .iter_mut()
        .filter(|val| **val < 2_i64)
        .collect();
    if s.len() == 0 {
        s = v
            .iter_mut()
            .filter(|val| **val > 2_i64)
            .collect();
    }
    *s[0] = 0;
    println!("{:?}", v);
}
```

@snap[south span-100]
@[3-6, zoom-10](条件を満たすスライスを作成)
@[7-12, zoom-10](条件を満たすものがなかった場合は別の方法で作成)
@[13, zoom-10](共通の処理)
@snapend

---

@snap[span-100]
![Rust Code](20191201-Rust-Study-Event/assets/img/Selection_048.png)
@snapend

### Mutiple borrowing error

他のスライスへの借用がある状態で新しく v の借用スライスを作成できない．

---

プログラム的には Vec の借用で競合する部分は出ないはず．実装方法は？

---

[Stackoverflow で聞いてみました．](https://stackoverflow.com/questions/58951421/how-to-get-mutable-slice-with-conditions-in-rust?noredirect=1#comment104159318_58951421)

---

### 解答例

```rust code-wrap code-scrollable zoom-08
fn main() {
    let mut v: Vec<i64> = vec![1, 2, 3, 4, 5];
    let s: Vec<_> = v.iter_mut().filter(|val| **val < 2_i64).collect();
    let mut s = if s.len() == 0 {
        v.iter_mut().filter(|val| **val > 2_i64).collect()
    } else {
        s
    };

    *s[0] = 0;
    println!("{:?}", v);
}
```

@snap[south span-100]
@[4-8, zoom-10](let if で変数を再定義させることで借用を複数回発生させないようにしている．)
@[4-8, zoom-10](変数の再定義をされると，そこで借用は開放されるみたい．)
@snapend

---

## Rust の Iterator の種類

---

本を読んでると `iter()` と `iter_mut()` を使えば大体やりたいことができるんだなぁということは理解できる．

---

`iter()` などで参照の collection を作った時に，その要素へのアクセスはどうしていけばいいんだろうと少し悩んだことがあった．

---

そのような場合は `into_iter()` を使う．

---

```rust code-wrap code-scrollable zoom-09
use std::collections::VecDeque;

#[derive(Debug)]
struct TestStruct {
    name: String,
}

fn main() {
    let mut deq = VecDeque::new();
    deq.push_back(TestStruct{name:"name".to_string()});
    let slice : Vec<&TestStruct> = deq.iter().collect();
    let slice2 : Vec<&TestStruct> = slice.into_iter().collect();
    let slice3 : Vec<&&TestStruct> = slice2.iter().collect();
}
```

@snap[south span-100]
@[11, zoom-10](VecDequeからその要素に対する Vec<&TestStruct> を作成)
@[12, zoom-10](slice から新しい Vec<&TestStruct> を作成)
@[13, zoom-10](into_iter ではなく iter を使ってしまうと，Vec<&&TestStruct>になってしまう．)
@snapend

---

## 他のメモ的な内容

---

### Option
Option も Result と同じように unwrap とか expect とかできる

---

### Test で一回だけログの設定を呼ぶ

```rust code-wrap code-scrollable zoom-09
use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
fn setup() {
    INIT.call_once(|| {
        env_logger::init().unwrap();
    });
}
```

---

## GroupBy した結果から複数の結果をまとめて得る．

`Itertools` の `group_by` メソッド

---

```rust code-wrap code-scrollable zoom-09
let orders = friend_list
    .iter()
    .group_by(|friend| &friend.main_group)
    .into_iter()
    .map(|(key, group)| {
        let group = group.collect::<Vec<&FriendList>>();
        (
            key.clone(),
            group.iter().map(|friend| friend.num_of_friends).sum::<i64>(),
            group.iter().map(|friend| friend.num_of_groups).sum::<i64>(),
        )
    })
    .collect::<Vec<(String, i64, i64)>>();
```


@snap[south span-100]
@[3,5,6, zoom-10](一度，Group を Iterator から Vec にしてしまう．)
@[9,10, zoom-10](すると iter を使える．そのままだと group のイテレータを消耗してしまう．)
@snapend

---

## まとめ

---

Rust の所有権と参照・借用，ライフタイムを反映したオブジェクト指向が必要

構造体のメソッドの柔軟性は低い

---

## Rust を書いた上で感じた点

- パッケージマネージャの標準搭載は快適
- テンプレートの標準化などの書きやすさの向上
- 設計を考える上での学びが多い

---

# Thank you!!
