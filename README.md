![Banner de la LAN Party](./Banner.png)

# LAN Party: reservas de cupos en Stellar

Una LAN Party tiene una cantidad limitada de puestos con espacio, enchufe y conexión de red. Si las reservas se coordinan por chat, es fácil anotar dos veces a alguien o prometer más lugares de los que hay. Este contrato en Rust y Soroban descuenta los cupos de un único evento y no permite que la misma dirección reserve dos veces.

## Cómo funciona

| Operación | Resultado |
| --- | --- |
| Desplegar con `cupo_maximo` | Fija el cupo inicial del evento. |
| `reservar(participante)` | Exige autorización de esa dirección, reserva un lugar y devuelve los cupos restantes. |
| `cupos_disponibles()` | Devuelve los cupos restantes sin exigir autorización del participante. |

Por ejemplo, con un cupo inicial de 2, la primera reserva deja 1 lugar. Si esa misma dirección intenta reservar otra vez, recibe `ReservaDuplicada` y todavía queda 1 lugar. Cuando otra dirección ocupa el último puesto, las nuevas reservas reciben `SinCupos`.

El contrato distingue direcciones, no personas. No cobra, no permite cancelar y no tiene interfaz web. Se pensó para un único evento de corta duración; si el estado se archiva por el TTL de Soroban, hay que restaurarlo antes de volver a operar.

## Cómo probarlo

Necesitás [Rust](https://rustup.rs/) y, para compilar el Wasm, [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli) y el destino `wasm32v1-none`.

```sh
cargo test --locked
rustup target add wasm32v1-none
stellar contract build --locked
```

Las pruebas llaman al contrato a través del cliente de Soroban. Verifican que una reserva válida descuenta un cupo y que los intentos duplicados, no autorizados o con el cupo agotado no cambian los cupos restantes.

## Referencias

- [Contador de la clase](https://github.com/KevinLatino/Pura-Vida-Bootcamp/tree/main/Smart%20Contracts/examples/contador): base para el estado, la autorización y las pruebas.
- [Soroban: ejemplo de autorización](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/auth).
