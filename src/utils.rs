#[inline(always)]
pub fn trans_date(date: &str) -> String {
    format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..])
}
/// 将时间格式转换 1305->13:05
#[inline(always)]
pub fn trans_time(s: &str) -> String {
    format!("{}:{}", &s[..2], &s[2..])
}
