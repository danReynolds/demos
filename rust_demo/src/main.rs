fn main() {
  let dan : Developer = Developer::new("danReynolds", "Dan", Project::Cognitive);
  let blake : Designer = Designer::new("blakestevenson", "Blaké", Project::Cognitive);
  let slacker : Developer = Developer::new("meh", "doSomething McLazy", Project::Idle);

  let developers: [&Developer; 2] = [&dan, &slacker];
  let designers: [&Designer; 1] = [&blake];

  for developer in developers.iter() {
      match developer.project {
          Project::Cognitive => { developer.introduce(); },
          Project::Idle => {}
      }
  }

  let mut designers_iterator = designers.into_iter();
  loop {
      if let Some(designer) = designers_iterator.next() {
          designer.introduce();
      } else {
          break;
      }
  }
}

enum Project {
    Cognitive,
    Idle
}

struct Designer<'a> {
    dribbble: &'a str,
    name: &'a str,
    project: Project
}
impl<'a> Designer<'a> {
    fn new(dribbble: &'a str, name: &'a str, project: Project) -> Self {
        Designer { name: name, dribbble: dribbble, project: project }
    }
}

struct Developer<'a> {
    github: &'a str,
    name: &'a str,
    project: Project
}
impl<'a> Developer<'a> {
    fn new(github: &'a str, name: &'a str, project: Project) -> Self {
        Developer { github: github, name: name, project: project }
    }
}

trait Boltmaker {
    fn introduce(&self);
}

impl<'a> Boltmaker for Designer<'a> {
    fn introduce(&self) {
        println!("Hi my name is {}, you can see some of my work at http://dribbble.com/{}", self.name, self.dribbble);
    }
}

impl<'a> Boltmaker for Developer<'a> {
    fn introduce(&self) {
        println!("Hi my name is {}, my codes are available at http://github.com/{}", self.name, self.github);
    }
}
