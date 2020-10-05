use oorandom;

#[test]
fn research_on_directionary() {
    // 词性分类函数, 找出.dic的标识符以及每个标识符 10 个id
    use std::fs;
    use std::collections::HashMap;
    let mut hash : HashMap<&str, Vec<&str>> = HashMap::new();
    let raw_bytes = fs::read_to_string("resources/ansj_seg-master/default.dic").expect("failed to open directionary file");
    let filterd_bytes = raw_bytes.replace(&['0','1','2','3','4','5','6','7','8','9','\n'][..], "");
    let seperate_words : Vec<&str> = filterd_bytes.split('\t').collect();
    let mut i : u64 = 0;
    let mut last_word : &str = "";
    for tag in seperate_words {
        i = i + 1;
        // 奇数列为word, 偶数列为tag
        if i % 2 != 0 {
            last_word = tag;
            continue;
        }
        let hash_get = hash.get_mut(tag);
        match hash_get {
            None => {
                let vec = vec!(last_word);
                hash.insert(tag, vec);
            }
            Some(vec) => {
                if vec.len() >= 10 {continue;}
                vec.push(last_word);
            }
        }
    }
    println!("{:?}", hash);
}

#[test]
fn add_directionary() {
    let mut random_word = Word::from_literal("okay");
    random_word.set_tag("v");
    let mut random_wor2 = Word::from_literal("no");
    random_wor2.set_tag("l");
    let mut directionary = Directionary::new();
    directionary.add_a_word(&random_word);
    directionary.add_a_word(&random_wor2);
    println!("{:?}", directionary);
}

#[test]
fn create_directionary() {
    let directionary = Directionary::from_default(None, Some(500));
    println!("generation finished!");
    println!("result {:?}", directionary);
}

#[test]
fn create_sentance() {
    let mut resolver = RandomResolver::from_seed(64u128);
    let directionary = Directionary::from_default(None, None);
    let mut sentance = 
            SentanceNode::word("其实")
        .next(
            SentanceNode::icon(',')
        .next(
            SentanceNode::element("Noun")
        .next(
            SentanceNode::word("是")
        .next(
            SentanceNode::element("Noun")
        .next(
            SentanceNode::icon(',')
        .next(
        SentanceNode::word("你知道吗?")
        ))))));
    sentance.resolve_sentance(&mut resolver, &directionary);
    let output = sentance.to_string();
    println!("result {}", output);
}

#[test]
fn multi_sentance() {
    let mut resolver = RandomResolver::from_seed(1024u128);
    let directionary = Directionary::from_default(None, None);
    let generic_sentance = sentance!(
        [element="Who"][word="是"][element= "Adjective"][word="的"][element="Adjective"][element="IntranstiveVerb"][word="器."]
    );
    for _ in 1..255 {
        let mut sentance = generic_sentance.clone();
        sentance.resolve_sentance(&mut resolver, &directionary);
        let output = sentance.to_string();
        println!("result {}", output);
    }
}

#[macro_export]
macro_rules! sentance {
    ([$function:ident=$string:expr]$([$nfunction:ident=$nstring:expr])+
    ) => {
        SentanceNode::$function($string).next(sentance!($([$nfunction=$nstring])+))
    };
    ([$function:ident=$string:expr]) => {
        SentanceNode::$function($string)
    };
}
#[derive(Debug)]
struct Word {
    pub tag : String,
    pub literal : String,
}

impl Word {
    pub fn from_literal(init_literal : &str) -> Word {
        Word{
            tag : String::new(),
            literal : String::from(init_literal)
        }
    }
    pub fn set_tag(&mut self, new_tag : &str) {
        self.tag = String::from(new_tag);
    }
}

#[derive(Debug)]
struct TagMatcher {
    pub matchers_pool : std::collections::BTreeMap<String, Vec<String>>,
}

