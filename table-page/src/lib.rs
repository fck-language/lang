use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::path::PathBuf;
use rand::Rng;

pub fn display(languages: Vec<((String, String), Vec<[u16; 256]>, Vec<[u8; 256]>, Vec<[u8; 256]>)>) -> std::io::Result<()> {
	let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	out_dir.extend(["..", "target", "html-tables"]);
	std::fs::create_dir_all(&out_dir)?;
	let mut html = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_dir.join("index.html"))?;
	std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_dir.join("useful.js"))?.write(include_bytes!("../useful.js"))?;
	std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_dir.join("index.js"))?.write(include_bytes!("../index.js"))?;
	out_dir.extend(["lang"]);
	std::fs::create_dir_all(&out_dir)?;
	let mut options = String::new();
	let mut switch = String::new();
	let mut langs = String::new();
	for ((full, short), map1, map2, map3) in languages {
		options += &*format!("<option>{} ({})</option>", full, short);
		switch += &*format!("case \"{} ({})\": return {1};", full, short);
		langs += &*format!("<script src=\"lang/{}.js\"></script>", short);
		let mut graph = Graph::new();
		for _ in 0..map1.len() { graph.new_node(); }
		for (i, (r1, r2)) in map1.iter().zip(map2.iter()).enumerate() {
			let mut blank_accept: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
			for (b, row) in r1.iter().enumerate() {
				match (row, r2[b]) {
					(0, 0) => {},
					(r, 0) => graph.new_transition(i, *r as usize, b as u8, None),
					(0, tt) => {
						if let Some(val) = blank_accept.get_mut(&(tt, map3[i][b])) {
							val.push(b as u8)
						} else {
							blank_accept.insert((tt, map3[i][b]), vec![b as u8]);
						}
					},
					(r, tt) => {
						graph.new_transition(i, *r as usize, b as u8, None);
						if let Some(val) = blank_accept.get_mut(&(tt, map3[i][b])) {
							val.push(b as u8)
						} else {
							blank_accept.insert((tt, map3[i][b]), vec![b as u8]);
						}
					}
				}
			}
			for ((tt, td), map) in blank_accept {
				graph.0[i].children.push(SimpleNode {
					tt,
					td,
					required: map,
				})
			}
		}
		graph.reposition();
		std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(out_dir.join(format!("{}.js", short)))?.write(format!("let {} = {:?};", short, graph).as_bytes())?;
	}
	let html_str = format!(
		include_str!("../index.html"),
		format!("{}<script>const get_lang=(n)=>{{switch(n){{{}default: return}}}}</script>", langs, switch),
		options);
	html.write(html_str.as_bytes())?;
	html.try_clone()?;
	Ok(())
}

struct Graph (Vec<Node>);

impl Graph {
	pub fn new() -> Self {
		Self(Vec::new())
	}
	
	pub fn new_transition(&mut self, source: usize, end: usize, byte: u8, name: Option<String>) {
		self.0[source].new_transition(end, byte, name);
	}
	
	pub fn new_group_transition(&mut self, source: usize, end: usize, bytes: Vec<u8>, name: Option<String>) {
		self.0[source].new_group_transition(end, bytes, name);
	}
	
	pub fn new_node(&mut self) -> usize {
		let index = self.0.len();
		let mut rng = rand::thread_rng();
		self.0.push(Node {
			position: Position(rng.gen_range(-500..500), rng.gen_range(-500..500)),
			displacement: Position(0., 0.),
			index,
			is_accepting: false,
			transitions: vec![],
			children: vec![]
		});
		index
	}
	
	pub fn new_accepting_node(&mut self) -> usize {
		let index = self.0.len();
		let mut rng = rand::thread_rng();
		self.0.push(Node {
			position: Position(rng.gen_range(-500..500), rng.gen_range(-500..500)),
			displacement: Position(0., 0.),
			index,
			is_accepting: true,
			transitions: vec![],
			children: vec![]
		});
		index
	}
	
	/// Reposition nodes using the Fruchterman-Reingold algorithm
	pub fn reposition(&mut self) {
		let k: f64 = 1_000.;
		
		// iteratively do better
		for i in 0..100 {
			let t = 50. * (-0.05 * i as f64).exp();
			for v in 0..self.0.len() {
				for u in 0..self.0.len() {
					if v != u {
						let delta: Position<f64> = (self.0[v].position - self.0[u].position).into();
						self.0[v].displacement += delta.normalise() * k * k / delta.length()
					}
				}
			}
			for v_i in 0..self.0.len() {
				let mut v = self.0[v_i].clone();
				for e in v.transitions.iter() {
					let mut u = self.0[e.destination].clone();
					let delta: Position<f64> = (v.position - u.position).into();
					let delta = delta * delta.length() / k;
					v.displacement -= delta;
					u.displacement += delta;
					self.0[e.destination] = u;
				}
				self.0[v_i] = v;
			}
			for v in self.0.iter_mut() {
				v.position += (v.displacement.normalise() * t.min(v.displacement.length())).into();
				v.displacement = Position(0., 0.);
			}
		}
	}
}

