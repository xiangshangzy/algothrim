use std::{thread, time};

use tauri::{window, App, AppHandle, Window};

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
    fn a_display(&mut self, window: Window, state: [[i32; 3]; 3]) {
        let mut visited: Vec<Node> = Vec::with_capacity(CAP);
        let mut search: Vec<Node> = Vec::with_capacity(CAP);
        search.push(Node::new(state, 0, self.evaluate_h(state)));
        'lable: loop {
            search.sort_by(|a, b| b.f.cmp(&a.f));
            let min_state = search.pop().unwrap();

            if min_state.h == 0 {
                break 'lable;
            }
            let mut state: [i32; 9] = [0; 9];
            for i in 0..N {
                state[i * N..(i + 1) * N].clone_from_slice(&min_state.state[i]);
            }
            window.emit("state", Payload { state }).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
            self.count += 1;
            for offset in OFFSEIS {
                let (x_blank, y_blank) = blank(&min_state.state);
                let x: isize = x_blank as isize + offset[0];
                let y: isize = y_blank as isize + offset[1];
                if x >= 0 && x < 3 && y >= 0 && y < 3 {
                    let x = x as usize;
                    let y = y as usize;
                    let mut arr = min_state.state;
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
                    search.push(Node::new(arr, min_state.g + 1, h));
                }
            }
            visited.push(min_state);
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
#[derive(Clone, serde::Serialize)]
struct Payload {
    state: [i32; 9],
}

#[tauri::command]
pub fn search(window: Window, state: [i32; N * N], target: [i32; N * N]) {
    thread::spawn(move || {
        let mut state_: [[i32; N]; N] = [[0; N]; N];
        let mut target_: [[i32; N]; N] = [[0; N]; N];
        for i in 0..N {
            state_[i].clone_from_slice(&state[i * N..(i + 1) * N]);
            target_[i].clone_from_slice(&target[i * N..(i + 1) * N]);
        }
        let mut chess = Chess::new(target_);
        if !chess.exsit(state_) {
            return;
        }
        chess.a_display(window, state_);
    });
}
