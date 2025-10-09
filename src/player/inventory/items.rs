use std::fmt::Display;

pub enum Item {
    UnknownItem,
    MeleeWeapon(MeleeWeaponItem),
    RangedWeapon(RangedWeaponItem),
    //Clothing(ClothingItem),
    //Resource(ResourceItem),
}

impl Display for Item { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Item: {}", &self) } }
impl Default for Item { fn default() -> Self { Item::UnknownItem } }
impl Default for &Item { fn default() -> Self { &Item::UnknownItem } }

pub struct MeleeWeaponItem {
    pub kind: MeleeWeaponKind,
    pub damage: f32,
    pub use_time: f32, //Seconds it takes to complete one full "use" of the time
    pub melee_range: f32,
}

pub struct RangedWeaponItem {
    pub kind: RangedWeaponKind,
    pub damage: f32,
    pub fire_rate: f32,
    pub muzzle_velocity: f32,
    //pub bullet_kind:  
}

pub enum DamageKind {
    Impact, //Subdermal bleeding, possible osseous damage or fracture
    Piercing, //Localized but extreme damage
    Abrasion,
    Incision,
    Laceration,
}

pub enum MeleeWeaponKind {
    //Melee generics
    MeleeImpact,
    /*
    * Sticks, bats, hammers, cinder blocks (any generic throwable for the matter),
    * pipes, crowbars, rebar, bricks, weapon stocks/handles, etc.
    */

    MeleePiercing,
    /*
    * Knife tips, spears, thrusting motions with pointed ends, skewers,
    * nails, morning stars, etc.
    */

    MeleeSlashing,
}

pub enum RangedWeaponKind {
    //Firearm type generics
    Handgun,
    SubMachineGun,

    BoltActionRifle,
    SemiAutoRifle,
    BurstFireRifle,
    FullAutoRifle,
    
    LowRangeSniperRifle,
    MidRangeSniperRifle,
    HighRangeSniperRifle,
    AntiMaterielSniperRifle,
}

pub enum ItemTrait {
    Equipable,
    Consumable,
    Craftable
}
