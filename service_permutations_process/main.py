from typing import Any, Annotated
from fastapi import FastAPI, Depends, Request
from fastapi.responses import JSONResponse

import json


from service_permutations_process.permutations import permutations as service_permutations

app = FastAPI()

class DecodeErrorException(Exception):
    def __init__(self, name: str):
        self.name = name



@app.exception_handler(DecodeErrorException)
async def unicorn_exception_handler(request: Request, exc: DecodeErrorException):
    return JSONResponse(
        status_code=418,
        content={"message": f"Error! '{exc.name}' not valid json."},
    )


def parse_args(items: str) -> list[str, Any]:
    try:
        return json.loads(items)
    except json.JSONDecodeError:
        raise DecodeErrorException(name=items)


def permutations(items: Annotated[list[str, Any], Depends(parse_args)]):
   return service_permutations(items)


@app.get("/health")
async def health() -> str:
    return "Hello World! I'm fine."


@app.get("/permutations_blocking")
async def permutations_blocking(items: Annotated[list[str, Any], Depends(permutations)]) -> list[list[Any]]:
    return items