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


# Bitácora — Compilación exitosa e instalación en VirtualBox
 
## 1 de Abril, 2026
 
---
 
### 9:00 AM — Retoma del proceso de compilación
 
Se retomó la sesión de compilación. La imagen continuaba acumulando tareas del sstate cache. Se identificó que el error de `binutils` del día anterior se había resuelto con el `cleanall`.
 
---
 
### 10:00 AM — Error de wit-bindgen y edition2024
 
Durante la compilación de la receta `colordetector`, se detectó que el crate `wit-bindgen 0.51.0` requería `edition2024`, característica no soportada por Cargo 1.75 de Yocto Scarthgap.
 
**Error encontrado:**
```
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`,
but that feature is not stabilized in this version of Cargo (1.75.0).
```
 
**Solución:**
Se instaló Rust 1.75.0 en el host para generar el vendor con la misma versión de Cargo que usa Yocto:
 
```bash
rustup install 1.75.0
rustup run 1.75.0 cargo --version
# cargo 1.75.0 (1d8b05cdd 2023-11-20)
```
 
Se cambió la versión de `opencv` en `Cargo.toml` a `=0.88.0` para evitar dependencias transitivas incompatibles con Cargo 1.75.
 
---
 
### 11:00 AM — Error de opencv-binding-generator con GCC 14
 
Al usar `opencv 0.88.0`, el `opencv-binding-generator 0.79.0` generó un panic al intentar parsear headers de C++14 de GCC 14 (incluido en Pop!OS 24.04):
 
**Error encontrado:**
```
internal error: entered unreachable code: Can't get kind of parent
for cpp namespace: Entity { kind: LinkageSpec, ...
file: ".../include/c++/14/bits/exception.h" }
```
 
**Solución:**
Se actualizó a `opencv = "=0.93.7"` con `default-features = false` para usar el `opencv-binding-generator 0.93.0` compatible con GCC 14. También se corrigieron manualmente los crates del vendor con `edition2024`:
 
```bash
sed -i 's/edition = "2024"/edition = "2021"/' vendor/spdx/Cargo.toml
sed -i 's/edition = "2024"/edition = "2021"/' vendor/wit-bindgen/Cargo.toml
```
 
---
 
### 12:00 PM — Integración de meta-clang
 
El generador de bindings de OpenCV requería `clang` durante la compilación cruzada. Se clonó e integró la capa `meta-clang`:
 
```bash
cd ~/Documents/poky-scarthgap
git clone -b scarthgap https://github.com/kraj/meta-clang.git
cd build
bitbake-layers add-layer ~/Documents/poky-scarthgap/meta-clang
```
 
Se actualizó la receta para incluir la dependencia y exportar las variables de entorno necesarias:
 
```bash
DEPENDS = "opencv clang-native"
export LIBCLANG_PATH = "${RECIPE_SYSROOT_NATIVE}/usr/lib"
export CLANG_PATH = "${RECIPE_SYSROOT_NATIVE}/usr/bin"
```
 
---
 
### 1:00 PM — Resolución del Cargo.lock versión 4
 
El `Cargo.lock` generado con Cargo 1.94 usaba el formato v4, incompatible con Cargo 1.75 de Yocto:
 
**Error encontrado:**
```
lock file version 4 requires `-Znext-lockfile-bump`
```
 
**Solución:**
Se eliminó el `Cargo.lock` del directorio del proyecto para que Yocto lo generara con su propio Cargo durante el `do_configure`. Se agregó la generación automática del lockfile en la receta:
 
```bash
do_configure:prepend() {
    cp -r ${S}/.cargo ${WORKDIR}/.cargo || true
    cp -r ${S}/vendor ${WORKDIR}/vendor || true
    cd ${S} && ${RECIPE_SYSROOT_NATIVE}/usr/bin/cargo \
        generate-lockfile --manifest-path ${S}/Cargo.toml || true
}
```
 
---
 
### 2:00 PM — Compilación exitosa del colordetector
 
Después de resolver todos los errores de dependencias, la receta `colordetector` compiló exitosamente:
 
```
NOTE: Tasks Summary: Attempted 7278 tasks of which 7265
didn't need to be rerun and all succeeded.
```
 
Se corrigió la ruta del binario en el `do_install`, ya que en cross-compilation el binario queda en `target/x86_64-poky-linux-gnu/release/` y no en `target/release/`:
 
```bash
do_install() {
    install -d ${D}${bindir}
    install -m 0755 \
        ${B}/target/x86_64-poky-linux-gnu/release/proyecto_openCV \
        ${D}${bindir}/colordetector
}
```
 
---
 
### 3:00 PM — Generación del archivo VMDK e importación a VirtualBox
 
Se verificó la imagen generada (~330 MB) y se instaló VirtualBox para crear la máquina virtual:
 
- **Nombre:** Yocto-ColorDetector
- **OS:** Other Linux (64-bit)
- **RAM:** 512 MB
- **Disco:** `core-image-minimal-qemux86-64.rootfs.wic.vmdk` en SATA Port 0
- **Boot Order:** Hard Disk primero
 
**Verificación inicial dentro de la VM:**
```bash
root@qemux86-64:~# colordetector --help
[ERROR] Especificá --video <ruta> o --youtube <url>
  Ejemplo: cargo run --release -- --video mi_video.mp4
