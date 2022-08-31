use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PYETGR {
    /// 交路信息
    pub circuits: Vec<CircuitsNode>,
    /// 配置信息
    pub config: String,
    /// 基线数据
    pub line: Line,
    /// qETRC支持的更多基线数据
    pub lines: Vec<Line>,
    /// 列车信息
    pub trains: Vec<TrainScheduleBlock>,
    ///
    pub markdown: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainScheduleNode {
    /// 站名
    pub zhanming: String,
    /// 到达时间
    pub ddsj: String,
    /// 出发时间
    pub cfsj: String,
    /// 备注
    pub note: String,
    /// 营业
    pub business: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrainScheduleBlock {
    /// 车次[全车次,下行,上行]
    pub checi: Vec<String>,
    #[serde(rename = "UI")]
    pub ui: serde_json::Value,
    /// 列车类型
    #[serde(rename = "type")]
    pub k_type: String,
    /// 时刻表
    pub timetable: Vec<TrainScheduleNode>,
    /// 首发站
    pub sfz: String,
    /// 终点站
    pub zdz: String,
    /// 是否显示
    pub shown: bool,
    // /// 本线路图首发站
    // local_first: Option<String>,
    // /// 本线路图终点站
    // local_last: Option<String>,
    /// 运行线自动设置
    pub auto_item: bool,
    // item_info: serde_json::Value,
    /// 是否客运
    pub passenger: bool,
    /// 交路信息
    pub carriage_circuit: Option<String>,
}

/// 交路信息单元
#[derive(Serialize, Deserialize, Debug)]
pub struct CircuitsNode {}

/// 基线信息单元
#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    /// 维修天窗
    pub forbid: LineForbid,
    /// 施工天窗
    pub forbid2: LineForbid,
    /// 线路名称
    pub name: String,
    /// 备注
    pub notes: LineNote,
    pub ordinate: String,
    /// 标尺
    pub rulers: Vec<Ruler>,
    /// 站点
    pub stations: Vec<Station>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineNote {
    pub author: String,
    pub note: String,
    pub version: String,
}
/// 天窗
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineForbid {
    pub different: bool,
    /// 显示上行天窗
    pub up_show: bool,
    /// 显示下行天窗
    pub down_show: bool,
    pub nodes: Vec<ForbidNode>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForbidNode {
    /// 开始时间
    begin: String,
    /// 结束时间
    end: String,
    /// 出发站名
    pub fazhan: String,
    /// 到达站名
    pub daozhan: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ruler {
    pub different: bool,
    /// 标尺名称
    pub name: String,
    pub nodes: Vec<RulerNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RulerNode {
    /// 出发站名
    pub fazhan: String,
    /// 到达站名
    pub daozhan: String,
    /// 间隔(秒)
    pub interval: u32,
    /// 起
    pub start: u32,
    /// 停
    pub stop: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    /// 对里程
    pub counter: Option<f32>,
    /// 等级
    pub dengji: u32,
    /// 单向站 1-下 2-上 3-上下 4-不通过
    pub direction: u8,
    /// 货运
    pub freight: bool,
    /// 里程
    pub licheng: u32,
    /// 客运
    pub passenger: bool,
    ///
    pub show: bool,
    ///
    // tracks: [],
    ///
    pub zhanming: String,
}
