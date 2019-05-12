use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_xml_rs;
use structopt::StructOpt;

#[derive(Deserialize, Debug, Clone)]
struct Item {
    #[serde(rename = "uiIndex")]
    index: u32,
    #[serde(rename = "szItemName")]
    name: String,
    #[serde(rename = "szLongItemName")]
    long_name: String,
    #[serde(rename = "szItemDesc")]
    description: String,
    #[serde(rename = "usItemClass")]
    class: u32,
    #[serde(rename = "AttachmentClass")]
    attachment_class: Option<u32>,
    #[serde(rename = "AvailableAttachmentPoint")]
    available_attachment_points: Option<Vec<u64>>,
    #[serde(rename = "ubWeight")]
    weight: Option<f32>,
    #[serde(rename = "ItemSize")]
    size: Option<u32>,
    #[serde(rename = "usPrice")]
    price: Option<u32>,
    #[serde(rename = "ubCoolness")]
    coolness: Option<u32>,
    #[serde(rename = "bReliability")]
    reliability: Option<i32>,
    #[serde(rename = "bRepairEase")]
    repair_ease: Option<i32>,
    #[serde(rename = "Damageable")]
    damageable: Option<u32>,
    #[serde(rename = "Repairable")]
    repairable: Option<u32>,
    #[serde(rename = "WaterDamages")]
    water_damages: Option<u32>,
    #[serde(rename = "Metal")]
    metal: Option<u32>,
    #[serde(rename = "Sinks")]
    sinks: Option<u32>,
    #[serde(rename = "ShowStatus")]
    show_status: Option<u32>,
    #[serde(rename = "BR_NewInventory")]
    br_new_inventory: Option<u32>,
    #[serde(rename = "BR_UsedInventory")]
    br_used_inventory: Option<u32>,
    #[serde(rename = "DefaultAttachment")]
    default_attachments: Option<Vec<u32>>,
    #[serde(rename = "DamageChance")]
    damage_chance: Option<u32>,
    #[serde(rename = "DirtIncreaseFactor")]
    dirty_increase_factor: Option<f32>,
    #[serde(rename = "NotBuyable")]
    not_buyable: Option<u32>,
    #[serde(rename = "TwoHanded")]
    two_handed: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct ItemList {
    #[serde(rename = "ITEM")]
    items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone)]
struct Ammo {
    #[serde(rename = "uiIndex")]
    index: u32,
    #[serde(rename = "AmmoCaliber")]
    caliber_name: String,
}

#[derive(Deserialize, Debug)]
struct AmmoList {
    #[serde(rename = "AMMO")]
    ammo_types: Vec<Ammo>,
}

