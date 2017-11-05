#![feature(vec_remove_item)]
#![feature(rand)]
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::Ordering;
use std::__rand::thread_rng;
use std::__rand::Rng;
use std::fmt;

#[derive(Eq)]
struct Job {
	name: String,
	p: i32,
	d: i32
}

impl PartialEq for Job {
	fn eq(&self, other: &Job) -> bool {
		self.name == other.name && self.p == other.p && self.d == other.d
	}
}

impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Job {{ name: {}, p: {}, d: {} }}", self.name, self.p, self.d)
    }
}

struct Schedule<> {
	jobs: Vec<Job>,
	cost: f32
}

fn read_job_list(filename: &String) -> Vec<Job> {
	let path = Path::new(filename);

	let mut file = match File::open(&path) {
		Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
		Ok(file) => file,
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
		Ok(_) => (),
	};
	
	let mut jobs = vec![];
	for line in s.split_whitespace() {
		let tokens: Vec<&str> = line.split('/').collect();
		let name = tokens[0].to_string();
		let p = tokens[1].parse::<i32>().unwrap();
		let d = tokens[2].parse::<i32>().unwrap();
		let job = Job { name, p , d };
		jobs.push(job);
	}

	return jobs;
}

fn edf_scheduler(mut jobs: Vec<Job>) -> Schedule {
	jobs.sort_by(|a, b| a.d.cmp(&b.d));
	let mut max = <i32>::min_value();
	let mut time = 0;
	for j in &jobs {
		time += j.p;
		if max < -(time - j.d) {
			max = -(time - j.d);
		}
	}
	return Schedule{ jobs, cost: max as f32 };
}

fn lawler_cost(time: &i32, deadline: &i32) -> f32 {
	let cost = (time - deadline/2) as f32;
	if cost < 0. {
		return 0.;
	} else {
		return cost.sqrt();
	}
}

fn lawler_get_next(jobs: &Vec<Job>, time: &i32) -> Job {
	let mut max = -1.;
	let mut max_j = &jobs[0];
	for j in jobs {
		if max < lawler_cost(time, &j.d) {
			max = lawler_cost(time, &j.d);
			max_j = j;
		}
	}
	return Job{name: max_j.name.clone(), p: max_j.p, d: max_j.d};
}

fn lawler_scheduler(mut jobs: Vec<Job>) -> Schedule {
	let mut n_jobs = vec![];
	let mut time = (&jobs).into_iter().fold(0, |sum, x| sum + x.p).clone();
	let mut max = 0.;
	while time > 0 {
		let j = lawler_get_next(&jobs, &time);
		if max < lawler_cost(&time, &j.d) {
			max = lawler_cost(&time, &j.d);
		}
		time -= j.p;
		jobs.remove_item(&j);
		n_jobs.push(j);
	}
	return Schedule{ jobs: n_jobs, cost: max as f32};
}

fn wsrt_scheduler(mut jobs: Vec<Job>) -> Schedule {
	jobs.sort_by(|a, b| ((a.d/a.p) as f32).partial_cmp(&((b.d / b.p) as f32)).unwrap_or(Ordering::Equal));
	let mut sum = 0.;
	let mut time = 0;
	for j in &jobs {
		time += j.p;
		sum += (j.d * time) as f32;
	}
	return Schedule{ jobs, cost: sum };
}

fn rnd_scheduler(mut jobs: Vec<Job>) -> Schedule {
	thread_rng().shuffle(&mut jobs);
	let mut sum = 0.;
	let mut time = 0;
	for j in &jobs {
		time += j.p;
		sum += (j.d * time) as f32;
	}
	return Schedule{ jobs, cost: sum };
}

fn print_schedule(schedule: Schedule) {
	println!("The jobs are scheduled in the following order:");
	for j in &schedule.jobs {
		print!("{},", j.name);
	}
	println!();
	println!("The total cost is: {}", schedule.cost);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    let scheduler = &args[1];

    let joblist = read_job_list(filename);
    let schedule = match scheduler.as_str() {
    	"edf" => edf_scheduler(joblist),
	"wsrt" => wsrt_scheduler(joblist),
	"lawler" => lawler_scheduler(joblist),
	_ => rnd_scheduler(joblist),
    };
    print_schedule(schedule);
}
