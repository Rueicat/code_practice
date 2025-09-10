from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i in range(len(nums)):
            compared = target - nums[i]
            for j in range(i + 1, len(nums)):
                if nums[j] == compared:
                    return [i, j]
        return []


#test

result = Solution()
print(result.twoSum([2,4,5,6,5,2], 11))
                
