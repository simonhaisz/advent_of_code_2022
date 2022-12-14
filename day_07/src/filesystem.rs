#[derive(Clone)]
pub struct FileSystem {
	root: Dir,
}

#[derive(Clone)]
pub struct Dir {
	name: String,
	contents: Vec<FileSystemObject>,
}

#[derive(Clone)]
pub struct File {
	name: String,
	size: u64,
}

#[derive(Clone)]
pub enum FileSystemObject {
	Dir(Dir),
	File(File),
}

pub struct Cursor {
	components: Vec<String>,
}

impl FileSystem {
	pub fn new() -> Self {
		Self { root: Dir::new("") }
	}

	pub fn find(&self, cursor: &Cursor) -> Option<FileSystemObject> {
		let mut current = Some(FileSystemObject::Dir(self.root.clone()));
		for path_component in cursor.components.iter() {
			match current.as_ref().unwrap().clone() {
				FileSystemObject::Dir(dir) => {
					for dir_content in dir.contents.iter() {
						match dir_content {
							FileSystemObject::Dir(dod) => {
								if dod.name == *path_component {
									current.replace(FileSystemObject::Dir(dod.clone()));
								}
							},
							FileSystemObject::File(dof) => {
								if dof.name == *path_component {
									current.replace(FileSystemObject::File(dof.clone()));
								}
							},
						}
					}
				},
				FileSystemObject::File(_) => return None,
			}
		}
		current
	}
}

impl Dir {
	pub fn new(name: &str) -> Self {
		Self { name: name.to_string(), contents: vec![] }
	}

	pub fn add(&mut self, fs_object: FileSystemObject) {
		self.contents.push(fs_object);
	}
}

impl File {
	pub fn new(name: &str, size: u64) -> Self {
		Self { name: name.to_string(), size }
	}
}

impl Cursor {
	pub fn new() -> Self {
		Self { components: vec![] }
	}

	pub fn move_root(&mut self) {
		self.components.clear();
	}

	pub fn move_up(&mut self) {
		self.components.pop();
	}

	pub fn move_down(&mut self, target: &str) {
		self.components.push(target.to_string());
	}
}