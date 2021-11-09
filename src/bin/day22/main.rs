use log::{debug, info};
use std::{cmp, collections::VecDeque, path::MAIN_SEPARATOR};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const MAGIC_MISSILE_COST: i64 = 53;
    const DRAIN_COST: i64 = 73;
    const SHIELD_COST: i64 = 113;
    const POISON_COST: i64 = 173;
    const RERCHARGE_COST: i64 = 229;

    fn get_cost(&self) -> i64 {
        match self {
            Self::MagicMissile => Self::MAGIC_MISSILE_COST,
            Self::Drain => Self::DRAIN_COST,
            Self::Shield => Self::SHIELD_COST,
            Self::Poison => Self::POISON_COST,
            Self::Recharge => Self::RERCHARGE_COST,
        }
    }

    fn cast(&self, caster: &mut Player, target: &mut Player) -> () {
        caster.mana -= self.get_cost();
        match self {
            Self::MagicMissile => {
                debug!("Player casts {:?}, dealing {} damage.", self, 4);
                target.hit_points -= 4;
            }
            Self::Drain => {
                debug!(
                    "Player casts {:?}, dealing {} damage and healing {} points.",
                    self, 2, 2
                );
                target.hit_points -= 2;
                caster.hit_points += 2;
            }
            Self::Shield => {
                let effect = Effect {
                    effect_type: EffectType::Shielded,
                    duration: EffectType::Shielded.get_initial_duration(),
                };
                debug!("Player casts {:?}, increasing armor by {}.", self, 7);
                *caster = effect.set(&caster);
                caster.effects.push(effect);
            }
            Self::Poison => {
                debug!("Player casts {:?}", self);
                let effect = Effect {
                    effect_type: EffectType::Poisoned,
                    duration: EffectType::Poisoned.get_initial_duration(),
                };
                *target = effect.set(&target);
                target.effects.push(effect);
            }
            Self::Recharge => {
                debug!("Player casts {:?}", self);
                let effect = Effect {
                    effect_type: EffectType::Recharging,
                    duration: EffectType::Recharging.get_initial_duration(),
                };
                *caster = effect.set(&caster);
                caster.effects.push(effect);
            }
        };
    }
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: i64,
    damage: i64,
    armor: i64,
    mana: i64,
    effects: Vec<Effect>,
}

impl Player {
    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }

    fn can_cast(&self, spell: &Spell) -> bool {
        spell.get_cost() <= self.mana
    }
}

fn parse_boss_stats(input: &str) -> Player {
    let parts: Vec<i64> = input
        .lines()
        .map(|line| line.split(" ").last().unwrap().parse::<i64>().unwrap())
        .collect();
    Player {
        hit_points: parts[0],
        damage: parts[1],
        armor: 0,
        mana: 0,
        effects: vec![],
    }
}

#[derive(Debug, Clone)]
struct Effect {
    effect_type: EffectType,
    duration: usize,
}

impl Effect {
    const POISON_DAMAGE_PER_TURN: i64 = 3;
    const RECHARGED_MANA_PER_TURN: i64 = 101;
    const SHIELD_ARMOR_INCREASE: i64 = 7;

    fn apply(&self, target: &Player) -> Player {
        let mut target = target.clone();
        match self.effect_type {
            EffectType::Poisoned => {
                debug!(
                    "Poison deals {} damage; its timer is now {}",
                    Self::POISON_DAMAGE_PER_TURN,
                    self.duration - 1
                );
                target.hit_points -= Self::POISON_DAMAGE_PER_TURN;
            }
            EffectType::Recharging => {
                debug!(
                    "Recharge provides {} mana; its timer is now {}",
                    Self::RECHARGED_MANA_PER_TURN,
                    self.duration - 1
                );
                target.mana += Self::RECHARGED_MANA_PER_TURN;
            }
            EffectType::Shielded => {
                debug!("Shield's timer is now {}", self.duration - 1);
            }
        };
        target
    }

    fn set(&self, target: &Player) -> Player {
        let mut target = target.clone();
        match self.effect_type {
            EffectType::Shielded => target.armor += Self::SHIELD_ARMOR_INCREASE,
            _ => {}
        };
        target
    }

    fn fade(&self, target: &Player) -> Player {
        let mut target = target.clone();
        match self.effect_type {
            EffectType::Shielded => {
                debug!(
                    "Shield wears off, decreasing armor by {}",
                    Self::SHIELD_ARMOR_INCREASE
                );
                target.armor -= Self::SHIELD_ARMOR_INCREASE
            }
            _ => {}
        };
        target
    }
}

#[derive(Debug, Clone)]
enum EffectType {
    Shielded,
    Poisoned,
    Recharging,
}

impl EffectType {
    const SHIELD_DURATION: usize = 6;
    const POISON_DURATION: usize = 6;
    const RECHARGE_DURATION: usize = 5;

