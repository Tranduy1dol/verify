# Generate os-latest.json
    make gen-os
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
### Before doing anything above, please run ```git submodule update --init --recursive```.