impl TagMatcher {
    pub fn new() -> TagMatcher {
        use std::collections::BTreeMap;
        TagMatcher{matchers_pool : BTreeMap::new()}
    }
    fn add(mut self, tag : &'static str, matchers : Vec<&'static str>) -> Self
    {   
        let matcher_result = self.matchers_pool.get(tag);
        match matcher_result {
            Some(_) => {                
            }
            None => {
                self.matchers_pool.insert(String::from(tag), Vec::new());
            }
        }
        let matcher_vec = self.matchers_pool.get_mut(tag).unwrap();
        for matcher in matchers {
            matcher_vec.push(String::from(matcher));
        }
        self
    }
    pub fn resolve(&self, tag : String) -> Option<Vec<String>>
    {
        let mut ret_vec : Option<Vec<String>> = None;
        for (element, matchers) in &self.matchers_pool
        {
            for matcher in matchers
            {
                if *matcher == tag
                {
                    match &mut ret_vec {
                        Some(vec) => {
                            vec.push(element.clone());
                        }
                        None => {
                            let vec = vec!(element.clone());
                            ret_vec = Some(vec);
                        }
                    }
                }
            }
        }
        ret_vec
    }
}

#[derive(Debug)]
struct Directionary {
    // verbs : Vec<String>,
    // nouns : Vec<String>,
    // advs : Vec<String>,
    // adjs : Vec<String>,
    library : std::collections::HashMap<String, Vec<String>>,
    matcher : TagMatcher,
}