    fn get_initial_duration(&self) -> usize {
        match self {
            Self::Shielded => Self::SHIELD_DURATION,
            Self::Poisoned => Self::POISON_DURATION,
            Self::Recharging => Self::RECHARGE_DURATION,
        }
    }
}

fn apply_effects(player: &Player, boss: &Player) -> (Player, Player) {
    let mut player = player.clone();
    let mut boss = boss.clone();

    let mut player_effects = player.effects.clone();
    for effect in &mut player_effects {
        player = effect.apply(&player);
        effect.duration -= 1;
        if effect.duration == 0 {
            player = effect.fade(&player)
        }
    }

    let mut boss_effects = boss.effects.clone();
    for effect in &mut boss_effects {
        boss = effect.apply(&boss);
        effect.duration -= 1;
        if effect.duration == 0 {
            boss = effect.fade(&boss)
        }
    }

    player.effects = player_effects
        .into_iter()
        .filter(|effect| effect.duration > 0)
        .collect_vec();
    boss.effects = boss_effects
        .into_iter()
        .filter(|effect| effect.duration > 0)
        .collect_vec();
    // player.effects = player
    //     .effects
    //     .into_iter()
    //     .map(|effect| Effect {
    //         duration: effect.duration - 1,
    //         ..effect
    //     })
    //     .filter(|effect| effect.duration > 0)
    //     .collect_vec();
    // boss.effects = boss
    //     .effects
    //     .into_iter()
    //     .map(|effect| Effect {
    //         duration: effect.duration - 1,
    //         ..effect
    //     })
    //     .filter(|effect| effect.duration > 0)
    //     .collect_vec();

    (player, boss)
}

fn execute_player_turn(
    player: &Player,
    boss: &Player,
    spell: &Spell,
    hard_mode: bool,
) -> (Player, Player) {
    let mut player = player.clone();
    let mut boss = boss.clone();

    if hard_mode {
        player.hit_points -= 1;
    }
    if player.is_dead() {
        return (player, boss);
    }

    let updated = apply_effects(&player, &boss);
    player = updated.0;
    boss = updated.1;

    if player.is_alive() {
        // cast spell
        let effect_already_applied: bool = match spell {
            Spell::Poison => {
                let count = boss
                    .effects
                    .iter()
                    .filter(|effect| match effect.effect_type {
                        EffectType::Poisoned => true,
                        _ => false,
                    })
                    .count()
                    > 0;
                debug!(
                    "Looking for {:?}. effects = {:?}; (match = {})",
                    spell,
                    boss.effects.clone(),
                    count
                );
                if count {
                    debug!(
                        "Player tried to cast {:?} while the boss was already poisoned!",
                        spell
                    );
                }
                count
            }
            Spell::Shield => {
                let count = player
                    .effects
                    .iter()
                    .filter(|effect| match effect.effect_type {
                        EffectType::Shielded => true,
                        _ => false,
                    })
                    .count()
                    > 0;
                debug!(
                    "Looking for {:?}. effects = {:?}; (match = {})",
                    spell,
                    player.effects.clone(),
                    count
                );
                if count {
                    debug!("Player tried to cast {:?} while shielded already!", spell);
                }
                count
            }
            Spell::Recharge => {
                let count = player
                    .effects
                    .iter()
                    .filter(|effect| match effect.effect_type {
                        EffectType::Recharging => true,
                        _ => false,
                    })
                    .count()
                    > 0;
                debug!(
                    "Looking for {:?}. effects = {:?}; (match = {})",
                    spell,
                    player.effects.clone(),
                    count
                );
                if count {
                    debug!("Player tried to cast {:?} while recharging already!", spell);
                }
                count
            }
            _ => false,
        };
        if player.can_cast(&spell) && !effect_already_applied {
            spell.cast(&mut player, &mut boss);
        } else {
            debug!("Player dies due to lack of mana / casting double effect.");
            player.hit_points = 0;
        }
    }

    (player, boss)
}
fn execute_boss_turn(player: &Player, boss: &Player) -> (Player, Player) {
    let mut player = player.clone();
    let mut boss = boss.clone();

    let updated = apply_effects(&player, &boss);
    player = updated.0;
    boss = updated.1;

    if boss.is_alive() {
        let damage = cmp::max(boss.damage - player.armor, 1);
        debug!("Boss attacks for {} damage.", damage);
        player.hit_points -= damage;
    } else {
        debug!("Boss dies (HP = {})", boss.hit_points);
    }

    (player, boss)
}

fn main() {
    env_logger::init();
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(900, output_part_1);
    assert_eq!(1216, output_part_2);
}

type WeightedSpell = (i64, Spell, Player, Player, Vec<Spell>);

