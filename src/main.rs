mod models;
mod solver;

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
    ];

    let capacity = 45;
    let (max_val, combination) = solver::solve_knapsack(&items, capacity);

    println!("最大価値: {}", max_val);
    println!("品目の組み合わせ: {:?}", combination);
}
