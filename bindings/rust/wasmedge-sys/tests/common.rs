use wasmedge_sys::{FuncType, Function, ImportObject, ValType, Value};

pub fn create_extern_module(name: impl AsRef<str>) -> ImportObject {
    // create an ImportObj module
    let result = ImportObject::create(name);
    assert!(result.is_ok());
    let mut import_obj = result.unwrap();

    // add host function: "func-add"
    let result = FuncType::create(vec![ValType::ExternRef, ValType::I32], vec![ValType::I32]);
    let func_ty = result.unwrap();
    let result = Function::create(&func_ty, Box::new(extern_add), 0);
    assert!(result.is_ok());
    let host_func = result.unwrap();
    import_obj.add_func("func-add", host_func);

    // add host function: "func-sub"
    let result = FuncType::create(vec![ValType::ExternRef, ValType::I32], vec![ValType::I32]);
    let func_ty = result.unwrap();
    let result = Function::create(&func_ty, Box::new(extern_sub), 0);
    assert!(result.is_ok());
    let host_func = result.unwrap();
    import_obj.add_func("func-sub", host_func);

    // add host function: "func-mul"
    let result = FuncType::create(vec![ValType::ExternRef, ValType::I32], vec![ValType::I32]);
    let func_ty = result.unwrap();
    let result = Function::create(&func_ty, Box::new(extern_mul), 0);
    assert!(result.is_ok());
    let host_func = result.unwrap();
    import_obj.add_func("func-mul", host_func);

    // add host function: "func-div"
    let result = FuncType::create(vec![ValType::ExternRef, ValType::I32], vec![ValType::I32]);
    let func_ty = result.unwrap();
    let result = Function::create(&func_ty, Box::new(extern_div), 0);
    assert!(result.is_ok());
    let host_func = result.unwrap();
    import_obj.add_func("func-div", host_func);

    // add host function: "func-term"
    let result = FuncType::create([], [ValType::I32]);
    assert!(result.is_ok());
    let func_ty = result.unwrap();
    let result = Function::create(&func_ty, Box::new(extern_term), 0);
    let host_func = result.unwrap();
    import_obj.add_func("func-term", host_func);

    // add host function: "func-fail"
    let result = FuncType::create([], [ValType::I32]);
    assert!(result.is_ok());
    let func_ty = result.unwrap();
    let result = Function::create(&func_ty, Box::new(extern_fail), 0);
    let host_func = result.unwrap();
    import_obj.add_func("func-fail", host_func);

    import_obj
}

fn _real_add(inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    if inputs.len() != 2 {
        return Err(1);
    }

    if inputs.len() != 2 {
        return Err(1);
    }

    let a = if inputs[0].ty() == ValType::I32 {
        inputs[0].to_i32()
    } else {
        return Err(2);
    };

    let b = if inputs[1].ty() == ValType::I32 {
        inputs[0].to_i32()
    } else {
        return Err(3);
    };

    let c = a + b;

    Ok(vec![Value::from_i32(c)])
}

fn extern_add(inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    let val1 = if inputs[0].ty() == ValType::ExternRef {
        inputs[0]
    } else {
        return Err(2);
    };
    let val1 = val1
        .extern_ref::<i32>()
        .expect("fail to get i32 from an ExternRef");

    let val2 = if inputs[1].ty() == ValType::I32 {
        inputs[1].to_i32()
    } else {
        return Err(3);
    };

    Ok(vec![Value::from_i32(val1 + val2)])
}

fn extern_sub(inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    let val1 = if inputs[0].ty() == ValType::ExternRef {
        inputs[0]
    } else {
        return Err(2);
    };

    let val1 = val1
        .extern_ref::<i32>()
        .expect("fail to get i32 from an ExternRef");

    let val2 = if inputs[1].ty() == ValType::I32 {
        inputs[1].to_i32()
    } else {
        return Err(3);
    };

    Ok(vec![Value::from_i32(val1 - val2)])
}

fn extern_mul(inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    let val1 = if inputs[0].ty() == ValType::ExternRef {
        inputs[0]
    } else {
        return Err(2);
    };
    let val1 = val1
        .extern_ref::<i32>()
        .expect("fail to get i32 from an ExternRef");

    let val2 = if inputs[1].ty() == ValType::I32 {
        inputs[1].to_i32()
    } else {
        return Err(3);
    };

    Ok(vec![Value::from_i32(val1 * val2)])
}

fn extern_div(inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    let val1 = if inputs[0].ty() == ValType::ExternRef {
        inputs[0]
    } else {
        return Err(2);
    };
    let val1 = val1
        .extern_ref::<i32>()
        .expect("fail to get i32 from an ExternRef");

    let val2 = if inputs[1].ty() == ValType::I32 {
        inputs[1].to_i32()
    } else {
        return Err(3);
    };

    Ok(vec![Value::from_i32(val1 / val2)])
}

fn extern_term(_inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    Ok(vec![Value::from_i32(1234)])
}

fn extern_fail(_inputs: Vec<Value>) -> Result<Vec<Value>, u8> {
    Err(0x02)
}