impl Debug for Graph {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut states = (0..self.0.len()).map(|i| format!("\"q{}\"", i)).collect::<Vec<_>>();
		let mut accepting = vec!["false"; self.0.len()];
		let mut positions = self.0.iter().map(|i| i.position).collect::<Vec<_>>();
		let mut transitions = Vec::new();
		for (i, v) in self.0.iter().enumerate() {
			for e in v.transitions.iter() {
				let [name, digits] = e.name();
				transitions.push(format!("\"q{},q{}\": [{:?}, {:?}]", i, e.destination, name, digits))
			}
		}
		let mut i = self.0.len();
		for n in self.0.iter() {
			let step = std::f64::consts::TAU / n.children.len() as f64;
			for (c_i, c) in n.children.iter().enumerate() {
				states.push(format!("\"q{}\"", i));
				accepting.push("true");
				let pos = n.position + Position(((c_i as f64 * step).sin() * 200.) as i32, ((c_i as f64 * step).cos() * 200.) as i32);
				positions.push(pos);
				transitions.push(format!("\"q{},q{}\": [{:?}, {:?}]", n.index, i, format!("[{},{}]", c.tt, c.td), group_fmt(c.required.clone())));
				i += 1;
			}
		}
		let min_x = positions.iter().min_by_key(|p| p.0).map(|p| p.0).unwrap_or(0);
		let min_y = positions.iter().min_by_key(|p| p.1).map(|p| p.1).unwrap_or(0);
		let pos = Position(min_x - 100, min_y - 100);
		positions.iter_mut().for_each(|p| *p -= pos);
		let positions = positions.iter().map(|t| format!("[{},{}]", t.0, t.1)).collect::<Vec<_>>();
		write!(f, "{{states: [{}], accepting: [{}], positions: [{}], transitions: {{{}}}}}",
			   states.join(", "), accepting.join(", "), positions.join(", "), transitions.join(", ")
		)
	}
}

#[derive(Clone)]
struct Node {
	position: Position<i32>,
	displacement: Position<f64>,
	index: usize,
	is_accepting: bool,
	transitions: Vec<Transition>,
	children: Vec<SimpleNode>
}

#[derive(Clone)]
struct SimpleNode {
	tt: u8,
	td: u8,
	required: Vec<u8>
}

impl Node {
	pub fn new_transition(&mut self, end: usize, byte: u8, name: Option<String>) {
		if let Some(t) = self.transitions.iter_mut().find(|t| t.destination == end) {
			t.requirement.push(byte)
		} else {
			self.transitions.push(Transition {
				destination: end,
				name,
				requirement: vec![byte],
			})
		}
	}
	pub fn new_group_transition(&mut self, end: usize, bytes: Vec<u8>, name: Option<String>) {
		self.transitions.push(Transition {
			destination: end,
			name,
			requirement: bytes,
		})
	}
}

#[derive(PartialEq, Clone)]
struct Transition {
	destination: usize,
	name: Option<String>,
	requirement: Vec<u8>
}

fn group_fmt(mut requirements: Vec<u8>) -> String {
	requirements.sort();
	let mut requirements = requirements.iter().map(|t| *t as i16);
	let mut prev;
	let mut buf;
	if let Some(f) = requirements.next() {
		prev = f;
		buf = f;
	} else {
		return String::new();
	}
	let mut req_groups = Vec::new();
	for i in requirements {
		if prev + 1 != i {
			if buf == prev {
				req_groups.push(format!("{}", buf))
			} else {
				req_groups.push(format!("{}-{}", buf, prev))
			}
			buf = i;
		}
		prev = i;
	}
	if buf == prev {
		req_groups.push(format!("{}", buf))
	} else {
		req_groups.push(format!("{}-{}", buf, prev))
	}
	req_groups.join(",")
}

impl Transition {
	fn name(&self) -> [String; 2] {
		if let Some(name) = &self.name {
			[name.clone(), group_fmt(self.requirement.clone())]
		} else {
			[String::new(), group_fmt(self.requirement.clone())]
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
struct Position<T: Sized + Copy + Clone + Debug> (T, T);

impl<T: Sized + Copy + Clone + Debug + Mul<Output=T> + Add<Output=T> + Into<f64> + From<f64>> Position<T> {
	pub fn length(&self) -> f64 {
		(self.0 * self.0 + self.1 * self.1).into().sqrt()
	}
	
	pub fn normalise(&self) -> Self {
		*self / self.length()
	}
}

impl<T: Sized + Copy + Clone + Debug + Add<Output=T>> Add for Position<T> {
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl<T: Sized + Copy + Clone + Debug + AddAssign> AddAssign for Position<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl<T: Sized + Copy + Clone + Debug + Sub<Output=T>> Sub for Position<T> {
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0, self.1 - rhs.1)
	}
}

impl<T: Sized + Copy + Clone + Debug + SubAssign> SubAssign for Position<T> {
	fn sub_assign(&mut self, rhs: Self) {
		self.0 -= rhs.0;
		self.1 -= rhs.1;
	}
}

impl<T: Sized + Copy + Clone + Debug + Mul<Output=T> + Into<f64> + From<f64>> Div<f64> for Position<T> {
	type Output = Self;
	
	fn div(self, rhs: f64) -> Self::Output {
		Position((self.0.into() / rhs).into(), (self.1.into() / rhs).into())
	}
}

impl<T: Sized + Copy + Clone + Debug + Mul + Into<f64> + From<f64>> Mul<f64> for Position<T> {
	type Output = Self;
	
	fn mul(self, rhs: f64) -> Self::Output {
		Position((self.0.into() * rhs).into(), (self.1.into() * rhs).into())
	}
}

impl From<Position<i32>> for Position<f64> {
	fn from(value: Position<i32>) -> Self {
		Self(value.0 as f64, value.1 as f64)
	}
}

impl From<Position<f64>> for Position<i32> {
	fn from(value: Position<f64>) -> Self {
		Self(value.0 as i32, value.1 as i32)
	}
}

impl<T: Sized + Copy + Clone + Debug> Debug for Position<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:.3?}, {:.3?})", self.0, self.1)
	}
}
