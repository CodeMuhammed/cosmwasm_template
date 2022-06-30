use super::*;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Attribute};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {
        owner: Some("creator".to_string()),
    };
    let info = mock_info("creator", &coins(1000, "stake"));

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, _res.messages.len());
}

#[test]
fn execute_create_new_entry() {
    // setup an instance of the contract
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        owner: Some("creator".to_string()),
    };

    let info = mock_info("creator", &coins(2, "stake"));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // prepare execute data
    let info = mock_info("creator", &coins(2, "stake"));
    let msg = ExecuteMsg::NewEntry {
        description: "Test description".to_string(),
        priority: Some(Priority::High),
    };

    // run proper execute message
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(_res.attributes.len(), 2);
    assert_eq!(
        _res.attributes[0],
        Attribute {
            key: String::from("method"),
            value: "execute_create_new_entry".to_string()
        }
    );
    assert_eq!(
        _res.attributes[1],
        Attribute {
            key: String::from("id"),
            value: String::from("1"),
        }
    );

    // Test for expected output in res
    let _res = query(deps.as_ref(), mock_env(), QueryMsg::QueryEntry { id: 1 }).unwrap();
    let value: EntryResponse = from_binary(&_res).unwrap();
    assert_eq!(1, value.id);
}

#[test]
fn execute_update_entry() {
    // setup an instance of the contract
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        owner: Some("creator".to_string()),
    };

    let info = mock_info("creator", &coins(2, "stake"));
    instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // Here we create a new entry message
    let msg = ExecuteMsg::NewEntry {
        description: "Test description".to_string(),
        priority: Some(Priority::High),
    };

    // run execute message
    execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // create an update entry message
    let msg = ExecuteMsg::UpdateEntry {
        id: 1,
        description: Some("Test description updated".to_string()),
        priority: None,
        status: None,
    };

    // run execute message
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(_res.attributes.len(), 2);
    assert_eq!(
        _res.attributes[0],
        Attribute {
            key: String::from("method"),
            value: String::from("execute_update_entry")
        }
    );
    assert_eq!(
        _res.attributes[1],
        Attribute {
            key: String::from("id"),
            value: String::from("1"),
        }
    );

    // Test for expected output in res
    let _res = query(deps.as_ref(), mock_env(), QueryMsg::QueryEntry { id: 1 }).unwrap();
    let value: EntryResponse = from_binary(&_res).unwrap();
    assert_eq!("Test description updated".to_string(), value.description);
}

#[test]
fn execute_delete_entry() {
    // setup an instance of the contract
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        owner: Some("creator".to_string()),
    };

    let info = mock_info("creator", &coins(2, "stake"));
    instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // Here we create a new entry message
    let msg = ExecuteMsg::NewEntry {
        description: "Test description".to_string(),
        priority: Some(Priority::High),
    };

    // run execute message
    execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // create a delete entry message
    let msg = ExecuteMsg::DeleteEntry { id: 1 };

    // run execute message
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(_res.attributes.len(), 2);
    assert_eq!(
        _res.attributes[0],
        Attribute {
            key: String::from("method"),
            value: String::from("execute_delete_entry")
        }
    );
    assert_eq!(
        _res.attributes[1],
        Attribute {
            key: String::from("id"),
            value: String::from("1"),
        }
    );

    // Test for expected output in res
    let default_res = EntryResponse {
        id: 0,
        status: Status::ToDo,
        priority: Priority::High,
        description: "".to_string(),
    };

    let _res = query(deps.as_ref(), mock_env(), QueryMsg::QueryEntry { id: 1 })
        .unwrap_or(to_binary(&default_res).unwrap());

    let value: EntryResponse = from_binary(&_res).unwrap();
    assert_eq!(0, value.id);
}

#[test]
fn execute_transfer_owner() {
    // setup an instance of the contract
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        owner: Some(String::from("creator")),
    };

    let info = mock_info("creator", &coins(2, "stake"));
    instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // create a transfer owner message
    let msg = ExecuteMsg::TransferContractOwnership {
        new_owner: String::from("new_contract_owner"),
    };

    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(_res.attributes.len(), 2);
    assert_eq!(
        _res.attributes[0],
        Attribute {
            key: String::from("method"),
            value: String::from("execute_transfer_owner")
        }
    );
    assert_eq!(
        _res.attributes[1],
        Attribute {
            key: String::from("new_owner"),
            value: String::from("new_contract_owner"),
        }
    );

    // Here we try to call transfer owner with the old owner which should fail
    let msg = ExecuteMsg::TransferContractOwnership {
        new_owner: String::from("another_owner"),
    };
    let _err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    match _err {
        ContractError::Unauthorized {} => {}
        e => panic!("unexpected error: {}", e),
    }
}

#[test]
fn execute_burn_balance() {
    // setup an instance of the contract
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        owner: Some(String::from("creator")),
    };

    let info = mock_info("creator", &[]);
    instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // create a burn balnce  message
    let msg = ExecuteMsg::BurnContractBalance {};
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(_res.attributes.len(), 1);
    assert_eq!(
        _res.attributes[0],
        Attribute {
            key: String::from("method"),
            value: String::from("try_burn_balance")
        }
    );

    // check the messages in the response to see if the burn method was called
    assert_eq!(_res.messages.len(), 1);
    assert_eq!(
        _res.messages[0].msg,
        CosmosMsg::Bank(BankMsg::Burn { amount: vec![] })
    );
}
