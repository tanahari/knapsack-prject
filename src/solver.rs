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

pub fn solve_knapsack_dp(items: &[Item], capacity: usize) -> (u32, Vec<usize>) {
    let n = items.len();
    // dp[i][w] = i番目までの品物を使い、容量wに収まる最大価値
    // メモリ節約のため Vec を使用
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 0..n {
        let item = &items[i];
        for w in 0..=capacity {
            if item.weight as usize <= w {
                // 入れる場合と入れない場合の大きい方を選ぶ
                dp[i + 1][w] = dp[i][w].max(dp[i][w - item.weight as usize] + item.value);
            } else {
                // 入らない場合はそのまま
                dp[i + 1][w] = dp[i][w];
            }
        }
    }

    // どの品物を選んだか復元する処理
    let mut best_combination = Vec::new();
    let mut w = capacity;
    for i in (0..n).rev() {
        if dp[i + 1][w] != dp[i][w] {
            best_combination.push(items[i].id);
            w -= items[i].weight as usize;
        }
    }

    (dp[n][capacity], best_combination)
}