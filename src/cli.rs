use std;

const POST_INPUT_SPACER: &str = "-----\n";

pub fn match_first<S, S2>(input: S, v: &[S2]) -> Option<usize>
where
    S: Into<String>,
    S2: Into<String> + Clone,
{
    let input = input.into().to_lowercase();
    if input.len() == 0 {
        return None;
    }
    let mut v_it = v.iter().cloned().map(|s: S2| {
        let s: String = s.into();
        s.to_lowercase()
    });
    v_it.position(|s: String| {
        let input: &str = input.as_ref();
        let s: &str = s.as_ref();
        s.contains(input)
    })
}

pub fn prompt(query: &str) -> String {
    println!("{}", query);
    //print!("{}", PROMPT);
    // TODO: flush
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    input
}

pub struct Menu {
    story: String,
    options: Vec<(String, String)>,
    prompt: String,
}

impl Menu {
    // Returns action string
    pub fn show(&self) -> Option<&str> {
        println!("{}\n", self.story);

        for (option, _) in &self.options {
            println!("  {}", option);
        }

        println!("");

        // Query input
        let input = prompt(&self.prompt);
        println!("{}", POST_INPUT_SPACER);

        match match_first(
            input.trim().as_ref(),
            &self
                .options
                .iter()
                .map(|(s, _)| s.as_ref())
                .collect::<Vec<&str>>(),
        ) {
            Some(pos) => Some(&self.options[pos].1),
            None => None,
        }
    }
}

pub struct MenuBuilder(Menu);

impl MenuBuilder {
    pub fn new(story: &str) -> MenuBuilder {
        MenuBuilder(Menu {
            story: story.to_string(),
            options: vec![],
            prompt: String::new(),
        })
    }
    pub fn add_pretext(mut self, text: &str) -> MenuBuilder {
        self.0.story.push_str(text);
        self
    }
    pub fn prompt(mut self, prompt: &str) -> MenuBuilder {
        self.0.prompt = prompt.to_string();
        self
    }
    pub fn options<S>(mut self, options: Vec<(S, S)>) -> MenuBuilder
    where
        S: AsRef<str>,
    {
        self.0.options = options
            .iter()
            .map(|(s, act)| (s.as_ref().to_string(), act.as_ref().to_string()))
            .collect();
        self
    }
    pub fn build(self) -> Menu {
        self.0
    }
}
