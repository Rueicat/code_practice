class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        pairs = {
            ')': '(',
            ']': '[',
            '}': '{',
        }

        for ch in s:
            if ch in "([{":
                stack.append(ch)
            elif ch in ")]}":
                if not stack or stack.pop() != pairs[ch]:
                    return False
            else:
                return False

        return not stack


#test
print(Solution().isValid("((()))[]"))
