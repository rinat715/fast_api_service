from typing import List, Any
from itertools import permutations as itertools_permutations

ARGS_LENGTH = 5


class CustomErr(Exception):
    pass


class ArgsTooLong(CustomErr):
    def __init__(self, args: List[Any]):
        message = f"Args: {args} too long"
        super().__init__(message)



def permutations(args: List[Any]) -> List[List[Any]]:
    if len(args) > ARGS_LENGTH:
        raise ArgsTooLong(args)
    perm = itertools_permutations(args)
    return list(perm)
