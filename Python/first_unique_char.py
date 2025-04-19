# # Given a string s, return the index of the first non-repeating character in it. If it doesn't exist, return -1.

# Constraints:

# Do not use recursion.
# Do not use dynamic programming.
# Aim for O(n) time.
# Avoid extra passes if possible.
# Lowercase letters only.


def first_unique_char(string: str) -> int:
    char_count = {}
    
    for char in string:
        if char in char_count:
            char_count[char] += 1
        else:
            char_count[char] = 1
    
    for index, char in enumerate(string):
        if char_count[char] == 1:
            print(f"{char_count}")
            return index
    
    print(f"{char_count}")
    return -1


def first_unique_char_compression(s: str) -> int:
    char_count = {}
    for i, c in enumerate(s):
        if c in char_count:
            char_count[c] += 0.1
        else:
            char_count[c] = i + 0.1

    result = float('inf')
    for val in char_count.values():
        count = round((val % 1) * 10)
        if count == 1:
            idx = int(val)
            result = min(result, idx)

    print(f"{char_count}")
    return result if result != float('inf') else -1


def first_unique_char_key_value(s: str) -> int:
    char_count = {}

    for i, c in enumerate(s):
        if c in char_count:
            char_count[c] =  -1
        else:
            char_count[c] = i

    for i, c in enumerate(s):
        if char_count[c] != -1:
            return char_count[c]

    return -1




if __name__ == "__main__":
    print(f"Output: {first_unique_char_key_value("leetcode")}")  # Output: 0
    print(f"Output: {first_unique_char_key_value("loveleetcode")}")  # Output: 2
    print(f"Output: {first_unique_char_key_value("aabb")}")  # Output: -1
    print(f"Output: {first_unique_char_key_value("")}")  # Output: -1