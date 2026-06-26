use crate::models::Item;

pub fn solve_knapsack(items: &[Item], capacity: u32) -> (u32, Vec<usize>) {
    let n = items.len();
    let mut max_value = 0;
    let mut best_combination = Vec::new();

    // 2^n 通りの組み合わせを全探索
    for i in 0..(1 << n) {
        let mut current_weight = 0;
        let mut cuurent_value = 0;
        let mut current_item = Vec::new();

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                current_weight += items[j].weight;
                cuurent_value += items[j].value;
                current_item.push(items[j].id);
            }
        }

        if current_weight <= capacity && cuurent_value > max_value {
            max_value = cuurent_value;
            best_combination = current_item;
        }
    }

    (max_value, best_combination)
}