#[derive(Deserialize, Debug)]
struct Weapon {
    #[serde(rename = "uiIndex")]
    index: u32,
    #[serde(rename = "szWeaponName")]
    weapon_name: String,
    #[serde(rename = "bAccuracy")]
    b_accuracy: i32,
    #[serde(rename = "ubBurstPenalty")]
    burst_penalty: u32,
    #[serde(rename = "AutoPenalty")]
    auto_penalty: u32,
    #[serde(rename = "MaxDistForMessyDeath")]
    max_dist_for_messy_death: u32,
    #[serde(rename = "ubWeaponClass")]
    weapon_class: Option<u32>,
    #[serde(rename = "ubWeaponType")]
    weapon_type: Option<u32>,
    #[serde(rename = "ubCalibre")]
    caliber: Option<u32>,
    #[serde(rename = "ubReadyTime")]
    ready_time: Option<u32>,
    #[serde(rename = "ubShotsPer4Turns")]
    shots_per_4_turns: Option<f32>,
    #[serde(rename = "bBurstAP")]
    burst_ap: Option<u32>,
    #[serde(rename = "ubBulletSpeed")]
    bullet_speed: Option<u32>,
    #[serde(rename = "ubImpact")]
    impact: Option<u32>,
    #[serde(rename = "ubDeadliness")]
    deadliness: Option<u32>,
    #[serde(rename = "ubMagSize")]
    mag_size: Option<u32>,
    #[serde(rename = "usRange")]
    range: Option<u16>,
    #[serde(rename = "usReloadDelay")]
    reload_delay: Option<u16>,
    #[serde(rename = "ubAttackVolume")]
    attack_volume: Option<u32>,
    #[serde(rename = "ubHitVolume")]
    hit_volume: Option<u32>,
    #[serde(rename = "sSound")]
    sound: Option<i16>,
    #[serde(rename = "sReloadSound")]
    reload_sound: Option<i16>,
    #[serde(rename = "sLocknLoadSound")]
    lock_n_load_sound: Option<i16>,
    #[serde(rename = "SilencedSound")]
    silenced_sound: Option<u32>,
    #[serde(rename = "APsToReload")]
    aps_to_reload: Option<u32>,
    #[serde(rename = "APsToReloadManually")]
    aps_to_reload_manually: Option<u32>,
    #[serde(rename = "SwapClips")]
    swap_clips: Option<u32>,
    #[serde(rename = "ManualReloadSound")]
    manual_reload_sound: Option<u32>,
    #[serde(rename = "nAccuracy")]
    n_accuracy: Option<i32>,
    #[serde(rename = "ubAimLevels")]
    aiming_levels: Option<u32>,
    #[serde(rename = "Handling")]
    handling: Option<u32>,
    #[serde(rename = "bAutofireShotsPerFiveAP")]
    autofire_shots_per_five_ap: Option<u32>,
    #[serde(rename = "ubShotsPerBurst")]
    shots_per_burst: Option<u32>,
    #[serde(rename = "bRecoilX")]
    recoil_x: Option<f32>,
    #[serde(rename = "bRecoilY")]
    recoil_y: Option<f32>,
    #[serde(rename = "usOverheatingJamThreshold")]
    overheating_jam_threshold: Option<u32>,
    #[serde(rename = "usOverheatingDamageThreshold")]
    overheating_damage_threshold: Option<u32>,
    #[serde(rename = "usOverheatingSingleShotTemperature")]
    overheating_single_shot_temperature: Option<u32>,
    #[serde(rename = "NoSemiAuto")]
    no_semi_auto: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct WeaponList {
    #[serde(rename = "WEAPON")]
    weapons: Vec<Weapon>,
}

#[derive(Serialize, Debug)]
struct NcthWeapon {
    #[serde(rename = "Index")]
    index: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Long Name")]
    long_name: String,
    #[serde(rename = "Type")]
    weapon_type: WeaponType,
    #[serde(rename = "Ammo")]
    caliber: String,
    #[serde(rename = "Mag")]
    mag_size: u32,
    #[serde(rename = "Rng")]
    range: f32,
    #[serde(rename = "Acc")]
    accuracy: i32,
    #[serde(rename = "Aim")]
    aiming_levels: u32,
    #[serde(rename = "Dmg")]
    damage: u32,
    #[serde(rename = "Handling")]
    handling: u32,
    #[serde(rename = "rdy")]
    aps_to_ready: u32,
    #[serde(rename = "att")]
    aps_to_attack: Option<u32>,
    #[serde(rename = "bur")]
    aps_to_burst: Option<u32>,
    #[serde(rename = "aut")]
    aps_to_auto: Option<u32>,
    #[serde(rename = "lod")]
    aps_to_reload: u32,
    #[serde(rename = "rrd")]
    aps_to_reload_manually: Option<u32>,
    #[serde(rename = "Hands")]
    hands: u32,
    #[serde(rename = "Loud")]
    loudness: u32,
    #[serde(rename = "Rely")]
    reliability: i32,
    #[serde(rename = "Rep")]
    repair_ease: i32,
    #[serde(rename = "Burst shots")]
    shots_per_burst: Option<u32>,
    #[serde(rename = "Autofire per 5 AP")]
    autofire_shots_per_five_ap: Option<u32>,
    #[serde(rename = "Recoil X")]
    recoil_x: Option<f32>,
    #[serde(rename = "Recoil Y")]
    recoil_y: Option<f32>,
    #[serde(rename = "Recoil Total")]
    recoil_display: Option<f32>,
    #[serde(rename = "Weight")]
    weight: f32,
    #[serde(rename = "Coolness")]
    coolness: u32,
    #[serde(rename = "Buyable")]
    buyable: bool,
    #[serde(rename = "Price")]
    price: Option<u32>,
}

#[repr(u32)]
#[derive(Serialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum WeaponType {
    Pistol,
    MP,
    SMG,
    Rifle,
    SR,
    AR,
    LMG,
    Shotgun,
}

