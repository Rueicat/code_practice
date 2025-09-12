class Solution:
    def romanToInt(self, s: str) -> int:
        # 容器
        value = {
            'I' : 1,
            'V' : 5,
            'X' : 10,
            'L' : 50,
            'C' : 100,
            'D' : 500,
            'M' : 1000
        }

        # init
        total = 0
        max_right = 0

        # loop
        for ch in reversed(s):
            word = value[ch]
            if word < max_right:
                total -= word
            else:
                total += word
                max_right = word
        return total

#test
test_words = 'MCMXCIV'
result = Solution().romanToInt(test_words)
print(f"輸入羅馬文字為{test_words}, 翻譯數字為{result}")
