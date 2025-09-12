# input strings/output strings
# for i in strs, 取第一個字母, 比較其他字母有無一樣, 有的話存到新的box
# 問題點  怎麼取1st 字母

from typing import List

class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ""


        shortest = min(strs, key=len)
        for i, char in enumerate(shortest):
            for s in strs:
                if s[i] != char:
                    return shortest[:i]

        return shortest


#test
test = ["lisa", "lily", "liar"]
result = Solution().longestCommonPrefix(test)
print(result)