impl Directionary {
    pub fn new() -> Directionary {
        use std::collections::HashMap;
        // TODO FINISH THIS
        let tag_matcher = TagMatcher::new()
            .add("Location", vec!("nis","ntcb","ntcf","s","na","ns","ntc","nts","nth","ntch","nto","nit","nt","nsf","nz","f","ntu","nsf",))
            .add("Name", vec!("nr","nba","nrfg","nrf","nrj",))
            .add("Time", vec!("tg","t","Mg"))
            .add("GenericNoun", vec!("gb","vf","nnd","nhd","nmc","nbc","gc","nhm","ng","gg","gi","n","gp","gm","nnt",))
            .add("AllNouns", vec!("vf","nis","ntcb","ntcf","gb","nhd","j","nr","nba","s","nmc","nnd","nrfg","na","ns","ntc","nbc","gc","nts","nth","x","ntch","nto","nit","nrf","nhm","ng","nrt","ntu","gg","gi","nt","nsf","nrj","nz","f","n","gp","gm","tg","nnt","t","Mg",))
            .add("Numeral",vec!("m"))
            .add("Quantifier", vec!("qv","q","qt",))
            .add("IndependentVerb", vec!("vl",))
            .add("TranstiveVerb", vec!("pba","pbei","vyou","vshi","vd","vx","vq","vi","vn",))
            .add("IntranstiveVerb", vec!("vg","uguo","v","vf",))
            .add("AllVerbs", vec!("vyou","uguo","vd","v","vx","vi","pba","pbei","vl","vg","vq","vn","vshi","vf",))
            .add("Adjective", vec!("b","mq","bl","a","z","al","ag","an","œa",))
            .add("Adverb",vec!("b","bl","ad","d","dl","œa","dg",))
            .add("AllPronouns", vec!("rr","rz","ryt","Rg","ry","rys","rzs","rzt","ryv","k",))
            .add("AskWhen", vec!("rzt"))
            .add("When", vec!("ryt"))
            .add("AskHow", vec!("ryv"))
            .add("AskWhere", vec!("rys"))
            .add("Where", vec!("rzs"))
            .add("Who", vec!("rr","rz","Rg",))
            .add("AskWho", vec!("ry"))
            .add("Conjunction", vec!("rzv","u","c","cc",))
            .add("Preposition", vec!("r","uyy","udeng","p","udh",))
            .add("Particle", vec!("uzhe","uls","ule","usuo","ulian","uzhi","ude",))
            .add("AllModals", vec!("y","e","o",))
            .add("PostFixModal", vec!("y"))
            .add("PreFixModal", vec!("e"))
            .add("Onomatopoeia", vec!("o"));
    
        let new_library = HashMap::new();
        
        Directionary {
            library :   new_library,
            matcher :   tag_matcher,
        }
    }
    pub fn from_default(highest_input : Option<u32>, lowest_input : Option<u32>) -> Directionary {
        // TODO 
        use std::fs;
        let highest_frequency : u32 = match highest_input {
            Some(frequency) => frequency,
            None => 2147483647,
        };
        let lowest_frequency : u32 = match lowest_input {
            Some(frequency) => frequency,
            None=> 0,
        };
        let mut directionary = Directionary::new();
        let raw_bytes = fs::read_to_string("resources/ansj_seg-master/default.dic").expect("failed to open directionary file");
        let filterd_bytes = raw_bytes.replace(&['\n'][..], "\t");
        let seperate_words : Vec<&str> = filterd_bytes.split('\t').collect();
        let mut i : u64 = 0;
        let mut last_word : &str = "";
        let mut last_tag : &str = "";
        let mut frequency;
        for tag in seperate_words {
            i = i + 1;
            // 第一列为word, 第二列为tag, 第三列为frequency
            let count = i % 3;
            match count {
                0 => {
                    // println!("tag{:?}, result{:?}", &tag, tag.parse::<u32>());
                    frequency = tag.parse::<u32>().unwrap();
                },    // 第三列
                1 => {
                    last_word = tag;
                    continue;
                },    // 第一列
                _ => {
                    last_tag = tag;
                    continue;
                },    // 第二列
            }
            if (frequency > highest_frequency) || (frequency < lowest_frequency) {
                continue;
            } 
            let mut word = Word::from_literal(last_word);
            word.set_tag(last_tag);
            directionary.add_a_word(&word);
        }
        directionary
    }
    pub fn find_a_word(&self, element : &str, resolver : &mut RandomResolver) -> String {
        let library_vec = self.library.get(element);
        match library_vec {
            Some(_) => {}
            None => {panic!("failed to get element type {}", element);}
        }
        let library_vec = library_vec.unwrap();
        let word = library_vec.get(
            resolver.get_pos(library_vec.len())).unwrap();
        word.clone()
    }
    pub fn add_a_word(&mut self, new_word : &Word) {
        let matcher_result = self.matcher.resolve(new_word.tag.clone());
        // print!("word :{:?}, result :{:?}", &new_word, matcher_result);
        match matcher_result {
            Some(element_vec) => {
                for element in element_vec{
                    let library_result = self.library.get_mut(element.as_str());
                    match library_result {
                        Some(ele_vec) => {
                            ele_vec.push(new_word.literal.clone());
                        }
                        None => {
                            self.library.insert(element.clone(), vec!(new_word.literal.clone()));
                        }
                    }
                }
            }
            None => {}
        }
    }    
}

#[derive(Debug)]
struct RandomResolver {
    rng : oorandom::Rand64,
}

impl RandomResolver {
    pub fn from_seed(seed : u128) -> Self {
        let rng = oorandom::Rand64::new(seed);
        RandomResolver{rng : rng}
    }
    fn resolve_pos(&mut self, vec : &Vec<f64>) -> usize {
        let float_result = self.rng.rand_float();
        let mut sum = 0.0f64;
        let size = vec.len();
        for i in 0..size {
            sum += vec[i];
            if sum > float_result {
                return i;
            }                
        }
        return size;
    }
    pub fn get_pos(&mut self, size : usize) -> usize {
        let float_result = self.rng.rand_float();
        (size as f64 * float_result) as usize        
    }
}
#[derive(Debug, Clone)]
enum SentanceItem {
    Element(String), // element type to be resolve
    Word(String),
    Icon(char),
}

