// 统一日志 target，围绕 6 个域进行输出：
// ctrl  - 平台控制面，覆盖启动/调度/治理等生命周期事件。
// data  - 数据平面，覆盖 source/parse/connector/sink 的数据流处理。
// rule  - 规则与业务策略执行，包含插件/接口等扩展点。
// dfx   - 诊断与恢复能力，包含容错、兜底、应急处理。
// mtrc  - 指标与观测数据，记录性能、吞吐、容量等信息。
// kdb   - 知识库或知识图谱相关逻辑，集中管理特征/上下文。
//
// 为保证兼容，旧的 target 宏仍然导出，但已经标记为过期（#[deprecated]），
// 并在内部转发到新的域。迁移新日志时请直接使用新的 target 宏。

// ctrl 域（平台控制面）
#[macro_export]
macro_rules! trace_ctrl { ($($arg:tt)+) => { $crate::re_exports::log::trace!(target: "ctrl", $($arg)+) } }
#[macro_export]
macro_rules! debug_ctrl { ($($arg:tt)+) => { $crate::re_exports::log::debug!(target: "ctrl", $($arg)+) } }
#[macro_export]
macro_rules! info_ctrl  { ($($arg:tt)+) => { $crate::re_exports::log::info!( target: "ctrl", $($arg)+) } }
#[macro_export]
macro_rules! warn_ctrl  { ($($arg:tt)+) => { $crate::re_exports::log::warn!( target: "ctrl", $($arg)+) } }
#[macro_export]
macro_rules! error_ctrl { ($($arg:tt)+) => { $crate::re_exports::log::error!(target: "ctrl", $($arg)+) } }

// data 域（数据平面）
#[macro_export]
macro_rules! trace_data { ($($arg:tt)+) => { $crate::re_exports::log::trace!(target: "data", $($arg)+) } }
#[macro_export]
macro_rules! debug_data { ($($arg:tt)+) => { $crate::re_exports::log::debug!(target: "data", $($arg)+) } }
#[macro_export]
macro_rules! info_data  { ($($arg:tt)+) => { $crate::re_exports::log::info!( target: "data", $($arg)+) } }
#[macro_export]
macro_rules! warn_data  { ($($arg:tt)+) => { $crate::re_exports::log::warn!( target: "data", $($arg)+) } }
#[macro_export]
macro_rules! error_data { ($($arg:tt)+) => { $crate::re_exports::log::error!(target: "data", $($arg)+) } }

// rule 域（策略/插件/接口）
#[macro_export]
macro_rules! trace_rule { ($($arg:tt)+) => { $crate::re_exports::log::trace!(target: "rule", $($arg)+) } }
#[macro_export]
macro_rules! debug_rule { ($($arg:tt)+) => { $crate::re_exports::log::debug!(target: "rule", $($arg)+) } }
#[macro_export]
macro_rules! info_rule  { ($($arg:tt)+) => { $crate::re_exports::log::info!( target: "rule", $($arg)+) } }
#[macro_export]
macro_rules! warn_rule  { ($($arg:tt)+) => { $crate::re_exports::log::warn!( target: "rule", $($arg)+) } }
#[macro_export]
macro_rules! error_rule { ($($arg:tt)+) => { $crate::re_exports::log::error!(target: "rule", $($arg)+) } }

// dfx 域（诊断 & 恢复）
#[macro_export]
macro_rules! trace_dfx { ($($arg:tt)+) => { $crate::re_exports::log::trace!(target: "dfx", $($arg)+) } }
#[macro_export]
macro_rules! debug_dfx { ($($arg:tt)+) => { $crate::re_exports::log::debug!(target: "dfx", $($arg)+) } }
#[macro_export]
macro_rules! info_dfx  { ($($arg:tt)+) => { $crate::re_exports::log::info!( target: "dfx", $($arg)+) } }
#[macro_export]
macro_rules! warn_dfx  { ($($arg:tt)+) => { $crate::re_exports::log::warn!( target: "dfx", $($arg)+) } }
#[macro_export]
macro_rules! error_dfx { ($($arg:tt)+) => { $crate::re_exports::log::error!(target: "dfx", $($arg)+) } }

// mtrc 域（指标 & 观测）
#[macro_export]
macro_rules! trace_mtrc { ($($arg:tt)+) => { $crate::re_exports::log::trace!(target: "mtrc", $($arg)+) } }
#[macro_export]
macro_rules! debug_mtrc { ($($arg:tt)+) => { $crate::re_exports::log::debug!(target: "mtrc", $($arg)+) } }
#[macro_export]
macro_rules! info_mtrc  { ($($arg:tt)+) => { $crate::re_exports::log::info!( target: "mtrc", $($arg)+) } }
#[macro_export]
macro_rules! warn_mtrc  { ($($arg:tt)+) => { $crate::re_exports::log::warn!( target: "mtrc", $($arg)+) } }
#[macro_export]
macro_rules! error_mtrc { ($($arg:tt)+) => { $crate::re_exports::log::error!(target: "mtrc", $($arg)+) } }

#[macro_export]
macro_rules! println_mtrc {
    ($($arg:tt)+) => {
        let val = std::env::var($crate::conf::PRINT_STAT).unwrap_or("false".to_string());
        if val.eq("true") {
            println!("{}", format_args!($($arg)+));
        }
    }
}

// kdb 域（知识库）
#[macro_export]
macro_rules! trace_kdb { ($($arg:tt)+) => { $crate::re_exports::log::trace!(target: "kdb", $($arg)+) } }
#[macro_export]
macro_rules! debug_kdb { ($($arg:tt)+) => { $crate::re_exports::log::debug!(target: "kdb", $($arg)+) } }
#[macro_export]
macro_rules! info_kdb  { ($($arg:tt)+) => { $crate::re_exports::log::info!( target: "kdb", $($arg)+) } }
#[macro_export]
macro_rules! warn_kdb  { ($($arg:tt)+) => { $crate::re_exports::log::warn!( target: "kdb", $($arg)+) } }
#[macro_export]
macro_rules! error_kdb { ($($arg:tt)+) => { $crate::re_exports::log::error!(target: "kdb", $($arg)+) } }
