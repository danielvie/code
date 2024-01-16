mudar default CMAKE target

```powershell
$env:CMAKE_GENERATOR = "Visual Studio 15 2017"
```

mudar para compilar com o clang
```
cmake -G "MinGW Makefiles" -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -B build
```
