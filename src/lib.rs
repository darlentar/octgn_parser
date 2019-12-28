use serde::{Deserialize, Serialize};

use std::str::FromStr;
use serde_xml_rs::{from_str};
use bitvec::prelude::*;

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

impl HeroCard {
    fn from_ucard(card: &UCard) -> Result<Self, String> {
        let mut hero_card = HeroCard::default();
        let mut setted = bitvec![0; 11];
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
                // TODO: REIMPLEMENT THIS
                // "Set" => {
                //     setted.set(11, true);
                //     hero_card.set = String::from(&property.value)
                // },
                x => panic!("Cannot parse {} for hero card.", x)
            }
        }
        if setted.all() {
            Ok(hero_card)
        } else {
            Err(String::from(format!("Error {:?} in card hero parsing", setted)))
        }
    }
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
    fn hero_card_from_ucard() {
        let cards = &parse_octgn_set(default()).cards;
        let hero_card = HeroCard::from_ucard(&cards.card[0]).unwrap();
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
        let mut hero_card_without_willpower = cards.card[0].clone();
        hero_card_without_willpower.properties.remove(0);
        assert!(HeroCard::from_ucard(&cards.card[1]).is_err());
    }

    #[test]
    fn card_set_from_uset() {
        let card_set = &parse_octgn_set(default());
        assert_eq!(card_set.cards.card[0].properties[0].value, "81");
    }

    // TDOO: REIMPLEMENT THIS FUNCTIONALITY
    //#[test]
    //fn add_card_set() {
    //    let mut card_set1 = parse_octgn_set(default());
    //    let mut card_set2 = parse_octgn_set(default());
    //    card_set1.append(&mut card_set2);
    //    assert_eq!(card_set1.cards.card.len(), 6);
    //}
}
