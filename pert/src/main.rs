#![allow(non_snake_case)]
use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	// *** file read ***
	let mut file: File;
	if args.len() < 2 {
		println!("Usage: pert <input file> [<cdf/inv> <input value>]");
		return;
	} else {
		file = File::open(&args[1]).unwrap();
	}
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	let mut lines = contents.lines();
	let nm = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
	let n = nm[0];
	let m = nm[1];
	let all_times = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
	let mut atimes = vec![0; n];
	let mut mtimes = vec![0; n];
	let mut btimes = vec![0; n];
	for i in 0..n {
		atimes[i] = all_times[3 * i];
		mtimes[i] = all_times[3 * i + 1];
		btimes[i] = all_times[3 * i + 2];
	}
	let connections = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();	
	let mut graph = vec![vec![0; n]; n];
	for i in 0..m {
		let a = connections[2 * i] - 1;
		let b = connections[2 * i + 1] - 1;
		graph[a][b] = 1;
	}
	let mut times = vec![0; n];
	let mut sigmas = vec![0.0; n];
	for i in 0..n {
		times[i] = (atimes[i] + 4 * mtimes[i] + btimes[i]) / 6;
		sigmas[i] = ((btimes[i] - atimes[i]) as f64 / 6.0).powf(2.0);
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
	// *** statistics ***
	let mu = *LFs.iter().max().unwrap();
	let mut variance = 0.0;
	for i in 0..critical_path.len() {
		variance += sigmas[critical_path[i]];
	}
	let sigma = variance.sqrt();
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
	print!("Critical path: ");
	for i in 0..critical_path.len() {
		print!("{} ", critical_path[i] + 1);
	}
	println!("");
	println!("Critical path length: {}", critical_path.len());
	println!("");
	println!("Statistic values: ");
	println!("mu = {}", mu);
	println!("sigma^2 = {}", variance);
	println!("sigma = {}", sigma);
	if args.len() < 4 {
		println!("Probability values not calculated (usage: pert <input file> <cdf/inv> <input value>)");
		return;
	}
	println!("");
	if &args[2] == "cdf" {
		let x = args[3].parse::<f64>().unwrap();
		println!("Probability of finishing the project in time {}: {:.2}%", x, cdf(x, mu, sigma) * 100.0);
	} else if &args[2] == "inv" {
		let p = args[3].parse::<f64>().unwrap() / 100.0;
		println!("The project will finish in time {:.2} with {:.2}% probability", inv_cdf(p, mu, sigma), p * 100.0);
	} else {
		println!("Invalid argument: {}", args[2]);
	}
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

fn cdf(x: f64, mu: usize, sigma: f64) -> f64 {
	const A: f64 = 0.2316419;
	const A1: f64 = 0.31938153;
	const A2: f64 = -0.356563782;
	const A3: f64 = 1.781477937;
	const A4: f64 = -1.821255978;
	const A5: f64 = 1.330274429;
	let z = (x - mu as f64) / sigma; // standardize
	let t = 1.0 / (1.0 + A * z.abs()); // t = 1 / (1 + a * z)
	let d = (1.0 / (2.0 * PI as f64).sqrt()) * (-z * z / 2.0).exp(); // d = (1 / sqrt(2 * pi)) * e^(-z^2 / 2)
	let mut prob = d * t * (A1 + t * (A2 + t * (A3 + t * (A4 + t * A5)))); // probability
	if z > 0.0 {
		prob = 1.0 - prob;
	}
	return prob;
}

fn inv_cdf(p: f64, mu: usize, sigma: f64) -> f64 {
	let mut x = -1.0; // initial guess
	let mut low = 0.0; // lower bound
	let mut high = 100.0; // upper bound
	let mut mid = 0.0; // midpoint
	while (x - p).abs() > 0.0000001 { // while not close enough
		mid = (low + high) / 2.0; // find midpoint
		x = cdf(mid, mu, sigma); // find cdf at midpoint
		if x > p { // if midpoint is too high
			high = mid; // set upper bound to midpoint
		} else { // if midpoint is too low
			low = mid; // set lower bound to midpoint
		}
	}
	return mid;
}