```
 
El binario respondió correctamente, confirmando que la instalación fue exitosa.
 
---
 
### 4:00 PM — Implementación del modo headless
 
Al intentar ejecutar el programa con un video en la VM, se detectó que la imagen minimal no tiene display gráfico:
 
```
can't initialize GTK backend in function 'cvInitSystem'
```
 
Se modificó el código `main.rs` para agregar el flag `--headless` que desactiva toda la interfaz gráfica y guarda el reporte en un archivo de texto `/root/reporte_colores.txt`. Las llamadas a `highgui::imshow()` y `highgui::wait_key()` se envuelven en `if !args.headless { ... }`.
 
---
 
### 5:00 PM — Preparación del video de prueba
 
La imagen Yocto minimal no incluye los codecs H.264 necesarios para reproducir archivos `.mp4`. Se convirtió el video a formato raw sin codec y se creó un disco VMDK de 9 GB que se agregó como segundo disco (SATA Port 1) en la VM:
 
```bash
ffmpeg -i ~/Pictures/v1.mp4 -t 120 \
  -c:v rawvideo -pix_fmt bgr24 \
  ~/Pictures/v1_short.avi
 
dd if=/dev/zero of=/tmp/video.img bs=1M count=9000
mkfs.ext4 /tmp/video.img
sudo mount /tmp/video.img /tmp/videomount
sudo cp ~/Pictures/v1_short.avi /tmp/videomount/
sudo umount /tmp/videomount
VBoxManage convertfromraw /tmp/video.img /tmp/video.vmdk --format VMDK
```
 
---
 
### 6:00 PM — Recompilación con plugins de GStreamer y xserver-xorg
 
Se agregaron paquetes adicionales al `local.conf` para soporte de video y display:
 
```bash
IMAGE_INSTALL:append = " opencv colordetector \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-base \
    xserver-xorg"
```
 
Se recompiló la imagen (8219 tareas completadas en su mayoría desde el sstate cache).
 
---
 
### 7:00 PM — Ejecución exitosa en la imagen embebida
 
Se actualizó la VM con la nueva imagen, se montó el disco de video y se ejecutó el detector en modo headless:
 
```bash
root@qemux86-64:~# mkdir -p /mnt/video
root@qemux86-64:~# mount /dev/sdb /mnt/video
root@qemux86-64:~# colordetector --video /mnt/video/v1_short.avi --headless
```
 
**Reporte generado:**
```
════════════════════════════════════════════════════
      REPORTE FINAL — CONTADOR DE COLORES
════════════════════════════════════════════════════
  Frames procesados : 4529
  Tiempo total      : 55.9 segundos
 
  OBJETOS DETECTADOS POR COLOR:
  ────────────────────────────────────────
  Blanco       19631 detecciones  30%
  Negro        17423 detecciones  27%
  Naranja      15185 detecciones  23%
  Rojo          6687 detecciones  10%
  Azul          3762 detecciones   5%
  Verde         1327 detecciones   2%
  Amarillo        48 detecciones   0%
  Violeta         27 detecciones   0%
 
  COLOR MÁS FRECUENTE: BLANCO
════════════════════════════════════════════════════
[INFO] Reporte guardado en: /root/reporte_colores.txt
```
 
**Resultado:** El programa Rust con OpenCV funcionó correctamente dentro de la imagen Yocto embebida en VirtualBox, detectando y contando colores en el video de prueba.
 
---
 
## Resumen del 1 de Abril
 
| Hora | Actividad | Resultado |
|------|-----------|-----------|
| 9:00 AM | Retoma compilación Yocto | En progreso |
| 10:00 AM | Error wit-bindgen edition2024 | ✅ Resuelto con Rust 1.75.0 |
| 11:00 AM | Error opencv-binding-generator GCC 14 | ✅ Resuelto con opencv 0.93.7 |
| 12:00 PM | Integración meta-clang | ✅ Exitosa |
| 1:00 PM | Error Cargo.lock versión 4 | ✅ Resuelto eliminando lockfile |
| 2:00 PM | Compilación exitosa colordetector | ✅ 7278 tareas completadas |
| 3:00 PM | Instalación en VirtualBox | ✅ VM arrancando |
| 4:00 PM | Implementación modo headless | ✅ Código actualizado |
| 5:00 PM | Preparación video de prueba | ✅ Disco VMDK creado |
| 6:00 PM | Recompilación con GStreamer | ✅ 8219 tareas completadas |
| 7:00 PM | Ejecución exitosa en VM | ✅ Reporte generado |
 
---
 
## Configuración Final del Sistema
 
```
Yocto Poky Scarthgap 5.0.16
├── Capas activas
│   ├── meta (core)
│   ├── meta-poky
│   ├── meta-yocto-bsp
│   ├── meta-oe
│   ├── meta-python
│   ├── meta-multimedia
│   ├── meta-colordetector (prioridad 6)
│   └── meta-clang (prioridad 7)
├── Máquina: qemux86-64
├── Imagen: core-image-minimal (~330 MB VMDK)
└── Paquetes adicionales
    ├── opencv 4.9.0
    ├── colordetector (Rust + OpenCV)
    ├── gstreamer1.0-plugins-good
    ├── gstreamer1.0-plugins-base
    └── xserver-xorg
