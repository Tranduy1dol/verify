> [!IMPORTANT]
> Before doing anything above, please run ```git submodule update --init --recursive```.

# Generate compiled os.cairo
    make gen-os-v0.13.1
    make gen-os-v0.13.2
> [!NOTE] 
> Need to install `fastecdsa`. Run  ```pip install fastecdsa==2.3.2```.

# Run Madara and Pathfinder
    make run-madara
    make run-pathfinder
# Do transactions on Madara
    make txs
> [!NOTE] 
> Need to install `dojo`, `scarb`.
# Clear database
    make clean

### Current output when running with `cairo-lang v0.13.1`, layout `all-cairo`
```
 SnOsError(Runner(VmException(VmException 
 { 
    pc: Relocatable { 
        segment_index: 0, offset: 9389 
        }, 
    inst_location: Some(Location { 
        end_line: 145, 
        end_col: 58, 
        input_file: InputFile { 
            filename: "/home/ubuntu/code/verify/cairo-lang/src/starkware/starknet/core/os/execution/execute_transactions.cairo" 
            }, 
        parent_location: None, 
        start_line: 145, 
        start_col: 19
     }), 
     inner_exc: Hint((0, UnknownHint("memory[fp + 8] = to_felt_or_relocatable(len(os_input.transactions))"))), 
     error_attr_value: None, 
     traceback: Some("Cairo traceback (most recent call last):
                cairo-lang/src/starkware/starknet/core/os/os.cairo:90:49: (pc=0:10612)
                let (local reserved_range_checks_end) = execute_transactions(block_context=block_context);

```
### Current output when running with `cairo-lang v0.13.2`, layout `starknet-with-keccak`
```
SnOsError(Runner(Runner(NoBuiltinForInstance(({range_check96, mul_mod, add_mod}, starknet_with_keccak)))))
```