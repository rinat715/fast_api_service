from asyncio import get_event_loop
from concurrent.futures import ProcessPoolExecutor
from contextlib import asynccontextmanager
from typing import Any, Annotated
from fastapi import FastAPI, Depends, Request, Query
from fastapi.responses import JSONResponse
from pydantic import BaseModel, validator, Json, ValidationError, parse_obj_as


from service_permutations_process.permutations import permutations as service_permutations



@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.executor = ProcessPoolExecutor()
    yield
    app.state.executor.shutdown()


app = FastAPI(lifespan=lifespan)


class ListRequeredErrorException(Exception):
    def __init__(self, name: str):
        self.name = name



@app.exception_handler(ListRequeredErrorException)
async def decode_exception_handler(_: Request, exc: ListRequeredErrorException):
    return JSONResponse(
        status_code=418, # чайник
        content={"message": f"Error! '{exc.name}' not list json"},
    )


class Request(BaseModel):
    items: list[Any]
    
    @validator('items', pre=True)
    def items_only_list(cls, v):
        if not isinstance(v, list):
            raise ValueError('items only list')
        return v


def get_parsed_object(value: Json = Query(alias='items')):
    try:
        return parse_obj_as(Request, {'items': value})
    except ValidationError:
        raise ListRequeredErrorException(name=value)


def permutations(item: Annotated[Request, Depends(get_parsed_object)]) -> list[list[Any]]:
   return service_permutations(item.items)


@app.get("/health")
async def health() -> str:
    return "Hello World! I'm fine."


@app.get("/permutations_blocking")
async def permutations_blocking(items: Annotated[list[list[Any]], Depends(permutations)]):
    return items


async def run_in_process(item: Annotated[Request, Depends(get_parsed_object)]) -> list[list[Any]]:
    return await get_event_loop().run_in_executor(app.state.executor, service_permutations, item.items)  


@app.get("/permutations_non_blocking")
async def permutations_non_blocking(items: Annotated[list[list[Any]], Depends(run_in_process)]):
    return items
