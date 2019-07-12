use std::u32;
const INF: u32 = 999999;

fn print_graph(graph: &[[u32;4];4]){
    for r in graph.iter() {
        for c in r {
            if *c == INF {
                print!("{:>5}", "INF");
            }else {
                print!("{:5}", c);
            }
        }
        println!();
    }
}

fn main() {

    let mut graph = [[0, 2, 6, 4], [INF, 0, 3, INF], [7, INF, 0, 1], [5, INF, 12, 0]];
    println!("start:\n-----");
    print_graph(&graph);
    let vertex_num = graph.len();
    for k in 0..vertex_num {
        for v1 in 0..vertex_num {
            for v2 in 0..vertex_num {
                if graph[v1][v2] > graph[v1][k] + graph[k][v2]{
                    graph[v1][v2] = graph[v1][k] + graph[k][v2];
                }
            }
        }
    }
    println!("\nresult:\n-----");
    print_graph(&graph);
}
