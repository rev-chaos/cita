# State Test

Test the cita state using [Tests](https://github.com/ethereum/tests/blob/develop/GeneralStateTests/)


## Usage

```sh
$ cd cita

$ RUST_BACKTRACE=1 RUST_LOG=error,state_test=trace,core_executor=trace,evm=trace RUST_MIN_STACK=1342189999 cargo run --bin state_test
```
