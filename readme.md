run
```sh
poetry run uvicorn service_permutations_process.main:app 
```

сборка
```sh
maturin develop
```
 

 тесты:
 ```sh
 export LD_LIBRARY_PATH=/home/user/.pyenv/versions/3.10.9/lib/
 cargo test --no-default-features 
 ```

 обычный запуск
 ```
'localhost:8000/permutations_from_rust?items=[1,2]'
```

запуск в отдельном процессе через ProcessPoolExecutor
```
'localhost:8000/permutations_blocking?items=[1,2,3]'
```

запуск библиотеки на rust
```
'localhost:8000/permutations_non_blocking?items=[1,2]'
```