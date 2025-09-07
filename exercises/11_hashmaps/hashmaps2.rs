use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    // 只添加缺失的新水果
    if !basket.contains_key(&Fruit::Banana) {
        basket.insert(Fruit::Banana, 1);
    }
    if !basket.contains_key(&Fruit::Pineapple) {
        basket.insert(Fruit::Pineapple, 1);
    }
    
    // 计算当前总数
    let total: u32 = basket.values().sum();
    
    // 如果总数不超过11，需要增加新水果的数量
    if total <= 11 {
        let needed = 12 - total; // 至少需要达到12个
        
        // 选择其中一种新水果来增加数量
        if basket.contains_key(&Fruit::Banana) {
            if let Some(count) = basket.get_mut(&Fruit::Banana) {
                *count += needed;
            }
        } else if basket.contains_key(&Fruit::Pineapple) {
            if let Some(count) = basket.get_mut(&Fruit::Pineapple) {
                *count += needed;
            }
        }
    }
}

fn main() {
    // 测试代码
    let mut basket = get_fruit_basket();
    println!("初始篮子: {:?}", basket);
    fruit_basket(&mut basket);
    println!("处理后篮子: {:?}", basket);
    println!("总数: {}", basket.values().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content.into_iter())
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {:?} was not found in basket", fruit_kind);
            };
            assert!(*amount > 0);
        }
    }
}