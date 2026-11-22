use lan_party::{Error, LanParty, LanPartyClient};
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

fn iniciar(cupo: u32) -> (Env, LanPartyClient<'static>) {
    let env = Env::default();
    let id = env.register(LanParty, (cupo,));
    let cliente = LanPartyClient::new(&env, &id);
    (env, cliente)
}

fn con_autorizacion<T>(
    env: &Env,
    cliente: &LanPartyClient<'_>,
    participante: &Address,
    llamar: impl FnOnce(LanPartyClient<'_>) -> T,
) -> T {
    let invocacion = MockAuthInvoke {
        contract: &cliente.address,
        fn_name: "reservar",
        args: (participante,).into_val(env),
        sub_invokes: &[],
    };
    let autorizaciones = [MockAuth {
        address: participante,
        invoke: &invocacion,
    }];
    llamar(cliente.mock_auths(&autorizaciones))
}

#[test]
fn la_reserva_autorizada_descuenta_un_cupo() {
    let (env, cliente) = iniciar(2);
    let participante = Address::generate(&env);

    assert_eq!(cliente.cupos_disponibles(), 2);
    assert_eq!(
        con_autorizacion(&env, &cliente, &participante, |c| c.reservar(&participante)),
        1
    );
    assert_eq!(cliente.cupos_disponibles(), 1);
}

#[test]
fn rechaza_la_reserva_duplicada_sin_consumir_cupo() {
    let (env, cliente) = iniciar(2);
    let ana = Address::generate(&env);
    let leo = Address::generate(&env);

    assert_eq!(
        con_autorizacion(&env, &cliente, &ana, |c| c.reservar(&ana)),
        1
    );
    assert_eq!(
        con_autorizacion(&env, &cliente, &ana, |c| c.try_reservar(&ana)),
        Err(Ok(Error::ReservaDuplicada))
    );
    assert_eq!(cliente.cupos_disponibles(), 1);
    assert_eq!(
        con_autorizacion(&env, &cliente, &leo, |c| c.reservar(&leo)),
        0
    );
}

#[test]
fn rechaza_la_reserva_sin_autorizacion_y_permite_reservar_despues() {
    let (env, cliente) = iniciar(1);
    let participante = Address::generate(&env);
    let otra_persona = Address::generate(&env);

    assert!(cliente.try_reservar(&participante).is_err());
    assert_eq!(cliente.cupos_disponibles(), 1);
    assert!(con_autorizacion(&env, &cliente, &otra_persona, |c| c
        .try_reservar(&participante))
    .is_err());
    assert_eq!(cliente.cupos_disponibles(), 1);
    assert_eq!(
        con_autorizacion(&env, &cliente, &participante, |c| c.reservar(&participante)),
        0
    );
}

#[test]
fn rechaza_nuevas_reservas_cuando_se_agotan_los_cupos() {
    let (env, cliente) = iniciar(1);
    let ana = Address::generate(&env);
    let leo = Address::generate(&env);

    assert_eq!(
        con_autorizacion(&env, &cliente, &ana, |c| c.reservar(&ana)),
        0
    );
    assert_eq!(
        con_autorizacion(&env, &cliente, &leo, |c| c.try_reservar(&leo)),
        Err(Ok(Error::SinCupos))
    );
    assert_eq!(cliente.cupos_disponibles(), 0);
}
