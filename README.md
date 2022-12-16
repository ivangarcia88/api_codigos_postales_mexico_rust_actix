# Introducción

API desarrollada en rust (actix-web) para consultar colonia, municipio y estado sobre un código postal.

**Esta API emplea información provista por el sitio oficial:**

[https://www.correosdemexico.gob.mx/SSLServicios/ConsultaCP/CodigoPostal_Exportar.aspx](https://www.correosdemexico.gob.mx/SSLServicios/ConsultaCP/CodigoPostal_Exportar.aspx)

La información fue transformada a un archivo JSON mediante un script, el código para este proceso se encuentra en: 

[https://github.com/ivangarcia88/mexican_postal_code_reformat](https://github.com/ivangarcia88/mexican_postal_code_reformat)

# Build and Run

Para compilar la aplicación es necesario tener rust instalado. 
Para instalar rust en ubuntu 22.04 se emplean los siguientes comandos

```plaintext
sudo apt install rustc
sudo apt install cargo
```
Para compilar el código se emplea el siguiente comando

```plaintext
cargo build --release
```
Para correr el servicio ejecute el siguiente comando:

```plaintext
cargo run --release
```

# Running on Ubuntu 22.04

La aplicación no cuenta con un archivo de configuración, en el archivo "main.rs" se encuentra el puerto en el que corre _.bind("0.0.0.0:PORT")_
Si esta corriendo esta aplicación en ubuntu es posible que requiera habilitar el trafico a este puerto

```plaintext
sudo ufw allow 8000
```

