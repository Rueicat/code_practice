//標準化時鐘, 只顯示一天
//可以做計算
//相等比較
// ((x % m) + m ) % m  轉換負數

use core::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Clock {
    minutes: i32,
}


impl Clock {

    //標準化邏輯
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = hours * 60 + minutes;
        let day = 24*60;

        //Euclidean remainder 歐幾里得餘數, 恆正
        let norm = ((total % day) + day) % day;     //轉換負數, 保證在0=<r<m
        Self {minutes: norm}    //輸出mins,且保證是正數
    }

    //算時間邏輯
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }

    //split hours and mins time
    fn hour_min(&self) -> (i32, i32) {
        (self.minutes / 60, self.minutes % 60)
    }
}

// trait
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h,m) = self.hour_min();
        write!(f, "{:02}:{:02}", h,m)
    }
}

fn main() {

}
