use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareDebugRevision {
    pub sw_ver: String,
    pub hw: String,
    pub status: u8,
}

#[derive(Debug, Deserialize)]
pub enum CleanMissionPhase {
    #[serde(rename = "charge")]
    Charge,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "evac")]
    Empty,
    #[serde(rename = "stop")]
    Paused,
    #[serde(rename = "stuck")]
    Stuck,
    #[serde(rename = "hmUsrDock")]
    SentHome,
    #[serde(rename = "hmMidMsn")]
    MidDock,
    #[serde(rename = "hmPostMsn")]
    FinalDock,
}

#[derive(Debug, Deserialize)]
pub enum CleanMissionCycle {
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "quick")]
    QuickClean,
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "evac")]
    Empty,
    #[serde(rename = "dock")]
    Dock,
    #[serde(rename = "train")]
    Train,
    #[serde(rename = "none")]
    Ready,
}

#[derive(Debug, Deserialize)]
pub enum CleanMissionInitiator {
    #[serde(rename = "schedule")]
    Plan,
    #[serde(rename = "rmtApp")]
    App,
    #[serde(rename = "manual")]
    Robot,
    #[serde(rename = "localApp")]
    HA,
}

/// Status of current mission
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanMissionStatus {
    // pub cond_not_ready: Vec<()>, // unknown internal type
    pub cycle: CleanMissionCycle,
    pub error: u8,
    pub expire_m: u32,
    pub expire_tm: u32,
    pub initiator: String,
    pub mission_id: String,
    pub mssn_m: u8,
    pub mssn_strt_tm: u32,
    pub n_mssn: u32,
    pub not_ready: u8,
    pub operating_mode: u8,
    pub phase: CleanMissionPhase,
    pub rechrg_m: u32,
    pub rechrg_tm: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoombaMessage {
    Audio { volume: u8 },
    BatAuthEnable(Option<bool>),
    BatInfo {},
    BatPct(u8),
    BatteryType(String),
    Bbchg {},
    Bbchg3 {},
    Bbmssn {},
    Bbnav {},
    Bbpause {},
    Bbrstinfo {},
    Bbrun {},
    Bbswitch {},
    Bbsys {},
    BehaviorFwk(Option<bool>),
    Bin { full: bool, present: bool },
    BinPause(bool),
    BleDevLoc(bool),
    Cap {},
    CarpetBoost(bool),
    ChildLock(bool),
    ChrgLrPtrn(u32),
    CleanMissionStatus(CleanMissionStatus),
    CleanSchedule2(Vec<()>),
    CloudEnv(String),
    Connected(bool),
    Country(String),
    DeploymentState(u8),
    Dock {},
    EcoCharge(bool),
    EvacAllowed(bool),
    FeatureFlags {},
    HwDbgr(Option<HardwareDebugRevision>),
    HwPartsRev {},
    Langs2 {},
    LastCommand {},
    LastDisconnect(u16),
    MapUploadAllowed(bool),
    MissionTelemetry {},
    MssnNavStats {},
    Name(String),
    Netinfo {},
    NoAutoPasses(bool),
    OpenOnly(bool),
    PmapCL(bool),
    PmapLearningAllowed(bool),
    PmapSGen(u8),
    PmapShare {},
    Pmaps(Vec<HashMap<String, String>>),
    RankOverlap(u32),
    ReflexSettings {},
    RuntimeStats { hr: u32, min: u32, sqft: u32 },
    SceneRecog(Option<u16>),
    SchedHold(bool),
    SecureBoot {},
    Signal {},
    Sku(String),
    SoftwareVer(String),
    SubModSwVer {},
    SvcEndpoints {},
    Timezone(String),
    Tls {},
    TwoPass(bool),
    Tz {},
    VacHigh(bool),
    WDevLoc(bool),
    Wifistat {},
    Wlcfg {},
}
