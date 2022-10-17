#![allow(non_snake_case)]
use std::fs::File;
use std::io::Read;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	// *** file read ***
	let mut file: File;
	if args.len() == 2 {
		file = File::open(&args[1]).unwrap();
	} else {
		file = File::open("inputs/data10.txt").unwrap();
	}
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	let mut lines = contents.lines();
	let nm = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
	let n = nm[0];
	let m = nm[1];
	let times = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
	let connections = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();	
	let mut graph = vec![vec![0; n]; n];
	for i in 0..m {
		let a = connections[2 * i] - 1;
		let b = connections[2 * i + 1] - 1;
		graph[a][b] = 1;
	}
	
	// *** cycle detection ***
	if is_cycle(&graph, n) {
		println!("Cycle detected");
		return;
	}
	
	// *** topological sort ***
	let mut sorted = vec![];
	let mut visited = vec![false; n];
	for i in 0..n {
		if !visited[i] {
			topological_sort(&graph, i, &mut visited, &mut sorted);
		}
	}
	sorted.reverse();

	let mut ESs = vec![0; n];
	let mut EFs = vec![0; n];
	let mut LSs = vec![0; n];
	let mut LFs = vec![0; n];

	// *** forward operations (Early Start, Early Finish) ***
	for i in sorted.iter(){
		for j in sorted.iter() {
			if *i == *j { continue; }
			if graph[*j][*i] == 1 {
				if ESs[*i] < EFs[*j] {
					ESs[*i] = EFs[*j];
				}
			}
		}
		EFs[*i] = ESs[*i] + times[*i];
	}

	// *** backward operations (Late Start, Late Finish) ***
	LSs[sorted[n - 1]] = EFs[sorted[n - 1]];
	LFs[sorted[n - 1]] = EFs[sorted[n - 1]];
	for i in sorted.iter().rev() {
		let mut min = *EFs.iter().max().unwrap();
		for j in sorted.iter().rev() {
			if *i == *j { continue; }
			if graph[*i][*j] == 1 {
				if LSs[*j] < min {
					min = LSs[*j];
				}
			}
		}
		let _temp = times[*i];
		LSs[*i] = min - times[*i];
		LFs[*i] = min;
	}

	
	// *** critical path ***
	let mut TFs = vec![0; n];
	let mut critical_path = vec![];
	for i in sorted.iter() {
		TFs[*i] = LSs[*i] - ESs[*i];
		if TFs[*i] == 0 {
			critical_path.push(*i);
		}
	}
	// *** output ***
	println!("Tasks:");
	for i in 0..n {
		print!("Task {}: ", i+1);
		print!("Time: {}, ", times[i]);
		print!("ES: {}, ", ESs[i]);
		print!("EF: {}, ", EFs[i]);
		print!("LS: {}, ", LSs[i]);
		print!("LF: {}, ", LFs[i]);
		print!("TF: {} ", TFs[i]);
		println!("");
	}
	println!("Process time: {}", *LFs.iter().max().unwrap());
	print!("Critical path: ");
	for i in 0..critical_path.len() {
		print!("{} ", critical_path[i] + 1);
	}
	println!("");
	println!("Critical path length: {}", critical_path.len());
}

fn is_cycle(graph: &Vec<Vec<usize>>, n: usize) -> bool {	
	let mut visited = vec![0; n];
	let mut stack = vec![0; n];
	let mut stack_size = 0;
	let mut cycle = false;
	for i in 0..n {
		if visited[i] == 0 {
			visited[i] = 1;
			stack[stack_size] = i;
			stack_size += 1;
			while stack_size > 0 {
				let node = stack[stack_size - 1];
				let mut found = false;
				for j in 0..n {
					if graph[node][j] == 1 {
						if visited[j] == 0 {
							visited[j] = 1;
							stack[stack_size] = j;
							stack_size += 1;
							found = true;
							break;
						} else if visited[j] == 1 {
							cycle = true;
							break;
						}
					}
				}
				if !found {
					visited[node] = 2;
					stack_size -= 1;
				}
			}
		}
	}
	return cycle;
}

fn topological_sort(graph: &Vec<Vec<usize>>, node: usize, visited: &mut Vec<bool>, sorted: &mut Vec<usize>) {
	visited[node] = true;
	for i in 0..graph.len() {
		if graph[node][i] == 1 && !visited[i] {
			topological_sort(graph, i, visited, sorted);
		}
	}
	sorted.push(node);
}
