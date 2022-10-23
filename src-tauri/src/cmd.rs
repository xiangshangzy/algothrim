struct Chess {
    target: [[i32; 3]; 3],
}
struct Node {
    state: [[i32; 3]; 3],
    f: i32,
    g: i32,
    h: i32,
    pre: [[i32; 3]; 3],
}
#[derive(Clone, serde::Serialize)]
struct Payload {
    state: [i32; 9],
}
const CAP: usize = 362880;
const OFFSEIS: [[isize; 2]; 4] = [[0, -1], [0, 1], [-1, 0], [1, 0]];
impl Node {
    fn new(state: [[i32; 3]; 3], g: i32, h: i32, pre: [[i32; 3]; 3]) -> Self {
        Self {
            state,
            f: g + h,   // 总代价
            g,           //当前代价
            h,           //预测代价
            pre,        //记录上一个节点回溯 
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
        Self { target }
    }
    // 暴力bfs搜索
    fn bfs(&self, state: [[i32; 3]; 3]) {
        let mut visited: Vec<Node> = Vec::with_capacity(CAP);
        let mut search: Vec<Node> = Vec::with_capacity(CAP);
        search.push(Node::new(state, 0, self.forecast_h(state), [[0; 3]; 3]));
        loop {
            let min_node = search.pop().unwrap();
            if min_node.h == 0 {
                self.backtrack(&visited, min_node);
                return;
            }
            for offset in OFFSEIS {
                let (x_blank, y_blank) = blank(&min_node.state);
                let x: isize = x_blank as isize + offset[0];
                let y: isize = y_blank as isize + offset[1];
                if x >= 0 && x < 3 && y >= 0 && y < 3 {
                    let x = x as usize;
                    let y = y as usize;
                    let mut arr = min_node.state;
                    let tmp = arr[x_blank][y_blank];
                    arr[x_blank][y_blank] = arr[x][y];
                    arr[x][y] = tmp;
                    search.push(Node::new(arr, min_node.g + 1, 0, min_node.state.clone()));
                }
            }
            visited.push(min_node);
        }
    }
    // 预测状态到目标的代价
    fn forecast_h(&self, state: [[i32; 3]; 3]) -> i32 {
        let mut g: i32 = 0;
        for i in 0..3 {
            for j in 0..3 {
                // 统计不在位数
                if state[i][j] != 0 && state[i][j] != self.target[i][j] {
                    g += 1;
                }
            }
        }
        g
    }
    // 根据逆序数判断能否到达目标状态
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
    // A*算法
    fn a(&mut self, state: [[i32; 3]; 3]) {
        // 已访问队列
        let mut visited: Vec<Node> = Vec::with_capacity(CAP);
        // 搜索列表
        let mut search: Vec<Node> = Vec::with_capacity(CAP);
        search.push(Node::new(state, 0, self.forecast_h(state), [[0; 3]; 3]));
        loop {
            // 搜索队列根据代价f排序
            search.sort_by(|a, b| b.f.cmp(&a.f));
            // 弹出队尾代价最小的节点
            let min_node = search.pop().unwrap();
            // 最小节点的g即为最少经过次数
            if min_node.h == 0 {
                self.backtrack(&visited, min_node);
                return;
            }
            // 依次上下左右对0位移
            for offset in OFFSEIS {
                // 获取状态为0的点坐标
                let (x_blank, y_blank) = blank(&min_node.state);
                let x: isize = x_blank as isize + offset[0];
                let y: isize = y_blank as isize + offset[1];
                // 判断点在棋盘内
                if x >= 0 && x < 3 && y >= 0 && y < 3 {
                    let x = x as usize;
                    let y = y as usize;
                    let mut arr = min_node.state;
                    let tmp = arr[x_blank][y_blank];
                    arr[x_blank][y_blank] = arr[x][y];
                    arr[x][y] = tmp;
                    //    跳过已访问的节点
                    for node in &search {
                        if node.state == arr {
                            continue;
                        }
                    }
                    //  跳过已在搜索列表的节点
                    for node in &visited {
                        if node.state == arr {
                            continue;
                        }
                    }
                    let h = self.forecast_h(arr);
                    search.push(Node::new(arr, min_node.g + 1, h, min_node.state.clone()));
                }
            }
            // 记录已访问节点
            visited.push(min_node);
        }
    }
    // 目标状态回溯到初始状态
    fn backtrack(&self, visited: &Vec<Node>, target: Node) {
        let mut p = &target;
        'lable: loop {
            for node in visited {
                let state = node.state;
                // 查找搜索树的上一节点
                if p.pre == state {
                    p = node;
                    debug(state);
                    // 遍历到根节点终止
                    if node.g == 0 {
                        break 'lable;
                    }
                    break;
                }
            }
        }
    }
}

// 打印状态
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

// 输入初始状态和目标
pub fn search(init: [i32; N * N], target: [i32; N * N]) {
    let mut state_: [[i32; N]; N] = [[0; N]; N];
    let mut target_: [[i32; N]; N] = [[0; N]; N];
    // 将输入的一维数组转成二维数组
    for i in 0..N {
        state_[i].clone_from_slice(&init[i * N..(i + 1) * N]);
        target_[i].clone_from_slice(&target[i * N..(i + 1) * N]);
    }
    let mut chess = Chess::new(target_);
    // 判断是否有解
    if !chess.exsit(state_) {
        return;
    }
    // 有解则a*搜索
    chess.a(state_);
}