impl TryFrom<u32> for WeaponType {
    type Error = ();
    fn try_from(wt: u32) -> Result<Self, Self::Error> {
        match wt {
            1 => Ok(WeaponType::Pistol),
            2 => Ok(WeaponType::MP),
            3 => Ok(WeaponType::SMG),
            4 => Ok(WeaponType::Rifle),
            5 => Ok(WeaponType::SR),
            6 => Ok(WeaponType::AR),
            7 => Ok(WeaponType::LMG),
            8 => Ok(WeaponType::Shotgun),
            _ => Err(()),
        }
    }
}

impl TryFrom<(Weapon, Item, Ammo)> for NcthWeapon {
    type Error = ();
    fn try_from((weapon, item, ammo): (Weapon, Item, Ammo)) -> Result<Self, Self::Error> {
        if weapon.index != item.index || weapon.caliber.unwrap_or(0) != ammo.index {
            return Err(());
        }
        match (weapon, item, ammo) {
            (
                Weapon {
                    index,
                    n_accuracy: Some(accuracy),
                    impact: Some(damage),
                    range: Some(range),
                    handling: Some(handling),
                    aiming_levels: Some(aiming_levels),
                    attack_volume: Some(loudness),
                    ready_time: Some(aps_to_ready),
                    shots_per_4_turns: Some(shots_per_4_turns),
                    mag_size: Some(mag_size),
                    aps_to_reload: Some(aps_to_reload),
                    weapon_type: Some(weapon_type),
                    shots_per_burst,
                    autofire_shots_per_five_ap,
                    aps_to_reload_manually,
                    recoil_x,
                    recoil_y,
                    no_semi_auto,
                    burst_ap,
                    ..
                },
                Item {
                    name,
                    long_name,
                    not_buyable,
                    two_handed,
                    reliability,
                    repair_ease,
                    price,
                    weight: Some(weight),
                    coolness: Some(coolness),
                    ..
                },
                Ammo {
                    caliber_name: caliber,
                    ..
                },
            ) => {
                let weapon_type = WeaponType::try_from(weapon_type)?;
                let basic_attack = {
                    let top = 8 * 80 * 100;
                    let bottom = ((100.0 + 80.0) * shots_per_4_turns) as u32;
                    (top + bottom / 2) / bottom
                };
                let buyable = !option_u32_to_bool(not_buyable);
                let hands = if option_u32_to_bool(two_handed) { 2 } else { 1 };
                let single_shot = !option_u32_to_bool(no_semi_auto);
                let aps_to_attack = if single_shot {
                    Some(basic_attack)
                } else {
                    None
                };
                let aps_to_burst = match (shots_per_burst, burst_ap) {
                    (Some(shots), Some(burst_ap)) if shots > 0 => {
                        let calc = (burst_ap * 80 + 99) / 100;
                        let half = (burst_ap + 1) / 2;
                        let max = std::cmp::max(calc, half);
                        let aps = basic_attack + max;
                        Some(aps)
                    }
                    _ => None,
                };
                let aps_to_auto = match autofire_shots_per_five_ap {
                    Some(auto_per_five) if auto_per_five > 0 => {
                        let mut aps = ((20 * 3 * 80) / auto_per_five + 99) / 100;
                        aps = std::cmp::max(aps, (aps + 1) / 2);
                        aps += basic_attack;
                        Some(aps)
                    }
                    _ => None,
                };
                let reliability = reliability.unwrap_or(0);
                let repair_ease = repair_ease.unwrap_or(0);
                let range = range as f32 / 10.0;
                let recoil_display = match (recoil_x, recoil_y) {
                    (Some(x), Some(y)) => {
                        Some(((x.powi(2) + y.powi(2)).sqrt() * 10.0).round() / 10.0)
                    }
                    _ => None,
                };
                Ok(NcthWeapon {
                    index,
                    weapon_type,
                    name,
                    long_name,
                    caliber,
                    mag_size,
                    accuracy,
                    damage,
                    range,
                    handling,
                    aiming_levels,
                    loudness,
                    reliability,
                    repair_ease,
                    aps_to_ready,
                    aps_to_attack,
                    aps_to_burst,
                    aps_to_auto,
                    aps_to_reload,
                    aps_to_reload_manually,
                    shots_per_burst,
                    autofire_shots_per_five_ap,
                    recoil_x,
                    recoil_y,
                    recoil_display,
                    buyable,
                    hands,
                    weight,
                    coolness,
                    price,
                })
            }
            _ => Err(()),
        }
    }
}

