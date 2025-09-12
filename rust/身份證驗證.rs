// 1. 輸入格式檢查
//     - 大寫字母 + 9個數字
//     - 第一個數字表示性別(1 or 2)
// 2. 建立char to int form
// 3. 判斷邏輯
// 4. 輸出


// 開頭字母對照表
fn code(c: char) -> i32 {
    match c {
    'A' => 10,   // 台北市
    'B' => 11,   // 台中市
    'C' => 12,   // 基隆市
    'D' => 13,   // 台南市
    'E' => 14,   // 高雄市
    'F' => 15,   // 新北市
    'G' => 16,   // 宜蘭縣
    'H' => 17,   // 桃園縣
    'I' => 34,   // 嘉義市
    'J' => 18,   // 新竹縣
    'K' => 19,   // 苗栗縣
    'L' => 20,   // 台中縣
    'M' => 21,   // 南投縣
    'N' => 22,   // 彰化縣
    'O' => 35,   // 新竹市
    'P' => 23,   // 雲林縣
    'Q' => 24,   // 嘉義縣
    'R' => 25,   // 台南縣
    'S' => 26,   // 高雄縣
    'T' => 27,   // 屏東縣
    'U' => 28,   // 花蓮縣
    'V' => 29,   // 台東縣
    'W' => 32,   // 金門縣
    'X' => 30,   // 澎湖縣
    'Y' => 31,   // 陽明山
    'Z' => 33,   // 連江縣
    _   => 0,    //match假設要有全部的情況, 如果字母輸入其他特殊符號
    }
}

fn check_id(id: &str) -> bool {
    // 確認長度 = 10
    if id.len() != 10 {
        return false;
    }

    let mut chars = id.chars();    // 取unicode
    
    //取字母轉數字(要大寫)
    let first_letter = chars.next().unwrap();
    if !first_letter.is_ascii_uppercase() {
        return false;
    }
    let code_num = code(first_letter);
    if code_num == 0 {          //理論不會出現, 但如果輸入36個字母以外的字母情況?
        return false;
    }

    // 各個位數的權重
    let weight: [u32; 11] = [1,9,8,7,6,5,4,3,2,1,1];  //長度11, 全部都u32的array

    //依照判斷邏輯拆解字母
    let tens = (code_num / 10) as u32;
    let ones = (code_num % 10) as u32;

    //依照規定的邏輯處理字母權重
    let mut sum: u32 = tens * weight[0] + ones * weight[1];

    //後面9個數字, 性別判斷
    for (i, ch) in chars.enumerate() {
        if let Some(d) = ch.to_digit(10) {
            if i == 0 && d != 1 && d != 2 {
                return false;
            }
            sum += d * weight[i + 2];       //數字*權重表
        } else {
            return false;
        }
    }
    sum % 10 == 0
}




fn main() {
    let id = "T112663836";    //這是測試, 不存在的身份證
    println!("{} is valid: {}", id, check_id(id));
}
