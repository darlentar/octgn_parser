use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::str::FromStr;
use serde_xml_rs::{from_str};
use bitvec::vec::{BitVec};
use bitvec::bitvec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UProperty {
    pub name: String,
    pub value: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UCard {
    pub id: String,
    pub name: String,
    #[serde(rename = "property", default)]
    pub properties: Vec<UProperty>
}

#[derive(Debug, Deserialize)]
struct UCards {
    card: Vec<UCard>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum CardType {
    Attachment,
    Hero,
    SideQuest,
    Event,
    Ally,
    Objective,
    Enemy,
    Location,
    Treachery,
    Quest,
    Rules,
    Nightmare,
    Treasure,
    Campaign,
    ObjectiveAlly,
    ShipEnemy,
    Contract,
}

impl Default for CardType {
    fn default() -> Self {CardType::Attachment}
}

#[derive(Debug, Deserialize)]
struct USet {
    version: String,
    #[serde(rename = "gameVersion", default)]
    game_version: String,
    #[serde(rename = "gameId", default)]
    game_id: String,
    id: String,
    name: String,

    cards: UCards,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Sphere {
    Spirit,
    Lore,
    Leadership,
    Tactics,
}

impl FromStr for Sphere {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Spirit" => Ok(Sphere::Spirit),
            "Lore" => Ok(Sphere::Lore),
            "Leadership" => Ok(Sphere::Leadership),
            "Tactics" => Ok(Sphere::Tactics),
            x => Result::Err(format!("{} is not a shpere", x)),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {Sphere::Spirit}
}

#[derive(Default)]
pub struct HeroCard {
    pub card_number: u8,
    pub quantity: u8,
    pub card_type: CardType,
    pub sphere: Sphere,
    pub traits: Vec<String>,
    pub cost: u8,
    pub willpower: u8,
    pub attack: u8,
    pub defense: u8,
    pub health: u8,
    pub text: String,
    pub set: String,
}

pub struct CardSet {
    pub cards: Vec<UCard>,
}

impl CardSet {
    pub fn empty() -> Self {
        CardSet{cards:vec!{}}
    }

    pub fn append(&mut self, other: &mut Self) {
        self.cards.append(&mut other.cards);
    }

    fn from_uset(set: &USet) -> Self {
        let mut cards = Vec::with_capacity(set.cards.card.len());
        for card in &set.cards.card {
            let mut new_card = card.clone();
            new_card.properties.push(UProperty{name: String::from("Set"), value: set.name.clone()});
            cards.push(new_card);
        }
        CardSet{cards}
    }

    pub fn select_by_name(&self) -> HashMap<String, BitVec> {
        let mut v : HashMap<String, BitVec> = HashMap::new();
        let cards_number = self.cards.len();
        for (n, card) in self.cards.iter().enumerate() {
            for name in card.name.split(' ') {
                let name = name.to_lowercase().replace("ú", "u");
                let name = name.to_lowercase().replace("’", "'");
                let name = name.to_lowercase().replace("í", "i");
                let name = name.to_lowercase().replace("û", "u");
                let name = name.to_lowercase().replace("î", "i");
                let name = name.to_lowercase().replace("ó", "o");
                let name = name.to_lowercase().replace("é", "e");
                let name = name.to_lowercase().replace("â", "a");
                let name = name.to_lowercase().replace("“", "");
                let name = name.to_lowercase().replace("ë", "e");
                let name = name.to_lowercase().replace("”", "");
                let name = name.to_lowercase().replace("á", "a");
                let name = name.to_lowercase().replace("\u{a0}", "eo");
                let name = name.to_lowercase().replace("ä", "a");
                let name = name.to_lowercase().replace("\u{301}", "");
                let name = name.to_lowercase().replace("ö", "o");
                let name = name.to_lowercase().replace("\u{302}", "u");
                let i = name.len();
                for j in 0..i {
                    let card_name_start = name[0..i-j].to_string();
                    v.entry(card_name_start)
                        .and_modify(|v| v.set(n, true))
                        .or_insert_with(|| {let mut m = bitvec![0; cards_number]; m.set(n, true); m});
                }
            }
        }
        v
    }

    pub fn select_by_type(&self) -> HashMap<CardType, BitVec> {
        let mut v : HashMap<CardType, BitVec> = HashMap::new();
        let cards_number = self.cards.len();
        for (n, card) in self.cards.iter().enumerate() {
            for property in &card.properties {
                if property.name == "Type" {
                    if property.value.as_str() != "Internal" {
                        let card_type = match property.value.as_str() {
                            "Hero" => CardType::Hero,
                            "Attachment" => CardType::Attachment,
                            "Side Quest" => CardType::SideQuest,
                            "Event" => CardType::Event,
                            "Ally" => CardType::Ally,
                            "Objective" => CardType::Objective,
                            "Enemy" => CardType::Enemy,
                            "Location" => CardType::Location,
                            "Treachery" => CardType::Treachery,
                            "Quest" => CardType::Quest,
                            "Rules" => CardType::Rules,
                            "Nightmare" => CardType::Nightmare,
                            "Treasure" => CardType::Treasure,
                            "Campaign" => CardType::Campaign,
                            "Objective Ally" => CardType::ObjectiveAlly,
                            "Ship-Enemy" => CardType::ShipEnemy,
                            "Contract" => CardType::Contract,
                            val => panic!{"{} is card type not known", val},
                        };
                        v.entry(card_type)
                            .and_modify(|v| v.set(n, true))
                            .or_insert_with(|| {let mut m = bitvec![0; cards_number]; m.set(n, true); m});
                        }
                }
            }
        }
        v
    }
}

impl HeroCard {
    fn from_ucard(card: &UCard) -> Result<Self, String> {
        let mut hero_card = HeroCard::default();
        let mut setted = bitvec![0; 12];
        for property in &card.properties {
            match property.name.as_str() {
                "Card Number" => {
                    setted.set(0, true);
                    hero_card.card_number = u8::from_str(property.value.as_str()).unwrap()
                },
                "Quantity" => {
                    setted.set(1, true);
                    hero_card.quantity = u8::from_str(property.value.as_str()).unwrap()
                },
                "Type" => {
                    setted.set(2, true);
                    hero_card.card_type = CardType::Hero
                },
                "Traits" => {
                    setted.set(3, true);
                    let traits = property.value.split(' ')
                                    .into_iter()
                                    .map(|x| String::from(&x[..x.len()-1]))
                                    .collect();
                    hero_card.traits = traits;
                },
                "Cost" => {
                    setted.set(4, true);
                    hero_card.cost = u8::from_str(property.value.as_str()).unwrap()
                },
                "Willpower" => {
                    setted.set(5, true);
                    hero_card.willpower = u8::from_str(property.value.as_str()).unwrap()
                },
                "Attack" => {
                    setted.set(6, true);
                    hero_card.attack = u8::from_str(property.value.as_str()).unwrap()
                },
                "Defense" => {
                    setted.set(7, true);
                    hero_card.defense = u8::from_str(property.value.as_str()).unwrap()
                },
                "Health" => {
                    setted.set(8, true);
                    hero_card.health = u8::from_str(property.value.as_str()).unwrap()
                },
                "Sphere" => {
                    setted.set(9, true);
                    hero_card.sphere = Sphere::from_str(property.value.as_str()).unwrap()
                },
                "Text" => {
                    setted.set(10, true);
                    hero_card.text = String::from(&property.value)
                },
                "Set" => {
                    setted.set(11, true);
                    hero_card.set = String::from(&property.value)
                },
                x => panic!("Cannot parse {} for hero card.", x)
            }
        }
        if setted.all() {
            Ok(hero_card)
        } else {
            Err(String::from(format!("Error in card hero parsing")))
        }
    }
}

pub fn parse_octgn_set(s: &str) -> CardSet {
    let i = s.find('\n').unwrap();
    let uset = from_str(s.get(i..).unwrap()).unwrap();
    CardSet::from_uset(&uset)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default() -> &'static str {
        r##"<?xml version="1.0" encoding="UTF-8" standalone="true"?>
            <set version="1.0.0" gameVersion="2.3.6.0" gameId="a21af4e8-be4b-4cda-a6b6-534f9717391f" id="4ce33205-d863-4f24-a0c1-c03104cf1091" name="The Black Serpent" xmlns:noNamespaceSchemaLocation="CardSet.xsd">
                <cards>
                    <card id="d87344c3-d234-44c8-89b1-249f5c88bfab" name="Fastred">
                        <property name="Card Number" value="81"/>
                        <property name="Quantity" value="1"/>
                        <property name="Type" value="Hero"/>
                        <property name="Sphere" value="Spirit"/>
                        <property name="Traits" value="Rohan. Warrior."/>
                        <property name="Cost" value="9"/>
                        <property name="Willpower" value="1"/>
                        <property name="Attack" value="2"/>
                        <property name="Defense" value="3"/>
                        <property name="Health" value="3"/>
                        <property name="Text" value="Response: After Fastred defends an enemy attack, return that enemy to the staging area to reduce your threat by 2. (Limit once per phase.)"/>
                    </card>
                    <card id="168c62d8-bb38-4258-9d2b-6b46aef6c700" name="Fearless Scout">
                        <property name="Card Number" value="86"/>
                        <property name="Quantity" value="3"/>
                        <property name="Type" value="Attachment"/>
                        <property name="Sphere" value="Spirit"/>
                        <property name="Traits" value="Skill."/>
                        <property name="Cost" value="1"/>
                        <property name="Text" value="Attach to a hero. Limit 1 per hero. Attached hero gains the Scout trait. Response: After you play Fearless Scout from your hand, draw a card."/>
                    </card>
                    <card id="97d09901-724d-4932-ac51-59df1ef1dbec" name="Rally the West" size="PlayerQuestCard">
                        <property name="Card Number" value="87"/>
                        <property name="Quantity" value="3"/>
                        <property name="Type" value="Side Quest"/>
                        <property name="Sphere" value="Spirit"/>
                        <property name="Cost" value="1"/>
                        <property name="Victory Points" value="1"/>
                        <property name="Text" value="“Foes and fire are before you, and your homes far behind. Yet, though you fight upon an alien field, the glory that you reap there shall be your own forever.” -Theoden, The Return of the King Limit 1 copy of Rally the West in the victory display. While this side quest is in the victory display, each hero gets +1 Ò."/>
                    </card>
                </cards>
            </set>"##
    }

    #[test]
    fn parse_octgn_set_works() {
        assert_eq!(parse_octgn_set(default()).cards[0].properties[2].name, "Type");
    }

    #[test]
    fn set_select_by_name_start_with_f() {
        let cards = &parse_octgn_set(default());
        let selectors = cards.select_by_name();
        assert_eq!(selectors["f"], bitvec![1, 1, 0])
    }

    #[test]
    fn set_select_by_name_start_with_r() {
        let cards = &parse_octgn_set(default());
        let selectors = cards.select_by_name();
        assert_eq!(selectors["r"], bitvec![0, 0, 1])
    }

    #[test]
    fn set_select_by_name_start_with_fe() {
        let cards = &parse_octgn_set(default());
        let selectors = cards.select_by_name();
        assert_eq!(selectors["fe"], bitvec![0, 1, 0])
    }

    #[test]
    fn set_select_by_name_contains_wes() {
        let cards = &parse_octgn_set(default());
        let selectors = cards.select_by_name();
        assert_eq!(selectors["wes"], bitvec![0, 0, 1])
    }

    #[test]
    fn set_select_by_type_hero() {
        let cards = &parse_octgn_set(default());
        let selectors = cards.select_by_type();
        assert_eq!(selectors[&CardType::Hero], bitvec![1, 0, 0])
    }

    #[test]
    fn set_select_by_type_attachment() {
        let cards = &parse_octgn_set(default());
        let selectors = cards.select_by_type();
        assert_eq!(selectors[&CardType::Attachment], bitvec![0, 1, 0])
    }

    #[test]
    fn hero_card_from_ucard() {
        let cards = &parse_octgn_set(default()).cards;
        let hero_card = HeroCard::from_ucard(&cards[0]).unwrap();
        assert_eq!(hero_card.card_number, 81);
        assert_eq!(hero_card.quantity, 1);
        assert_eq!(hero_card.card_type, CardType::Hero);
        assert_eq!(hero_card.sphere, Sphere::Spirit);
        assert_eq!(hero_card.traits, vec!["Rohan", "Warrior"]);
        assert_eq!(hero_card.cost, 9);
        assert_eq!(hero_card.willpower, 1);
        assert_eq!(hero_card.attack, 2);
        assert_eq!(hero_card.defense, 3);
        assert_eq!(hero_card.health, 3);
        assert!(hero_card.text.starts_with("Response: After Fastred def"));
    }

    #[test]
    fn hero_card_from_ucard_without_willpower() {
        let cards = &parse_octgn_set(default()).cards;
        let mut hero_card_without_willpower = cards[0].clone();
        hero_card_without_willpower.properties.remove(0);
        assert!(HeroCard::from_ucard(&cards[1]).is_err());
    }

    #[test]
    fn card_set_from_uset() {
        let card_set = &parse_octgn_set(default());
        assert_eq!(card_set.cards[0].properties[11].value, "The Black Serpent");
    }

    #[test]
    fn add_card_set() {
        let mut card_set1 = parse_octgn_set(default());
        let mut card_set2 = parse_octgn_set(default());
        card_set1.append(&mut card_set2);
        assert_eq!(card_set1.cards.len(), 6);
    }
}