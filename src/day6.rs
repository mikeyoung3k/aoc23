pub fn run(file: &str) -> (usize,usize) {
    let data = read_data(file);
    let data2 = read_data_2(file);
    (pt1(&data),pt2(&data2))
}

fn read_data(file: &str) -> Vec<(usize, usize)> {
    let file_lines = std::fs::read_to_string(file).expect("File not found");
    let mut file_iter = file_lines.lines();
    let times = file_iter.next().expect("Should have had a row of times").split_whitespace().skip(1).map(|x| x.parse::<usize>().expect("Failed to parse time"));
    let dists = file_iter.next().expect("Should have had a row of dists").split_whitespace().skip(1).map(|x| x.parse::<usize>().expect("Failed to parse distance"));

    times.zip(dists).collect::<Vec<(usize, usize)>>()
}

fn read_data_2(file: &str) -> (usize,usize) {
    let file_lines = std::fs::read_to_string(file).expect("File not found");
    let mut file_iter = file_lines.lines();
    let time = file_iter.next().expect("Should have a line").split(':').skip(1).next().expect("Should have a str").replace(" ","").parse::<usize>().expect("Failed to parse time");
    let dist = file_iter.next().expect("Should have a line").split(':').skip(1).next().expect("Should have a str").replace(" ","").parse::<usize>().expect("Failed to parse dist");

    (time,dist)
}

fn pt1(data: &Vec<(usize, usize)>) -> usize {
    data.iter().map(|x| {
        let solved = solve_quad(x.0,x.1);
        range_sols(solved.0,solved.1)
    }).product()

}

fn pt2(data: &(usize,usize)) -> usize {
    let solved = solve_quad(data.0,data.1);
    range_sols(solved.0,solved.1)
}

fn solve_quad(t: usize, d:usize) -> (usize,usize) {
    let b: f64 = -(t as f64);
    let c: f64 = d as f64;
    let mut pos_sol = (-b + (b*b-4.0*c).sqrt())/2.0;
    let mut neg_sol = (-b - (b*b-4.0*c).sqrt())/2.0;
    if neg_sol.ceil() == neg_sol {
        neg_sol += 0.5;
    }

    if pos_sol.floor() == pos_sol{
        pos_sol -= 0.5;
    }
    (neg_sol.ceil() as usize,pos_sol.floor() as usize)
}

fn range_sols(a: usize, b: usize) -> usize {
    if a > b{
        return a+1-b
    }

    return b+1 -a
}