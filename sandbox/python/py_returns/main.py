from dataclasses import dataclass

from returns.maybe import Maybe, Nothing, Some
from returns.result import Failure, Result, Success

@dataclass
class User:
    id: int
    name: str

USERS = {1: User(1, "Carol"), 2: User(2, "Daniel")}

def find_user(user_id) -> Maybe[User]:
    return Some(USERS[user_id].name) if user_id in USERS else Nothing

def div(a: int, b: int) -> Result[int, str]:
    return Success(a//b) if b != 0 else Failure("'b' value is zero, cannot divide!!!")

def add10(v: int) -> Result[int, str]:
    return Success(v + 10)

def main():
    # maybe
    res = find_user(1)
    print(res)

    res_mis = find_user(3)
    print(res_mis)

    res_div = div(7,1).bind(add10)
    res_div_err = div(7,0).bind(add10)

    print(res_div)
    print(res_div_err)

if __name__ == "__main__":
    main()
