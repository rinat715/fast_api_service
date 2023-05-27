from typing import Any, Annotated
from fastapi import FastAPI, Query


from service_permutations_process.permutations import permutations

app = FastAPI()


@app.get("/health")
async def health() -> str:
    return "Hello World! I'm fine."


@app.get("/permutations_blocking")
async def permutations_blocking(items: Annotated[list[Any], Query()]) -> list[list[Any]]:
    print(items)
    return permutations(items)