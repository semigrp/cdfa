# dfa-cli

DFA CLI is a command-line interface program for creating and testing Deterministic Finite Automata (DFA). You can define a DFA by providing the alphabet, states, transitions, start state, and accept states. Then, you can test if the DFA accepts a given input string.

## Installation

You can install the program by running the following command:

```bash
git clone https://github.com/semigrp/dfa-cli.git
cd dfa-cli
```

## Usage

1. Run the program:
    
```bash
cargo run
```

2. The program will enter interactive mode, prompting you for commands. Available commands are create, The program will enter interactive mode, prompting you for commands. Available commands are create, test, help, and exit.

### Create 
To create a DFA, enter the create command when prompted. You will be asked for the alphabet, states, transitions, start state, and accept states.

Example:

```bash
Enter a command (create, test, help, exit):
create
Enter the alphabet (e.g. 01):
01
Enter the states (e.g. q0,q1):
q0,q1
Enter the transitions (e.g. q0->0->q0,q0->1->q1,q1->0->q1,q1->1->q0):
q0->0->q0,q0->1->q1,q1->0->q1,q1->1->q0
Enter the start state (e.g. q0):
q0
Enter the accept states (e.g. q1):
q1
```

The program will create the DFA and display its representation.
```bash
DFA created: DFA { alphabet: ['0', '1'], states: ["q0", "q1"], transitions: {("q0", '0'): "q0", ("q0", '1'): "q1", ("q1", '0'): "q1", ("q1", '1'): "q0"}, start: "q0", accept: ["q1"] }
```

### Test 
After creating a DFA, you can test if it accepts a given input string using the test command.

Example:
```bash
Enter a command (create, test, help, exit):
test
Enter the input string:
010
```

The program will test the input string against the DFA and display the result.
```bash
DFA test result: true
```

### Help
To display the available commands and their descriptions, enter the help command when prompted.

### Exit
To exit the program, enter the exit command when prompted.

## License
This project is licensed under the MIT License.
