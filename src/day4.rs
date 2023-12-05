use std::collections::{HashMap,HashSet};

pub fn run(filename: &str) -> (usize,usize) {
    let data = read_data(filename);
    (pt1(&data),pt2(&data))
}

fn read_data(file: &str) -> HashMap<usize,Card> {
    std::fs::read_to_string(file)
    .expect("File not found")
    .lines()
    .map(|x| {
        // X is "Card n: a b c d e | qw e g jh"
        let mut splut = x.split(':');
        let id = splut.next().expect("Expected a Card").split_whitespace().nth(1).expect("Expected an ID").parse::<usize>().expect("Failed to parse ID as usize");
        let mut nums = splut.next().expect("Expected the numbers of a card").split('|');
        let winning_nums = nums.next().expect("Expected a number").split_whitespace().map(|n| n.parse::<usize>().expect("Failed to parse winning num")).collect::<HashSet<usize>>();
        let your_nums = nums.next().expect("Expected a number").split_whitespace().map(|n| n.parse::<usize>().expect("Failed to parse your num")).collect::<HashSet<usize>>();
        (id,Card{
            id,
            winning_nums,
            your_nums,
        })

    })
    .collect::<HashMap<usize,Card>>()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_nums: HashSet<usize>,
    your_nums: HashSet<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let count = self.wins();
        if count == 0 {
            return 0;
        }
        usize::pow(2,(count-1).try_into().expect("not a valid u32"))
    }
    fn wins(&self) -> usize {
        self.winning_nums.intersection(&self.your_nums).count()
    }
}


fn pt1(data: &HashMap<usize,Card>) -> usize {
    data.iter().map(|(_,x)| {
        x.score()
    }).sum()
}

fn pt2(data: &HashMap<usize,Card>) -> usize {
    let win_table = build_win_table(data);
    data.len() + win_table.values().sum::<usize>()
}

fn build_win_table(data: &HashMap<usize,Card>) -> HashMap<usize,usize> {
    let mut res = HashMap::new();
    for id in (1..=data.len()).rev() {
        if let Some(card) = data.get(&id) {
            let mut num_wins = card.wins();
            if num_wins > 0 {
                for won in id+1..=id+num_wins {
                    num_wins += res.get(&won).expect("Expected a value lower in table");
                }
            }
            res.insert(id,num_wins);
        }
    }
    res
}
