struct Chess {
    target: [[i32; 3]; 3],
    count: i32,
}
struct Node {
    state: [[i32; 3]; 3],
    f: i32,
    g: i32,
    h: i32,
}
const CAP: usize = 362880;
const OFFSEIS: [[isize; 2]; 4] = [[0, -1], [0, 1], [-1, 0], [1, 0]];
impl Node {
    fn new(state: [[i32; 3]; 3], g: i32, h: i32) -> Self {
        Self {
            state,
            f: g + h,
            g,
            h,
        }
    }
}
fn blank(state: &[[i32; 3]; 3]) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] == 0 {
                x = i;
                y = j;
            }
        }
    }
    (x, y)
}
impl Chess {
    fn new(target: [[i32; 3]; 3]) -> Chess {
        Self { target, count: 0 }
    }

    fn bfs(&self, states: Vec<[[i32; 3]; 3]>) {
        let mut children: Vec<[[i32; 3]; 3]> = Vec::new();
        for state in states {
            for offset in OFFSEIS {
                let (x_blank, y_blank) = blank(&state);
                let x: isize = x_blank as isize + offset[0];
                let y: isize = y_blank as isize + offset[1];
                if x >= 0 && x < 3 && y >= 0 && y < 3 {
                    let x = x as usize;
                    let y = y as usize;
                    let mut child = state;
                    let tmp = child[x_blank][y_blank];
                    child[x_blank][y_blank] = child[x][y];
                    child[x][y] = tmp;
                    debug(child);
                    if child == self.target {
                        println!("ok");
                        return;
                    }
                    children.push(child);
                }
            }
        }
        self.bfs(children);
    }
    fn evaluate_h(&self, state: [[i32; 3]; 3]) -> i32 {
        let mut g: i32 = 0;
        for i in 0..3 {
            for j in 0..3 {
                if state[i][j] != 0 && state[i][j] != self.target[i][j] {
                    g += 1;
                }
            }
        }
        g
    }
    fn exsit(&self, state: [[i32; 3]; 3]) -> bool {
        let arr = state.concat();
        let arr2 = self.target.concat();
        let mut n1 = 0;
        let mut n2 = 0;
        for i in 0..8 {
            for j in i + 1..9 {
                if arr[i] != 0 && arr[j] > arr[i] {
                    n1 += 1;
                }
            }
        }
        for i in 0..8 {
            for j in i + 1..9 {
                if arr2[i] != 0 && arr2[j] > arr2[i] {
                    n2 += 1;
                }
            }
        }
        if n1 % 2 == n2 % 2 {
            return true;
        }
        false
    }
    fn a_star(&mut self, state: [[i32; 3]; 3]) {
        let mut visited: Vec<Node> = Vec::with_capacity(CAP);
        let mut search: Vec<Node> = Vec::with_capacity(CAP);
        search.push(Node::new(state, 0, self.evaluate_h(state)));
        'lable: loop {
            search.sort_by(|a, b| b.f.cmp(&a.f));
            let min_f = search.pop().unwrap();
            if min_f.h == 0 {
                break 'lable;
            }

            self.count += 1;
            for offset in OFFSEIS {
                let (x_blank, y_blank) = blank(&min_f.state);
                let x: isize = x_blank as isize + offset[0];
                let y: isize = y_blank as isize + offset[1];
                if x >= 0 && x < 3 && y >= 0 && y < 3 {
                    let x = x as usize;
                    let y = y as usize;
                    let mut arr = min_f.state;
                    let tmp = arr[x_blank][y_blank];
                    arr[x_blank][y_blank] = arr[x][y];
                    arr[x][y] = tmp;

                    for node in &search {
                        if node.state == arr {
                            continue;
                        }
                    }
                    for node in &visited {
                        if node.state == arr {
                            continue;
                        }
                    }
                    let h = self.evaluate_h(arr);
                    if h == 0 {
                        return;
                    }
                    search.push(Node::new(arr, min_f.g + 1, h));
                }
            }
            visited.push(min_f);
        }
    }
}
fn debug(state: [[i32; 3]; 3]) {
    for row in state {
        for item in row {
            print!("{} ", item)
        }
        println!("")
    }
    println!("")
}
const N: usize = 3;
pub fn search(arr1: [i32; N * N], arr2: [i32; N * N]) {
    let mut state: [[i32; N]; N] = [[0; N]; N];
    let mut target: [[i32; N]; N] = [[0; N]; N];
    for i in 0..N {
        state[i].clone_from_slice(&arr1[i * N..(i + 1) * N]);
        target[i].clone_from_slice(&arr2[i * N..(i + 1) * N]);
    }
    let mut chess = Chess::new(target);
    if chess.exsit(state) {
        chess.a_star(state);
        println!("count: {} ", chess.count)
    } else {
        println!("no exsit solution");
    }
}

