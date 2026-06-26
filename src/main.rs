mod models;
mod solver;
use std::time::Instant;

fn main() {
    let items = vec![
        models::Item { id: 1, weight: 4, value: 6 },
        models::Item { id: 2, weight: 8, value: 12 },
        models::Item { id: 3, weight: 3, value: 4 },
        models::Item { id: 4, weight: 5, value: 3 },
        models::Item { id: 5, weight: 9, value: 7 },
        models::Item { id: 6, weight: 2, value: 1 },
        models::Item { id: 7, weight: 3, value: 3 },
        models::Item { id: 8, weight: 1, value: 2 },
        models::Item { id: 9, weight: 5, value: 7 },
        models::Item { id: 10, weight: 2, value: 3 },
        models::Item { id: 11, weight: 4, value: 4 },
        models::Item { id: 12, weight: 2, value: 2 },
        models::Item { id: 13, weight: 7, value: 10 },
        models::Item { id: 14, weight: 10, value: 13 },
        models::Item { id: 15, weight: 3, value: 5 },
        models::Item { id: 16, weight: 13, value: 16 },
        models::Item { id: 17, weight: 11, value: 14 },
        models::Item { id: 18, weight: 8, value: 9 },
        models::Item { id: 19, weight: 6, value: 8 },
        models::Item { id: 20, weight: 4, value: 5 },
        models::Item { id: 21, weight: 2, value: 4 },
        models::Item { id: 22, weight: 7, value: 9 },
        models::Item { id: 23, weight: 3, value: 6 },
        models::Item { id: 24, weight: 5, value: 8 },
        models::Item { id: 25, weight: 9, value: 10 },
    ];

    let capacity = 45;

    // 総当たり法の計測
    print!("総当たり法の場合\n");
    let start = Instant::now();
    let (val1, _) = solver::solve_knapsack(&items, capacity);
    println!("総当たり法: {:?}, 時間: {:?}\n", val1, start.elapsed());

    // DPの計測
    print!("動的計画法の場合\n");
    let start = Instant::now();
    let (val2, _) = solver::solve_knapsack_dp(&items, capacity as usize);
    println!("動的計画法: {:?}, 時間: {:?}\n", val2, start.elapsed());
}