#[derive(Debug, Clone)]
struct SentanceNode {
    item : SentanceItem,
    next : Option<Box<SentanceNode>>,
}

impl SentanceNode {
    pub fn element(element_name : &str) -> SentanceNode {
        SentanceNode {
            item : SentanceItem::Element(String::from(element_name)),
            next : None,
        }
    }
    pub fn icon(icon : char) -> SentanceNode{
        SentanceNode {
            item : SentanceItem::Icon(icon),
            next : None,
        }
    }
    pub fn word(word : &str) -> SentanceNode{
        SentanceNode {
            item : SentanceItem::Word(String::from(word)),
            next : None,
        }
    }
    // insert a node, panic if already has one
    pub fn next(mut self, next : SentanceNode) -> Self {
        match &self.next {
            Some(_) => {
                panic!("node {:?} already have a next node", &self);
            }
            None => {
                self.next = Some(Box::new(next));
            }
        }
        self
    }
    // get the result from element
    fn resolve(&mut self, resolver : &mut RandomResolver, dict : &Directionary) {
        match &self.item {
            SentanceItem::Element(element) => {
                self.item = SentanceItem::Word(
                    dict.find_a_word(element.as_str(), resolver));
            }
            _ => {}
        }
    }
    pub fn resolve_sentance(&mut self, resolver : &mut RandomResolver, dict : &Directionary) {
        self.resolve(resolver, dict);
        match &mut self.next {
            Some(node) => {node.resolve_sentance(resolver, dict);}
            None => {}
        }
    }
    pub fn to_string(self) -> String {
        let mut string = String::new();
        match self.item {
            SentanceItem::Word(word) => string.push_str(word.as_str()),
            SentanceItem::Icon(icon) => string.push(icon),
            _ => {}
        }
        match self.next {
            Some(node) => string.push_str(node.to_string().as_str()),
            None => {}
        }
        string
    }
}

#[derive(Debug)]
struct ComedyWriter {
    possibilitys : Vec<f64>,
    sentances : Vec<Box<SentanceNode>>,
    resolver : RandomResolver,
    directionary : Directionary,
}

impl ComedyWriter {
    pub fn from_seed(random_seed : u128, 
        highest_frequency : Option<u32>, 
        lowest_frequency : Option<u32>) 
        -> ComedyWriter {
        ComedyWriter{
            possibilitys : Vec::new(),
            sentances : Vec::new(),
            resolver : RandomResolver::from_seed(random_seed),
            directionary : Directionary::from_default(
                highest_frequency,
                lowest_frequency),
        }
    }
    pub fn add_node(&mut self, sentance : SentanceNode, posssibility : f64) -> &mut Self {
        self.sentances.push(Box::new(sentance));
        self.possibilitys.push(posssibility);
        self
    }
    fn normalize(&mut self) {
        let mut sum = 0.0f64;
        for chance in &self.possibilitys {
            sum += chance;
        }
        for chance in &mut self.possibilitys {
            *chance /= sum;
        }
    }
    pub fn write(&mut self, number : u32) -> String {
        self.normalize();
        let mut article = String::new();
        for _ in 0..number {
            let pos = self.resolver.resolve_pos(&self.possibilitys);
            let mut sentance = self.sentances[pos].clone();
            sentance.resolve_sentance(&mut self.resolver, &self.directionary);
            let string = sentance.to_string();
            article.push_str(string.as_str());
        }
        article
    }
}

fn main() {
    let mut writer = ComedyWriter::from_seed(
        65536, None, None);
    writer
        .add_node(sentance!(
        [word = "草"][element = "Who"][word="的!"]
    ), 1.8)
        .add_node(sentance!(
        [word = "建议"][element = "Location"][word="的"][element="Name"]
        [element="Time"][word="就"][element="IntranstiveVerb"][icon='.']
    ), 0.2);
    let result = writer.write(50);
    println!("{}",result);
}
