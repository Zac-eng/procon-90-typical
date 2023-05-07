use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w];h],
    }
    let array = Vec::from(a);
    let vertical_sum = get_vertical_sum(&array);
    let horizontal_sum = get_horizontal_sum(h, w, &array);
    let sum_array = gen_sum_array(array, vertical_sum, horizontal_sum, h, w);
    print_double_array(w, sum_array);
}

fn get_vertical_sum(array: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut vertical_sum: Vec<usize> = Vec::new();
    for row in array {
        vertical_sum.push(row.iter().sum());
    }
    vertical_sum
}

fn get_horizontal_sum(h: usize, w: usize, array: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut horizontal_sum: Vec<usize> = Vec::new();
    for column in 0..w {
        let mut row_sum: usize = 0;
        for row in 0..h {
            row_sum += array[row][column];
        }
        horizontal_sum.push(row_sum);
    }
    horizontal_sum
}

fn gen_sum_array(array: Vec<Vec<usize>>, vertical_sum: Vec<usize>, horizontal_sum: Vec<usize>, h: usize, w: usize) -> Vec<Vec<usize>> {
    let mut sum_array: Vec<Vec<usize>> = Vec::new();
    for row in 0..h {
        let mut sum_row: Vec<usize> = Vec::new();
        for column in 0..w {
            let component_sun: usize = vertical_sum[row] + horizontal_sum[column] - array[row][column];
            sum_row.push(component_sun);
        }
        sum_array.push(sum_row);
    }
    sum_array
}

fn print_double_array(w: usize, array: Vec<Vec<usize>>) {
    for row in array {
        print!("{}", row[0]);
        for component_index in 1..w {
            print!(" {}", row[component_index]);
        }
        print!("\n");
    }
}