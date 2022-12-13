<h1 align="center">Teoria de lenguaje - Rust and Roses</h1>

Integrantes:

Juan Cruz Caserío: jcaserio@fi.uba.ar

Franco Gazzola: fgazzola@fi.uba.ar

Axel Kelman: akelman@fi.uba.ar


Para ejecutar el servidor, posicionarse en tdl/server y ejecutar el siguiente comando  

```
RUST_LOG=INFO cargo run
```

Si no desea la informacion del server, simplemente ejecutar

```
cargo run
```

Para ejecutar el cliente, posicionarse en tdl/cliente y ejecutar el siguiente comando

```
cargo run
```

El cliente puede mandar los siguientes comandos al servidor

```
iniciar-sesion [nombre-usuario] [contraseña]
registrarse [nombre-usuario] [contraseña mail]
consultar-nominados
consultar-resultados
consultar-saldo
votar [nominado] [cantidad]
cargar-saldo [nombre-usuario] [monto]
salir
```
