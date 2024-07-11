# PebbleVault

> [!CAUTION]
> PebbleVault is still in early development and is not meant to be used in any production environments yet.

![logo-no-background](https://github.com/Stars-Beyond/PebbleVault/assets/34868944/927902b2-1579-4e3a-9c92-93a0f9e47e3e)

---
Welcome to PebbleVault, the database that rocks! 🚀 Imagine a world where pebbles are more than just tiny stones; they are the building blocks of your data dreams. PebbleVault is a memory database with a MySQL twist, all wrapped up in the cozy, memory-safe blanket of Rust. It’s like having a pet rock collection, but for grown-ups with serious data needs!

## Why PebbleVault? 🌟
- Speed: With memory-based storage, your pebbles are accessible at lightning speed.
- Safety: Thanks to Rust, your data is as safe as pebbles in a vault. No more worrying about memory leaks or data corruption.
- Flexibility: Easily throw your pebbles to MySQL when you need more permanent storage. It's like tossing a pebble across a lake, but with fewer ripples and more data integrity.
- Simplicity: Simple operations to add, drop, and throw pebbles make managing your data as easy as skipping stones on a serene pond.
- Amazing dad jokes
## Key Features 🎉
- In-Memory Speed: Keep your pebbles in memory for ultra-fast access.
- MySQL Persistence: Throw pebbles to MySQL for long-term storage, ensuring your data stays solid.
- Rust Reliability: Built with Rust, so your pebbles are safe and sound, protected from the elements (and by elements, we mean bugs).
## Operations 🔧

### Collect (Insert Data)
Store a pebble (data object) in memory.

```rs
vault.collect("my_precious_pebble", data);
```

### Throw (Persist Data)
Send a pebble to MySQL for long-term storage.

```rs
vault.throw("my_precious_pebble");
```

### Drop (Delete Data)
Remove a pebble from memory or disk.

```rs
vault.drop("my_precious_pebble");
```

### Skim (Read Data)
Retrieve a pebble from memory or disk.

```rs
let data = vault.skim("my_precious_pebble");
```

### PebbleStack (Create Table)
Create a new table (or collection) of pebbles.

```rs
vault.pebblestack("my_pebble_stack");
```

### PebbleDump (Bulk Insert)
Add multiple pebbles at once.

```rs
vault.pebbledump("my_pebble_stack", vec![data1, data2, data3]);
```

### PebbleShift (Update Data)
Update an existing pebble's data.

```rs
vault.pebbleshift("my_precious_pebble", new_data);
```

### PebbleSift (Query Data)
Filter and find specific pebbles.

```rs
let results = vault.pebblesift("my_pebble_stack", query_conditions);
```

### PebblePatch (Patch Data)
Partially update a pebble's data.

```rs
vault.pebblepatch("my_precious_pebble", partial_data);
```

### PebbleFlow (Transaction)
Ensure atomic operations.

```rs
vault.pebbleflow(|txn| {
    txn.collect("pebble1", data1);
    txn.collect("pebble2", data2);
    txn.throw("pebble1");
    txn.drop("pebble2");
});
```

### PebbleSquash (Delete Table)
Remove an entire table (or collection) of pebbles.

```rs
vault.pebblesquash("my_pebble_stack");
```

## Installation 🛠️
To get started with PebbleVault, just run:
```sh
cargo install pebblevault
```

## Example Usage

```rs
use pebblevault::Vault;

let vault = Vault::new();

// Create a new table (or collection) of pebbles
vault.pebblestack("my_pebble_stack");

// Insert data into the vault
vault.collect("my_precious_pebble", data);

// Bulk insert multiple pebbles
vault.pebbledump("my_pebble_stack", vec![data1, data2, data3]);

// Query the vault to find specific pebbles
let results = vault.pebblesift("my_pebble_stack", query_conditions);

// Update an existing pebble's data
vault.pebbleshift("my_precious_pebble", new_data);

// Partially update a pebble's data
vault.pebblepatch("my_precious_pebble", partial_data);

// Retrieve data from the vault
let data = vault.skim("my_precious_pebble");

// Persist data to MySQL
vault.throw("my_precious_pebble");

// Remove data from the vault
vault.drop("my_precious_pebble");

// Ensure atomic operations with a transaction
vault.pebbleflow(|txn| {
    txn.collect("pebble1", data1);
    txn.collect("pebble2", data2);
    txn.throw("pebble1");
    txn.drop("pebble2");
});

// Delete an entire table (or collection) of pebbles
vault.pebblesquash("my_pebble_stack");
```

## Contribute 🤝
Do you have ideas to make PebbleVault even better? Want to add more fun to our pebble party? Join us in making PebbleVault the best place for all your pebble-keeping needs! Check out our contributing guide and start throwing your ideas our way.

## License 📜
PebbleVault is licensed under the Apache 2.0 License. Rock on! 🤘
