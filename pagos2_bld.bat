@echo off
cd c:\rbrust\pagos2-rs\src
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\fixvalues.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\rblib.rs     . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\settings.rs  . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\pagos2.rs    . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\main.rs      . /D /C /Y
cd ..\target\debug
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\_config.json . /D /C /Y
cd ..\..
xcopy c:\c-portab\01-rb\pgmfiles\pagos2-rs\Cargo.toml   . /D /C /Y
cargo build
pause
