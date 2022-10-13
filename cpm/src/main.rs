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
		file = File::open("input.txt").unwrap();
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
		let a = connections[2*i] - 1;
		let b = connections[2*i+1] - 1;
		graph[a][b] = 1;
	}
	
	// *** cycle detection ***
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
	if cycle {
		println!("Cycle detected");
		return;
	}
	
	// *** topological sort ***
	// TODO

	let mut ESs = vec![0; n];
	let mut EFs = vec![0; n];
	let mut LSs = vec![0; n];
	let mut LFs = vec![0; n];

	// *** forward operations (Early Start, Early Finish) ***
	for i in 0..n {
		for j in 0..n {
			if i == j { continue; }
			if graph[j][i] == 1 {
				if ESs[i] < EFs[j] {
					ESs[i] = EFs[j];
				}
			}
		}
		EFs[i] = ESs[i] + times[i];
	}

	// *** backward operations (Late Start, Late Finish) ***
	LSs[n - 1] = EFs[n - 1];
	LFs[n - 1] = EFs[n - 1];
	for i in (0..n).rev() {
		let mut min = EFs[n -1];
		for j in (0..n).rev() {
			if i == j { continue; }
			if graph[i][j] == 1 {
				if LSs[j] < min {
					min = LSs[j];
				}
			}
		}
		LSs[i] = min - times[i];
		LFs[i] = min;
	}
	// *** critical path ***
	let mut TFs = vec![0; n];
	let mut critical_path = vec![];
	for i in 0..n {
		TFs[i] = LSs[i] - ESs[i];
		if TFs[i] == 0 {
			critical_path.push(i);
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
		print!("TF: {}, ", TFs[i]);
		println!("");
	}
	println!("");
	print!("Critical path: ");
	for i in 0..critical_path.len() {
		print!("{} ", critical_path[i]+1);
	}
	println!("");
	println!("Critical path length: {}", critical_path.len());
}
