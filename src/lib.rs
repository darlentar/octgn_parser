use serde::{Deserialize};
extern crate serde_xml_rs;

use std::collections::HashMap;
use serde_xml_rs::{from_str};
use bitvec::vec::{BitVec};
use bitvec::bitvec;

#[derive(Debug, Deserialize)]
pub struct UProperty {
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct UCard {
    pub id: String,
    pub name: String,
    #[serde(rename = "property", default)]
    pub properties: Vec<UProperty>
}

#[derive(Debug, Deserialize)]
pub struct UCards {
    pub card: Vec<UCard>,
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
    Nightmare
}

impl UCards {
    pub fn select_by_name(&self) -> HashMap<String, BitVec> {
        let mut v : HashMap<String, BitVec> = HashMap::new();
        let cards_number = self.card.len();
        for (n, card) in self.card.iter().enumerate() {
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
        let cards_number = self.card.len();
        for (n, card) in self.card.iter().enumerate() {
            for property in &card.properties {
                if property.name == "Type" {
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
                        val => panic!{"{} is card type not known", val},
                    };
                    v.entry(card_type)
                        .and_modify(|v| v.set(n, true))
                        .or_insert_with(|| {let mut m = bitvec![0; cards_number]; m.set(n, true); m});
                }
            }
        }
        v
    }
}

#[derive(Debug, Deserialize)]
pub struct USet {
    pub version: String,
    #[serde(rename = "gameVersion", default)]
    pub game_version: String,
    #[serde(rename = "gameId", default)]
    pub game_id: String,
    pub id: String,
    pub name: String,

    pub cards: UCards,
}

pub fn parse_octgn_set(s: &str) -> USet {
    let i = s.find('\n').unwrap();
    from_str(s.get(i..).unwrap()).unwrap()
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
        assert_eq!(parse_octgn_set(default()).cards.card[0].properties[2].name, "Type");
    }

    #[test]
    fn set_select_by_name_start_with_f() {
        let cards = &parse_octgn_set(default()).cards;
        let selectors = cards.select_by_name();
        assert_eq!(selectors["f"], bitvec![1, 1, 0])
    }

    #[test]
    fn set_select_by_name_start_with_r() {
        let cards = &parse_octgn_set(default()).cards;
        let selectors = cards.select_by_name();
        assert_eq!(selectors["r"], bitvec![0, 0, 1])
    }

    #[test]
    fn set_select_by_name_start_with_fe() {
        let cards = &parse_octgn_set(default()).cards;
        let selectors = cards.select_by_name();
        assert_eq!(selectors["fe"], bitvec![0, 1, 0])
    }

    #[test]
    fn set_select_by_name_contains_wes() {
        let cards = &parse_octgn_set(default()).cards;
        let selectors = cards.select_by_name();
        assert_eq!(selectors["wes"], bitvec![0, 0, 1])
    }

    #[test]
    fn set_select_by_type_hero() {
        let cards = &parse_octgn_set(default()).cards;
        let selectors = cards.select_by_type();
        assert_eq!(selectors[&CardType::Hero], bitvec![1, 0, 0])
    }

    #[test]
    fn set_select_by_type_attachment() {
        let cards = &parse_octgn_set(default()).cards;
        let selectors = cards.select_by_type();
        assert_eq!(selectors[&CardType::Attachment], bitvec![0, 1, 0])
    }
}