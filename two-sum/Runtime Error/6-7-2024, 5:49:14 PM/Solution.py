// https://leetcode.com/problems/two-sum

class Solution(object):
    def twoSum(self, nums, target):
        for i in range(len(nums)):
            for j in range(i+1, len(nums)):
                if nums[i]+nums[j] == target:
                    return [i, j]

[nums[i],nums[i+1] for i in range()]