fn part_1(input: &str) -> usize {
    let boss = parse_boss_stats(input);
    let player = Player {
        damage: 0,
        armor: 0,
        mana: 500,
        hit_points: 50,
        effects: vec![],
    };
    let mut min_so_far = 0;
    let mut queue: VecDeque<WeightedSpell> = VecDeque::from_iter([
        (0, Spell::MagicMissile, player.clone(), boss.clone(), vec![]),
        (0, Spell::Drain, player.clone(), boss.clone(), vec![]),
        (0, Spell::Shield, player.clone(), boss.clone(), vec![]),
        (0, Spell::Poison, player.clone(), boss.clone(), vec![]),
        (0, Spell::Recharge, player.clone(), boss.clone(), vec![]),
    ]);
    while queue.len() > 0 {
        let (mut total_spent, spell, player, boss, mut casted_spells) = queue.pop_back().unwrap();
        total_spent += spell.get_cost();
        let (player, boss) = execute_full_turn(&player, &boss, &spell);
        if player.is_dead() {
            continue;
        }
        casted_spells.push(spell);
        if boss.is_dead() {
            debug!(
                "Boss is dead after using {}. Minimum so far = {}",
                total_spent, min_so_far
            );
            if min_so_far == 0 || total_spent < min_so_far {
                debug!(
                    "Player won using {}; spells = {:?}",
                    total_spent, casted_spells
                );
                min_so_far = total_spent;
            }
            continue;
        }

        if min_so_far != 0 && total_spent >= min_so_far {
            continue;
        }
        queue.push_front((
            total_spent,
            Spell::MagicMissile,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Drain,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Shield,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Poison,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Recharge,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
    }
    min_so_far as usize
}

fn execute_full_turn(player: &Player, boss: &Player, spell: &Spell) -> (Player, Player) {
    debug!("\n-- Player turn --");
    debug!(
        "- Player has {} hit points, {} armor, {} mana",
        player.hit_points, player.armor, player.mana
    );
    debug!("- Boss has {} hit points", boss.hit_points);
    let (player, boss) = execute_player_turn(player, boss, spell, false);
    if player.is_dead() || boss.is_dead() {
        (player, boss)
    } else {
        debug!("\n-- Boss turn --");
        debug!(
            "- Player has {} hit points, {} armor, {} mana",
            player.hit_points, player.armor, player.mana
        );
        debug!("- Boss has {} hit points", boss.hit_points);
        execute_boss_turn(&player, &boss)
    }
}

fn part_2(input: &str) -> usize {
    let boss = parse_boss_stats(input);
    let player = Player {
        damage: 0,
        armor: 0,
        mana: 500,
        hit_points: 50,
        effects: vec![],
    };
    let mut min_so_far = 0;
    let mut queue: VecDeque<WeightedSpell> = VecDeque::from_iter([
        (0, Spell::MagicMissile, player.clone(), boss.clone(), vec![]),
        (0, Spell::Drain, player.clone(), boss.clone(), vec![]),
        (0, Spell::Shield, player.clone(), boss.clone(), vec![]),
        (0, Spell::Poison, player.clone(), boss.clone(), vec![]),
        (0, Spell::Recharge, player.clone(), boss.clone(), vec![]),
    ]);
    while queue.len() > 0 {
        let (mut total_spent, spell, player, boss, mut casted_spells) = queue.pop_back().unwrap();
        total_spent += spell.get_cost();
        let (player, boss) = execute_full_turn_v2(&player, &boss, &spell);
        if player.is_dead() {
            continue;
        }
        casted_spells.push(spell);
        if boss.is_dead() {
            debug!(
                "Boss is dead after using {}. Minimum so far = {}",
                total_spent, min_so_far
            );
            if min_so_far == 0 || total_spent < min_so_far {
                debug!(
                    "Player won using {}; spells = {:?}",
                    total_spent, casted_spells
                );
                min_so_far = total_spent;
            }
            continue;
        }

        if min_so_far != 0 && total_spent >= min_so_far {
            continue;
        }
        queue.push_front((
            total_spent,
            Spell::MagicMissile,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Drain,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Shield,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Poison,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
        queue.push_front((
            total_spent,
            Spell::Recharge,
            player.clone(),
            boss.clone(),
            casted_spells.clone(),
        ));
    }
    min_so_far as usize
}

fn execute_full_turn_v2(player: &Player, boss: &Player, spell: &Spell) -> (Player, Player) {
    debug!("\n-- Player turn --");
    debug!(
        "- Player has {} hit points, {} armor, {} mana",
        player.hit_points, player.armor, player.mana
    );
    debug!("- Boss has {} hit points", boss.hit_points);
    let (player, boss) = execute_player_turn(player, boss, spell, true);
    if player.is_dead() || boss.is_dead() {
        (player, boss)
    } else {
        debug!("\n-- Boss turn --");
        debug!(
            "- Player has {} hit points, {} armor, {} mana",
            player.hit_points, player.armor, player.mana
        );
        debug!("- Boss has {} hit points", boss.hit_points);
        execute_boss_turn(&player, &boss)
    }
}
