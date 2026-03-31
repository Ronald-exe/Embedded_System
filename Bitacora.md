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

# Bitácora — Integración con Yocto Project

## 30 de Marzo, 2026

**9:00 PM — Configuración del entorno Yocto Kirkstone**

Se inició el entorno de Yocto con la versión Kirkstone (4.0) previamente descargada y se verificó que el comando `bitbake` estuviera disponible.

**10:00 PM — Creación de la capa personalizada**

Se creó la capa `meta-colordetector` y se configuró la receta `colordetector_0.1.bb` para incluir el binario del proyecto. Se copiaron los archivos `main.rs` y `Cargo.toml` a la carpeta `files/` de la receta.

**11:00 PM — Integración de meta-openembedded**

Se clonó el repositorio `meta-openembedded` rama Kirkstone y se agregaron las capas `meta-oe`, `meta-python` y `meta-multimedia` para tener acceso a OpenCV dentro de la imagen.

**12:00 AM — Primera compilación y errores**

Se ejecutó `bitbake core-image-minimal` por primera vez. Se presentaron errores relacionados con la estructura del `Cargo.toml`, el `Cargo.lock` incompatible con la versión de Cargo de Yocto (1.59) y problemas con la búsqueda del crate `opencv` en crates.io sin acceso a internet.

**1:00 AM — Diagnóstico de incompatibilidad de versiones**

Se identificó que Yocto Kirkstone usa Cargo 1.59 mientras que el proyecto fue desarrollado con Cargo 1.94. El crate `opencv 0.93` no existía en la época de Cargo 1.59, por lo que la compilación era imposible en Kirkstone.

**2:00 AM — Migración a Yocto Scarthgap (5.0)**

Se descargó Poky Scarthgap y su correspondiente `meta-openembedded`. Se migró la capa `meta-colordetector` actualizando la compatibilidad a `scarthgap` y se configuró el `local.conf` con la arquitectura `qemux86-64`.

**3:00 AM — Nuevo error en Scarthgap y pausa**

Se inició la compilación con Scarthgap pero se presentó un error en `binutils 2.42` por incompatibilidad con el compilador del sistema en Pop!OS 24.04. Se dejó pendiente la resolución para el día siguiente.

---

## 31 de Marzo, 2026

**Continuación — Resolución del error de binutils**

Al retomar la sesión, se ejecutó `bitbake -c cleanall binutils` para limpiar la compilación anterior y se reinició el proceso de compilación de la imagen completa. La compilación continúa en proceso.
