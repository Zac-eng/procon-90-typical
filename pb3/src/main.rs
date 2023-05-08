// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         road_list: [[usize;2];n-1],
//     }
//     let farest_info = get_farest(1, &road_list, n);
//     let longest_info = get_farest(farest_info[0] + 1, &road_list, n);
//     let farest_distance: usize = longest_info[1] + 1;
//     println!("{}", farest_distance);
// }

// fn get_farest(start_pos: usize, road_list: &Vec<Vec<usize>>, n: usize) -> [usize;2] {
//     let mut distance_list: Vec<usize> = vec![0;n];
//     let mut road_avail: Vec<usize> = vec![1;n];
//     culc_dist(n, &mut distance_list, &road_list, start_pos, &mut road_avail);
//     get_max_pair(&distance_list)
// }

// fn culc_dist(n: usize, distance_list: &mut Vec<usize>, road_list: &Vec<Vec<usize>>, pos: usize, road_avail: &mut Vec<usize>) {
//     for index in 0..n-1 {
//         if road_avail[index] == 1 {
//             if road_list[index][0] == pos {
//                 distance_list[road_list[index][1] - 1] = distance_list[pos - 1] + 1;
//                 road_avail[index] = 0;
//                 culc_dist(n, distance_list, road_list, road_list[index][1], road_avail);
//             } else if road_list[index][1] == pos {
//                 distance_list[road_list[index][0] - 1] = distance_list[pos - 1] + 1;
//                 road_avail[index] = 0;
//                 culc_dist(n, distance_list, road_list, road_list[index][0], road_avail);
//             }
//         }
//     }
// }

// fn get_max_pair(list: &Vec<usize>) -> [usize;2] {
//     let mut max_pair:[usize;2] = [0;2];
//     let mut now_index: usize = 0;
//     for elem in list {
//         if elem > &max_pair[1] {
//             max_pair[0] = now_index;
//             max_pair[1] = *elem;
//         }
//         now_index += 1;
//     }
//     max_pair
// }

use proconio::input;

fn main() {
    input! {
        n: usize,
        road_list: [(usize, usize); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for (u, v) in road_list {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let farest_info = get_farest(0, &graph, n);
    let longest_info = get_farest(farest_info[0], &graph, n);
    let farest_distance: usize = longest_info[1] + 1;
    println!("{}", farest_distance);
}

fn get_farest(start_pos: usize, graph: &[Vec<usize>], n: usize) -> [usize; 2] {
    let mut distance_list: Vec<usize> = vec![0; n];
    culc_dist(start_pos, &mut distance_list, graph);
    get_max_pair(&distance_list)
}

fn culc_dist(pos: usize, distance_list: &mut Vec<usize>, graph: &[Vec<usize>]) {
    for &next_pos in &graph[pos] {
        if distance_list[next_pos] == 0 {
            distance_list[next_pos] = distance_list[pos] + 1;
            culc_dist(next_pos, distance_list, graph);
        }
    }
}

fn get_max_pair(list: &[usize]) -> [usize; 2] {
    let mut max_pair: [usize; 2] = [0; 2];
    let mut _now_index: usize = 0;
    for (i, elem) in list.iter().enumerate() {
        if elem > &max_pair[1] {
            max_pair[0] = i;
            max_pair[1] = *elem;
        }
        _now_index += 1;
    }
    max_pair
}
