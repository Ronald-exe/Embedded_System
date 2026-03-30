# Bitácora del Proyecto — Contador de Objetos por Color

## Tecnologías Utilizadas
- **Lenguaje:** Rust 1.94.1
- **Librería:** OpenCV 4.6.0
- **Sistema Operativo:** Pop!OS 22.04
- **Herramientas:** yt-dlp, Python 3.12

---

## Registro de Actividades

### 29 de Marzo, 2026

---

#### 7:40 PM — Instalación de Rust y Cargo

Se realizó la instalación del lenguaje de programación **Rust** junto con su gestor de paquetes y compilador **Cargo**.

**Comandos ejecutados:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verificación:**
```bash
rustc --version   # rustc 1.94.1 (e408947bf 2026-03-25)
cargo --version   # cargo 1.94.1 (29ea6fb6a 2026-03-24)
```

**Resultado:** Rust y Cargo instalados correctamente.

---

#### 8:00 PM — Depuración del Código con OpenCV y Dependencias

Se configuró el proyecto Rust con el crate de **OpenCV** y se resolvieron los errores de compilación encontrados durante el proceso.

**Dependencias del sistema instaladas:**
```bash
sudo apt install libopencv-dev clang libclang-dev \
    build-essential libstdc++-14-dev g++ python3-pip -y
```

**Dependencias de Python instaladas:**
```bash
pip3 install yt-dlp --break-system-packages
pip3 install "numpy<2.0" --break-system-packages --force-reinstall
pip3 install lapx --break-system-packages
```

**Configuración del Cargo.toml:**
```toml
[package]
name = "proyecto_openCV"
version = "0.1.0"
edition = "2021"

[dependencies]
opencv = { version = "0.93", features = ["videoio", "imgproc", "highgui"] }
```

**Errores encontrados y resueltos:**

| Error | Causa | Solución |
|---|---|---|
| `use of unresolved module opencv` | Faltaba la dependencia en Cargo.toml | Agregar crate opencv al Cargo.toml |
| `fatal error: 'memory' file not found` | Faltaban headers de C++ | `sudo apt install libstdc++-14-dev g++` |
| `numpy.dtype size changed` | Conflicto de versiones numpy | Bajar numpy a versión `<2.0` |
| `No module named 'lap'` | Faltaba librería de tracking | `pip3 install lapx` |
| `cannot borrow as mutable` | Borrow checker de Rust | Usar variable temporal con `Mat::default()` |

**Resultado:** Código compilado correctamente.

---

#### 9:00 PM — Primera Ejecución del Sistema

Se ejecutó el sistema por primera vez con un video local y posteriormente con un stream de YouTube.

**Comando utilizado:**
```bash
cargo run --release -- --video "/ruta/al/video.mp4"
```

**Observaciones:**
- El sistema detectó y clasificó correctamente los objetos por color en tiempo real.
- Se mostró el dashboard con estadísticas en la esquina superior izquierda.
- El video presentaba velocidad elevada, se corrigió calculando el delay según el FPS real del video.
- En Linux se presentó el problema de múltiples ventanas, resuelto inicializando una única ventana con `cv2.namedWindow()` antes del loop principal.

**Resultado:** Sistema funcionando correctamente con video local y YouTube.

---

## Notas Generales

- La primera compilación de Rust con OpenCV tardó aproximadamente **10 minutos** debido a la generación de bindings.
- Las compilaciones posteriores son casi inmediatas gracias al caché de Cargo.
- Se recomienda usar `--skip 3` para mejor rendimiento en equipos con recursos limitados.

---

## Estructura del Proyecto

```
proyecto_openCV/
    ├── Cargo.toml        ← Configuración y dependencias
    ├── Cargo.lock        ← Versiones exactas (generado automáticamente)
    └── src/
        └── main.rs       ← Código fuente principal
```