fn option_u32_to_bool(x: Option<u32>) -> bool {
    match x {
        Some(1) => true,
        _ => false,
    }
}

#[derive(Debug)]
enum Error {
    Read(io::Error),
    Parse(serde_xml_rs::Error),
    Write(csv::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Read(e)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(e: serde_xml_rs::Error) -> Self {
        Error::Parse(e)
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Error::Write(e)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "parse_weapons")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn read_table<T, P1, P2>(directory: P1, file_name: P2) -> Result<T, Error>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    T: Deserialize<'static>,
{
    let file: PathBuf = [directory.as_ref(), file_name.as_ref()].iter().collect();
    let bytes = fs::read(file)?;
    let (xml, _) = encoding_rs::UTF_8.decode_with_bom_removal(&bytes);
    let table = serde_xml_rs::from_str(&xml)?;
    Ok(table)
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let directory = opt.input;
    let output = opt.output;

    let weapon_table: WeaponList = read_table(&directory, "Weapons.xml")?;
    let items_table: ItemList = read_table(&directory, "Items.xml")?;
    let ammo_table: AmmoList = read_table(&directory, "AmmoStrings.xml")?;

    let weapons: HashMap<u32, Weapon> = weapon_table
        .weapons
        .into_iter()
        .map(|w| (w.index, w))
        .collect();
    let items: HashMap<u32, Item> = items_table
        .items
        .into_iter()
        .map(|i| (i.index, i))
        .collect();
    let ammo_types: HashMap<u32, Ammo> = ammo_table
        .ammo_types
        .into_iter()
        .map(|a| (a.index, a))
        .collect();

    let mut ncth_weapons: Vec<NcthWeapon> = weapons
        .into_iter()
        .filter_map(|(index, weapon)| {
            let item = items[&index].clone();
            let ammo = ammo_types[&weapon.caliber?].clone();
            NcthWeapon::try_from((weapon, item, ammo)).ok()
        })
        .collect();
    ncth_weapons.sort_by_key(|w| w.name.clone());
    ncth_weapons.sort_by_key(|w| w.weapon_type);

    let output: Box<Write> = match output {
        Some(output) => Box::new(File::create(output)?),
        None => Box::new(io::stdout()),
    };
    let mut writer = csv::Writer::from_writer(output);
    for weapon in ncth_weapons {
        writer.serialize(weapon)?;
    }
    writer.flush()?;
    Ok(())
}
