use std::collections::HashMap;
use serialize::{json, Decodable};
use std::io::{File, Open, Read};
use std::rand::{ mod, Rng };

#[deriving(Decodable)]
pub struct Insults {
    failed_retorts: Vec<String>,

    monkey_island1: HashMap<String, String>,
    sword_master: HashMap<String, String>,

    monkey_island3: HashMap<String, String>,
    captain_rottingham: HashMap<String, String>,

    monkey_island4: HashMap<String, String>,
}

impl Insults {
    pub fn new(location: &str) -> Insults {
        let p = Path::new(location);
        let mut file = match File::open_mode(&p, Open, Read) {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e)
        };

        let decoded: String = match file.read_to_string() {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e)
        };

        let json_object = match json::from_str(decoded[]) {
            Ok(x) => x,
            Err(e) => panic!("json error: {}", e)
        };
        let mut decoder = json::Decoder::new(json_object);

        match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => panic!("Decoding error: {}", e)
        }
    }

    pub fn failed_retorts<'a>(&'a self) -> &'a [String] {
        self.failed_retorts[]
    }

    pub fn rand_failed_retort<'a, R: Rng>(&'a self, rng: &mut R) -> &'a str {
        let retorts = self.failed_retorts();
        rand::sample(rng, retorts.iter(), 1)[0][]
    }

    /// Correctly retort to insult, if there is one.
    pub fn retort<'a>(&'a self, insult: &str) -> Option<&'a str> {
        self.sword_master_retort(insult).or_else(||
            self.mi1_retort(insult).or_else(||
                self.mi3_retort(insult).or_else(||
                    self.captain_rottingham_retort(insult).or_else(||
                        self.mi4_retort(insult)))))
    }

    /// Correctly retort to an insult, with fallback to a random failed retort.
    pub fn retort_or_rand_fail<'a, R: Rng>(&'a self, insult: &str, rng: &mut R) -> &'a str {
        match self.retort(insult) {
            Some(x) => x,
            None => self.rand_failed_retort(rng),
        }
    }

    /// Retort to an insult from Monkey Island 1.
    ///
    /// Will return a custom retort if you're using an insult from the Sword Master.
    pub fn mi1_retort<'a>(&'a self, insult: &str) -> Option<&'a str> {
        match self.sword_master_retort(insult) {
            Some(_) => Some("That's not fair, you're using the Sword Master's insults!"),
            None => self.retort_from(insult, &self.monkey_island1)
        }
    }

    /// Retort to a sword master insult.
    pub fn sword_master_retort<'a>(&'a self, insult: &str) -> Option<&'a str> {
        self.retort_from(insult, &self.sword_master)
    }

    /// Retort to an insult from Monkey Island 3.
    ///
    /// Will return a custom retort if you're using an insult from Captain Rottingham.
    pub fn mi3_retort<'a>(&'a self, insult: &str) -> Option<&'a str> {
        match self.captain_rottingham_retort(insult) {
            Some(_) => Some("That's not fair, you're using Captain Rottingham's insults!"),
            None => self.retort_from(insult, &self.monkey_island3)
        }
    }

    /// Retort to a sword master insult.
    pub fn captain_rottingham_retort<'a>(&'a self, insult: &str) -> Option<&'a str> {
        self.retort_from(insult, &self.captain_rottingham)
    }

    /// Retort to an insult from Monkey Island 4.
    pub fn mi4_retort<'a>(&'a self, insult: &str) -> Option<&'a str> {
        self.retort_from(insult, &self.monkey_island4)
    }

    fn retort_from<'a>(&self, insult: &str, map: &'a HashMap<String, String>) -> Option<&'a str> {
        match map.get(insult) {
            Some(x) => Some(x[]),
            None => None
        }
    }

    pub fn is_retort(&self, insult: &str, retort: &str) -> bool {
        match self.retort(insult) {
            Some(x) => x == retort,
            None => false
        }
    }

    /// Get all insults.
    pub fn insults(&self) -> Vec<&String> {
        let mut res = self.mi1_insults();
        res.push_all(self.sword_master_insults()[]);
        res.push_all(self.monkey_island3_insults()[]);
        res.push_all(self.captain_rottingham_insults()[]);
        res.push_all(self.monkey_island4_insults()[]);
        res
    }

    pub fn rand_insult<R: Rng>(&self, rng: &mut R) -> &String {
        let all = self.insults();
        rand::sample(rng, all.into_iter(), 1)[0]
    }

    pub fn mi1_insults(&self) -> Vec<&String> {
        self.monkey_island1.keys().collect()
    }

    pub fn sword_master_insults(&self) -> Vec<&String> {
        self.sword_master.keys().collect()
    }

    pub fn monkey_island3_insults(&self) -> Vec<&String> {
        self.monkey_island3.keys().collect()
    }

    pub fn captain_rottingham_insults(&self) -> Vec<&String> {
        self.captain_rottingham.keys().collect()
    }

    pub fn monkey_island4_insults(&self) -> Vec<&String> {
        self.monkey_island4.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let mi = Insults::new("insults.json");

        assert_eq!(mi.mi1_retort("You're lazy!"), None);
        assert_eq!(mi.mi1_retort("Have you stopped wearing diapers yet?"),
            Some("Why, did you want to borrow one?"));

        assert_eq!(mi.mi3_retort("You're lazy!"), None);
        assert_eq!(mi.mi3_retort("Would you like to be buried, or cremated?"),
            Some("With you around, I'd rather be fumigated."));

        assert_eq!(mi.mi4_retort("You're lazy!"), None);
        assert_eq!(mi.mi4_retort("Hey, look over there!"),
            Some("Yeah, yeah I know: it's a three headed monkey."));

        assert_eq!(mi.retort("You're lazy!"), None);
        assert_eq!(mi.retort("Have you stopped wearing diapers yet?"),
            Some("Why, did you want to borrow one?"));

        assert_eq!(mi.retort("You're lazy!"), None);
        assert_eq!(mi.retort("Would you like to be buried, or cremated?"),
            Some("With you around, I'd rather be fumigated."));

        assert_eq!(mi.retort("You're lazy!"), None);
        assert_eq!(mi.retort("Hey, look over there!"),
            Some("Yeah, yeah I know: it's a three headed monkey."));

        // Check insults/retorts for all regular ones
        for insult in mi.insults().iter() {
            match mi.retort(insult[]) {
                Some(retort) => assert!(mi.is_retort(insult[], retort)),
                None => {
                    println!("`{}` is missing a retort!", insult);
                    assert!(false);
                },
            }
        }

        for insult in mi.sword_master_insults().iter() {
            match mi.mi1_retort(insult[]) {
                Some(retort) => assert!(retort ==
                        "That's not fair, you're using the Sword Master's insults!"),
                None => {
                    println!("Sword master: `{}` is missing a retort!", insult);
                    assert!(false);
                },
            }
        }

        for insult in mi.captain_rottingham_insults().iter() {
            match mi.mi3_retort(insult[]) {
                Some(retort) => assert!(retort ==
                        "That's not fair, you're using Captain Rottingham's insults!"),
                None => {
                    println!("Captain rottingham: `{}` is missing a retort!", insult);
                    assert!(false);
                },
            }
        }
    }
}

