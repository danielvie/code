mudar default CMAKE target

```powershell
$env:CMAKE_GENERATOR = "Visual Studio 15 2017"
```

compilar com "Visual Studio 2022"
```powershell
cmake -G "Visual Studio 17 2022" -B build
```

mudar para compilar com o clang

```
$env:CXX="Path/to/clang++"  
$env:CC="Path/to/clang"   
```

```
cmake -G "MinGW Makefiles" -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -B build
```
