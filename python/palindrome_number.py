class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        origin_num = x
        rev = 0
        while x > 0:
            rev = rev * 10 + x % 10
            x //= 10
        return origin_num == rev

#test
test = 121
result = Solution()
print(f"{test} {result.isPalindrome(test)}")
