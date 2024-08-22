// src/lib.rs
use concordium_std::*;

#[derive(Serialize, SchemaType)]
struct Transaction {
    sender: AccountAddress,
    receiver: AccountAddress,
    amount: Amount,
}

#[derive(Serialize, Default)]
struct State {
    transactions: Vec<Transaction>,
}

#[init(contract = "concordium-dapp-sample")]
fn contract_init<S: HasStateApi>(ctx: &impl HasInitContext, _state: &mut State, _amount: Amount, _logger: &mut impl HasLogger) -> InitResult<()> {
    Ok(())
}

#[receive(contract = "concordium-dapp-sample", name = "register_transaction", parameter = "Transaction")]
fn register_transaction<S: HasStateApi>(ctx: &impl HasReceiveContext, host: &mut impl HasHost<State, StateApiType = S>) -> ReceiveResult<()> {
    let sender = ctx.sender();
    let amount = ctx.amount();
    let receiver: AccountAddress = ctx.parameter_cursor().get()?;

    let new_transaction = Transaction {
        sender,
        receiver,
        amount,
    };

    host.state_mut().transactions.push(new_transaction);

    Ok(())
}

#[receive(contract = "concordium-dapp-sample", name = "get_transactions", return_value = "Vec<Transaction>")]
fn get_transactions<S: HasStateApi>(ctx: &impl HasReceiveContext, host: &mut impl HasHost<State, StateApiType = S>) -> ReceiveResult<Vec<Transaction>> {
    Ok(host.state().transactions.clone())
}
